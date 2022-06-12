use super::{account::Account, Bank, Transaction};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

fn validate_account(account: &Account, available: Decimal, total: Decimal, held: Decimal) {
    assert_eq!(account.available, available);
    assert_eq!(account.total, total);
    assert_eq!(account.held, held);
}

//Used to shorten the tests and avoid unnecessary repetition
const CLIENT_ID: u16 = 1;
const DEPOSIT_TX: u32 = 1;
const AMOUNT: Decimal = dec!(1.0);
const WITHDRAWAL_TX: u32 = 2;
const W_AMOUNT: Decimal = dec!(2.0);

#[test]
fn transaction_id_collision_test() {
    let mut bank = Bank::new();

    let deposit_transaction = Transaction::new_deposit(CLIENT_ID, DEPOSIT_TX, AMOUNT);
    bank.process_transaction(deposit_transaction.clone())
        .unwrap();
    assert_eq!(bank.process_transaction(deposit_transaction).is_err(), true);

    let account = bank.get_account(&CLIENT_ID).unwrap();
    validate_account(&account, AMOUNT, AMOUNT, dec!(0.0));
}

#[test]
fn deposit_test() {
    let mut bank = Bank::new();

    let deposit_transaction = Transaction::new_deposit(CLIENT_ID, DEPOSIT_TX, AMOUNT);
    bank.process_transaction(deposit_transaction).unwrap();

    let account = bank.get_account(&CLIENT_ID).unwrap();
    validate_account(&account, AMOUNT, AMOUNT, dec!(0.0));
}

#[test]
fn withdrawal_test() {
    let mut bank = Bank::new();

    let deposit_transaction = Transaction::new_deposit(CLIENT_ID, DEPOSIT_TX, AMOUNT);
    let withdrawal_transaction = Transaction::new_withdrawal(CLIENT_ID, WITHDRAWAL_TX, AMOUNT);

    bank.process_transaction(deposit_transaction).unwrap();
    bank.process_transaction(withdrawal_transaction).unwrap();

    let account = bank.get_account(&CLIENT_ID).unwrap();
    validate_account(&account, dec!(0.0), dec!(0.0), dec!(0.0));
}

#[test]
fn withdraw_to_much_test() {
    let mut bank = Bank::new();

    let deposit_transaction = Transaction::new_deposit(CLIENT_ID, WITHDRAWAL_TX, AMOUNT);
    let withdrawal_transaction = Transaction::new_withdrawal(CLIENT_ID, WITHDRAWAL_TX, W_AMOUNT);

    bank.process_transaction(deposit_transaction).unwrap();
    assert_eq!(
        bank.process_transaction(withdrawal_transaction).is_err(),
        true
    );

    let account = bank.get_account(&CLIENT_ID).unwrap();
    validate_account(&account, AMOUNT, AMOUNT, dec!(0.0));
}

#[test]
fn dispute_deposit_test() {
    let mut bank = Bank::new();

    let deposit_transaction = Transaction::new_deposit(CLIENT_ID, DEPOSIT_TX, AMOUNT);
    let dispute_transaction = Transaction::new_dispute(CLIENT_ID, DEPOSIT_TX);

    bank.process_transaction(deposit_transaction).unwrap();
    bank.process_transaction(dispute_transaction).unwrap();

    let account = bank.get_account(&CLIENT_ID).unwrap();
    validate_account(&account, dec!(0.0), AMOUNT, AMOUNT);
}

#[test]
fn dispute_withdrawal_test() {
    let mut bank = Bank::new();

    let deposit_transaction = Transaction::new_deposit(CLIENT_ID, DEPOSIT_TX, AMOUNT);
    let withdrawal_transaction = Transaction::new_withdrawal(CLIENT_ID, WITHDRAWAL_TX, AMOUNT);
    let dispute_transaction = Transaction::new_dispute(CLIENT_ID, WITHDRAWAL_TX);

    bank.process_transaction(deposit_transaction).unwrap();
    bank.process_transaction(withdrawal_transaction).unwrap();
    bank.process_transaction(dispute_transaction).unwrap();

    let account = bank.get_account(&CLIENT_ID).unwrap();
    validate_account(&account, dec!(0.0), AMOUNT, AMOUNT);
}

#[test]
fn resolve_deposit_test() {
    let mut bank = Bank::new();

    let deposit_transaction = Transaction::new_deposit(CLIENT_ID, DEPOSIT_TX, AMOUNT);
    let dispute_transaction = Transaction::new_dispute(CLIENT_ID, DEPOSIT_TX);
    let resolve_transaction = Transaction::new_resolve(CLIENT_ID, DEPOSIT_TX);

    bank.process_transaction(deposit_transaction).unwrap();
    bank.process_transaction(dispute_transaction).unwrap();
    bank.process_transaction(resolve_transaction).unwrap();

    let account = bank.get_account(&CLIENT_ID).unwrap();
    validate_account(&account, AMOUNT, AMOUNT, dec!(0.0));
}

#[test]
fn resolve_withdrawal_test() {
    let mut bank = Bank::new();

    let deposit_transaction = Transaction::new_deposit(CLIENT_ID, DEPOSIT_TX, AMOUNT);
    let withdrawal_transaction = Transaction::new_withdrawal(CLIENT_ID, WITHDRAWAL_TX, AMOUNT);
    let dispute_transaction = Transaction::new_dispute(CLIENT_ID, WITHDRAWAL_TX);
    let resolve_transaction = Transaction::new_resolve(CLIENT_ID, WITHDRAWAL_TX);

    bank.process_transaction(deposit_transaction).unwrap();
    bank.process_transaction(withdrawal_transaction).unwrap();
    bank.process_transaction(dispute_transaction).unwrap();
    bank.process_transaction(resolve_transaction).unwrap();

    let account = bank.get_account(&CLIENT_ID).unwrap();
    validate_account(&account, AMOUNT, AMOUNT, dec!(0.0));
}

#[test]
fn chargeback_deposit_test() {
    let mut bank = Bank::new();

    let deposit_transaction = Transaction::new_deposit(CLIENT_ID, DEPOSIT_TX, AMOUNT);
    let dispute_transaction = Transaction::new_dispute(CLIENT_ID, DEPOSIT_TX);
    let chargeback_transaction = Transaction::new_chargeback(CLIENT_ID, DEPOSIT_TX);

    bank.process_transaction(deposit_transaction).unwrap();
    bank.process_transaction(dispute_transaction).unwrap();
    bank.process_transaction(chargeback_transaction).unwrap();

    let account = bank.get_account(&CLIENT_ID).unwrap();
    validate_account(&account, dec!(0.0), dec!(0.0), dec!(0.0));
}

#[test]
fn chargeback_withdrawal_test() {
    let mut bank = Bank::new();

    let deposit_transaction = Transaction::new_deposit(CLIENT_ID, DEPOSIT_TX, AMOUNT);
    let withdrawal_transaction = Transaction::new_withdrawal(CLIENT_ID, WITHDRAWAL_TX, AMOUNT);
    let dispute_transaction = Transaction::new_dispute(CLIENT_ID, WITHDRAWAL_TX);
    let chargeback_transaction = Transaction::new_chargeback(CLIENT_ID, WITHDRAWAL_TX);

    bank.process_transaction(deposit_transaction).unwrap();
    bank.process_transaction(withdrawal_transaction).unwrap();
    bank.process_transaction(dispute_transaction).unwrap();
    bank.process_transaction(chargeback_transaction).unwrap();

    let account = bank.get_account(&CLIENT_ID).unwrap();
    validate_account(&account, AMOUNT, AMOUNT, dec!(0.0));
}
