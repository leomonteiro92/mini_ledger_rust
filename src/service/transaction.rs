use std::sync::{Arc, Mutex};

use crate::{
    dto::transaction::{
        DepositTransactionRequest, TransferTransactionRequest, WithdrawalTransactionRequest,
    },
    model::transaction::Transaction,
    storage::Storage,
};

use super::types::TransactionService;

#[derive(Clone)]
pub struct TransactionServiceImpl {
    storage: Arc<Mutex<dyn Storage>>,
}

impl TransactionServiceImpl {
    pub fn new(storage: Arc<Mutex<dyn Storage>>) -> Self {
        TransactionServiceImpl { storage }
    }
}

impl TransactionService for TransactionServiceImpl {
    fn deposit(&self, request: DepositTransactionRequest) -> Result<Vec<Transaction>, String> {
        let transactions = request.to_transactions();
        let mut storage = self.storage.lock().unwrap();

        let account = storage.get_account(request.account_id.clone()).unwrap();
        if account.is_none() {
            return Err("Account not found".to_string());
        }

        let response = storage.save_transactions(transactions);
        match response {
            Ok(txs) => Ok(txs),
            Err(_) => Err("Error saving transactions".to_string()),
        }
    }

    fn withdrawal(
        &self,
        request: WithdrawalTransactionRequest,
    ) -> Result<Vec<Transaction>, String> {
        let transactions = request.to_transactions();
        let mut storage = self.storage.lock().unwrap();

        let result = storage.get_account(request.account_id.clone());
        match result {
            Ok(account) => match account {
                Some(account) => {
                    if account.balance < request.amount {
                        return Err("Insufficient balance".to_string());
                    }
                }
                None => return Err("Account not found".to_string()),
            },
            Err(_) => return Err("Error finding account".to_string()),
        }

        let response = storage.save_transactions(transactions);
        match response {
            Ok(txs) => Ok(txs),
            Err(_) => Err("Error saving transactions".to_string()),
        }
    }

    fn transfer(&self, request: TransferTransactionRequest) -> Result<Vec<Transaction>, String> {
        let transactions = request.to_transactions();
        let mut storage = self.storage.lock().unwrap();

        let account_from = storage
            .get_account(request.from_account_id.clone())
            .unwrap();
        if account_from.is_none() {
            return Err("Source account not found".to_string());
        }
        if account_from.unwrap().balance < request.amount {
            return Err("Insufficient balance".to_string());
        }
        let account_to = storage.get_account(request.to_account_id.clone()).unwrap();
        if account_to.is_none() {
            return Err("Destination account not found".to_string());
        }

        let response = storage.save_transactions(transactions);
        match response {
            Ok(txs) => Ok(txs),
            Err(_) => Err("Error saving transactions".to_string()),
        }
    }
}
