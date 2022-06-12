use super::{account::Account, transaction::TransactionType, Transaction};
use rust_decimal::prelude::*;
use std::collections::HashMap;
use std::error::Error;

pub struct Bank {
    accounts: HashMap<u16, Account>,
    transactions: HashMap<u32, Transaction>,
}

type Result = std::result::Result<(), Box<dyn Error>>;

impl Bank {
    pub fn new() -> Self {
        Bank {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    //Only used for testing
    #[cfg(test)]
    pub fn get_account(&self, client: &u16) -> Option<&Account> {
        self.accounts.get(client)
    }

    pub fn get_accounts(&self) -> &HashMap<u16, Account> {
        &self.accounts
    }

    pub fn process_transaction(&mut self, transaction: Transaction) -> Result {
        //Lazily create accounts if an unrecognized client id is detected
        if !self.accounts.contains_key(&transaction.client) {
            self.accounts
                .insert(transaction.client, Account::new(transaction.client));
        }

        //Safe to unwrap as we have already checked accounts contains the client.
        let account = self.accounts.get_mut(&transaction.client).unwrap();

        //Check if the account is locked since there is no need for any more operations if it is.
        if account.locked {
            Err(format!(
                "Account is locked, you can thus not perform any transactions against this account: {:?}",
                account
            ))?
        }

        //Evaluate matrix of valid transactions
        //In the real world I would recommend to handle errors individually
        //by filling out each individual error case in the match statement instead of grouping them together.
        //I have done this just to reduce the amount of code and make it easier to read.
        //I would also recommend replacing the errors with a custom error type to make debugging easier and remove the need to pass strings in our errors.
        let result = match (
            self.transactions.get(&transaction.tx),
            transaction.ty,
            transaction.amount,
        ) {
            (None, TransactionType::Deposit, Some(amount)) => Self::deposit(account, amount),
            (None, TransactionType::Withdrawal, Some(amount)) => Self::withdrawal(account, amount),
            (Some(referenced_transaction), _, None) => {
                Self::process_referenced_transaction(account, &transaction, referenced_transaction)
            }
            (referenced_transaction, _, _) => Err(format!(
                "Invalid transaction:\n{:?}\nReferenced transaction:\n{:?}",
                transaction, referenced_transaction
            ))?,
        };

        match (result.is_ok(), transaction.ty) {
            (true, TransactionType::Deposit | TransactionType::Withdrawal) => {
                self.transactions.insert(transaction.tx, transaction);
            }
            _ => {}
        }

        result
    }

    fn process_referenced_transaction(
        account: &mut Account,
        transaction: &Transaction,
        referenced_transaction: &Transaction,
    ) -> Result {
        if referenced_transaction.client != transaction.client {
            Err(format!(
                "Transaction(Client:{:?}) and referenced transaction(Client:{:?}) client doesn't match.",
                transaction.client, referenced_transaction.client
            ))?;
        }

        match (referenced_transaction.amount, transaction.ty) {
            (Some(referenced_amount), TransactionType::Dispute) => {
                Self::dispute(account, referenced_transaction.ty, referenced_amount)
            }
            (Some(referenced_amount), TransactionType::Resolve) => {
                Self::resolve(account, referenced_amount)
            }
            (Some(referenced_amount), TransactionType::Chargeback) => {
                Self::chargeback(account, referenced_transaction.ty, referenced_amount)
            }
            _ => Err(format!(
                "Invalid transaction {:?} referenced by transaction:\n{:?}",
                referenced_transaction, transaction,
            ))?,
        }
    }

    fn deposit(account: &mut Account, amount: Decimal) -> Result {
        account.available += amount;
        account.total += amount;

        Ok(())
    }

    fn withdrawal(account: &mut Account, amount: Decimal) -> Result {
        if amount > account.available {
            Err("Not enough funds available")?
        }
        account.available -= amount;
        account.total -= amount;

        Ok(())
    }

    fn dispute(
        account: &mut Account,
        referenced_ty: TransactionType,
        referenced_amount: Decimal,
    ) -> Result {
        match referenced_ty {
            TransactionType::Deposit => {
                account.held += referenced_amount;
                account.available -= referenced_amount;
            }
            TransactionType::Withdrawal => {
                //Unsure if dispute for withdrawal should be supported
                account.held += referenced_amount;
                account.total += referenced_amount;
            }
            ty => Err(format!("Unsupported dispute type {:?}", ty))?,
        }

        Ok(())
    }

    fn resolve(account: &mut Account, amount: Decimal) -> Result {
        account.held -= amount;
        account.available += amount;

        Ok(())
    }

    fn chargeback(
        account: &mut Account,
        referenced_ty: TransactionType,
        referenced_amount: Decimal,
    ) -> Result {
        match referenced_ty {
            TransactionType::Deposit => {
                account.held -= referenced_amount;
                account.total -= referenced_amount;
                account.locked = true;
            }
            TransactionType::Withdrawal => {
                //Unsure if chargeback for withdrawal should be supported
                account.held -= referenced_amount;
                account.available += referenced_amount;
                account.locked = true;
            }
            ty => Err(format!("Unsupported chargeback type {:?}", ty))?,
        }

        Ok(())
    }
}
