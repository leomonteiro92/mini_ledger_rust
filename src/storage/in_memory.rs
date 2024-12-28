use std::collections::HashMap;

use uuid::Uuid;

use crate::model::account::Account;

use super::Storage;

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

    fn get_account(&self, uuid: Uuid) -> Result<Option<Account>, String> {
        self.accounts
            .get(&uuid)
            .map(|account| Some(account.clone()))
            .ok_or_else(|| "Account not found".to_string())
    }

    fn save_transactions(
        &mut self,
        transactions: Vec<crate::model::transaction::Transaction>,
    ) -> Result<(), String> {
        for transaction in transactions {
            let account = self
                .accounts
                .get_mut(&transaction.account_id)
                .ok_or_else(|| "Account not found".to_string())?;
            account.balance += transaction.amount;
        }
        Ok(())
    }
}
