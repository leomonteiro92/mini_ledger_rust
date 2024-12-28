use std::sync::{Arc, Mutex};

use crate::{
    dto::transaction::{
        DepositTransactionRequest, TransferTransactionRequest, WithdrawalTransactionRequest,
    },
    model::account::Account,
    storage::Storage,
};

use super::{account, types::TransactionService};

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
    fn deposit(&self, request: DepositTransactionRequest) -> Result<(), String> {
        let transactions = request.to_transactions();
        let mut storage = self.storage.lock().unwrap();

        let account = storage.get_account(request.account_id.clone()).unwrap();
        if account.is_none() {
            return Err("Account not found".to_string());
        }

        storage.save_transactions(transactions)
    }

    fn withdrawal(&self, request: WithdrawalTransactionRequest) -> Result<(), String> {
        let transactions = request.to_transactions();
        let mut storage = self.storage.lock().unwrap();

        let account = storage.get_account(request.account_id.clone());
        match account {
            Ok(unwrapped_account) => {
                if unwrapped_account.is_none() {
                    return Err("Account not found".to_string());
                }
                if unwrapped_account.unwrap().balance < request.amount {
                    return Err("Insufficient balance".to_string());
                }
            }
            Err(e) => return Err(e),
        }

        storage.save_transactions(transactions)
    }

    fn transfer(&self, request: TransferTransactionRequest) -> Result<(), String> {
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

        storage.save_transactions(transactions)
    }
}
