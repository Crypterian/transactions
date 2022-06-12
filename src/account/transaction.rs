use rust_decimal::prelude::*;
use serde::Deserialize;
use serde::Deserializer;

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub(crate) ty: TransactionType,
    pub(crate) client: u16,
    pub(crate) tx: u32,
    #[serde(deserialize_with = "deserialize_decimal_from_str")]
    pub(crate) amount: Option<Decimal>,
}

pub fn deserialize_decimal_from_str<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    //If there is no valid decimal number then set it to None
    match Decimal::from_str(&s) {
        Ok(decimal) => Ok(Some(
            //Round to 4 decimals
            decimal.round_dp_with_strategy(4, RoundingStrategy::MidpointAwayFromZero),
        )),
        Err(_) => Ok(None),
    }
}

//Only used for testing
#[cfg(test)]
impl Transaction {
    pub fn new_deposit(client: u16, tx: u32, amount: Decimal) -> Self {
        Transaction {
            ty: TransactionType::Deposit,
            client,
            tx,
            amount: Some(amount),
        }
    }

    pub fn new_withdrawal(client: u16, tx: u32, amount: Decimal) -> Self {
        Transaction {
            ty: TransactionType::Withdrawal,
            client,
            tx,
            amount: Some(amount),
        }
    }

    pub fn new_dispute(client: u16, tx: u32) -> Self {
        Transaction {
            ty: TransactionType::Dispute,
            client,
            tx,
            amount: None,
        }
    }

    pub fn new_resolve(client: u16, tx: u32) -> Self {
        Transaction {
            ty: TransactionType::Resolve,
            client,
            tx,
            amount: None,
        }
    }

    pub fn new_chargeback(client: u16, tx: u32) -> Self {
        Transaction {
            ty: TransactionType::Chargeback,
            client,
            tx,
            amount: None,
        }
    }
}
