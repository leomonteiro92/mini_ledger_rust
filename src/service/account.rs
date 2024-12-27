use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::{model::account::Account, storage::types::Storage};

use super::types::AccountService;

#[derive(Clone)]
pub struct AccountServiceImpl {
    storage: Arc<Mutex<dyn Storage>>,
}

impl AccountServiceImpl {
    pub fn new(storage: Arc<Mutex<dyn Storage>>) -> Self {
        AccountServiceImpl { storage }
    }
}

impl AccountService for AccountServiceImpl {
    fn create_one(&self, account: Account) -> Result<Account, String> {
        let mut storage = self.storage.lock().unwrap();
        return storage
            .save_account(account.clone())
            .map(|_| account)
            .map_err(|error| error);
    }

    fn get_by_uuid(&self, uuid: Uuid) -> Result<Account, String> {
        let storage = self.storage.lock().unwrap();
        storage.get_account(uuid)
    }
}
