use uuid::Uuid;

use crate::model::{account::Account, transaction::Transaction};

pub trait Storage: Send + Sync {
    fn save_account(&mut self, account: Account) -> Result<(), String>;
    fn get_account(&self, uuid: Uuid) -> Result<Option<Account>, String>;
    fn save_transactions(&mut self, transactions: Vec<Transaction>) -> Result<(), String>;
}
