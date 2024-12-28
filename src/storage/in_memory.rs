use std::{collections::HashMap, fmt::Error};

use uuid::Uuid;

use crate::model::{account::Account, transaction::Transaction};

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

    fn get_account(&self, uuid: Uuid) -> Result<Option<Account>, Error> {
        self.accounts
            .get(&uuid)
            .map(|account| Some(account.clone()))
            .ok_or_else(|| Error)
    }

    fn save_transactions(
        &mut self,
        transactions: Vec<crate::model::transaction::Transaction>,
    ) -> Result<Vec<Transaction>, Error> {
        let mut txs = Vec::new();
        for transaction in transactions {
            let account = self
                .accounts
                .get_mut(&transaction.account_id)
                .ok_or_else(|| Error)?;
            account.balance += transaction.amount;
            txs.push(transaction);
        }

        Ok(txs)
    }
}
