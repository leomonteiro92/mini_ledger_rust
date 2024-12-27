use uuid::Uuid;

use crate::model::account::Account;

pub trait Storage: Send + Sync {
    fn save_account(&mut self, account: Account) -> Result<(), String>;
    fn get_account(&self, uuid: Uuid) -> Result<Account, String>;
}
