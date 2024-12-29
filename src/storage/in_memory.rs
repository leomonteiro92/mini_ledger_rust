use std::collections::HashMap;

use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::model::{account::Account, transaction::Transaction};

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
        &mut self,
        transactions: Vec<crate::model::transaction::Transaction>,
    ) -> Result<Vec<Transaction>, String> {
        let mut accounts = self.accounts.lock().await;
        let mut txs = Vec::new();
        for transaction in transactions {
            let account = accounts
                .get_mut(&transaction.account_id)
                .ok_or("Account not found".to_string())?;
            let amount_to_add = BigDecimal::from_f64(transaction.amount).unwrap();
            account.balance += amount_to_add;
            account.last_updated_at = transaction.created_at;
            txs.push(transaction);
        }

        Ok(txs)
    }
}
