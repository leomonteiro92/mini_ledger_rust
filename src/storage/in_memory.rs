use std::collections::HashMap;

use uuid::Uuid;

use crate::model::account::Account;

use super::types::Storage;

pub struct InMemoryStorage {
    accounts: HashMap<Uuid, Account>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        InMemoryStorage {
            accounts: HashMap::new(),
        }
    }
}

impl Storage for InMemoryStorage {
    fn save_account(&mut self, account: Account) -> Result<(), String> {
        self.accounts.insert(account.uuid, account.clone());
        Ok(())
    }

    fn get_account(&self, uuid: Uuid) -> Result<Account, String> {
        self.accounts
            .get(&uuid)
            .map(|account| account.clone())
            .ok_or_else(|| "Account not found".to_string())
    }
}
