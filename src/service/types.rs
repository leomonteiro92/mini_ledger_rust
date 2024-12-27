use uuid::Uuid;

use crate::model::account::Account;

pub trait AccountService: Send + Sync {
    fn create_one(&self, account: Account) -> Result<Account, String>;
    fn get_by_uuid(&self, uuid: Uuid) -> Result<Account, String>;
}
