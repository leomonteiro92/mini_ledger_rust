use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::model::{Account, Transaction};

use super::Storage;

pub struct InMemoryStorage {
    accounts: Mutex<HashMap<Uuid, Account>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        InMemoryStorage {
            accounts: Mutex::new(HashMap::new()),
        }
    }

    #[cfg(test)]
    pub async fn set_accounts(&self, accounts: HashMap<Uuid, Account>) {
        let mut storage = self.accounts.lock().await;
        *storage = accounts;
    }
}

#[async_trait]
impl Storage for InMemoryStorage {
    async fn save_account(&self, account: Account) -> Result<(), String> {
        let mut accounts = self.accounts.lock().await;
        accounts.insert(account.uuid, account.clone());
        Ok(())
    }

    async fn get_account(&self, uuid: Uuid) -> Result<Option<Account>, String> {
        let accounts = self.accounts.lock().await;
        Ok(accounts.get(&uuid).cloned())
    }

    async fn save_transactions(
        &self,
        created_transactions: Vec<Transaction>,
        updated_accounts: Vec<Account>,
    ) -> Result<Vec<Transaction>, String> {
        let mut accounts = self.accounts.lock().await;
        for account in updated_accounts {
            accounts.insert(account.uuid, account.clone());
        }

        Ok(created_transactions)
    }
}
