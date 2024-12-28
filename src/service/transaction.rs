use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    dto::transaction::{
        DepositTransactionRequest, TransferTransactionRequest, WithdrawalTransactionRequest,
    },
    model::transaction::Transaction,
    storage::Storage,
};

use super::types::TransactionService;

#[derive(Debug, Clone)]
pub struct TransactionServiceImpl<S: Storage> {
    storage: Arc<Mutex<S>>,
}

impl<S: Storage> TransactionServiceImpl<S> {
    pub fn new(storage: Arc<Mutex<S>>) -> Self {
        TransactionServiceImpl { storage }
    }
}

#[async_trait]
impl<S: Storage> TransactionService for TransactionServiceImpl<S> {
    async fn deposit(
        &self,
        request: DepositTransactionRequest,
    ) -> Result<Vec<Transaction>, String> {
        let transactions = request.to_transactions();
        let mut storage = self.storage.lock().await;
        storage
            .get_account(request.account_id.clone())
            .await?
            .ok_or("Account not found".to_string())?;
        let result = storage.save_transactions(transactions).await?;
        Ok(result)
    }

    async fn withdrawal(
        &self,
        request: WithdrawalTransactionRequest,
    ) -> Result<Vec<Transaction>, String> {
        let transactions = request.to_transactions();
        let mut storage = self.storage.lock().await;
        let account = storage
            .get_account(request.account_id.clone())
            .await?
            .ok_or("Account not found".to_string())?;
        if account.balance < request.amount {
            return Err("Insufficient balance".to_string());
        }

        let response = storage.save_transactions(transactions).await?;
        Ok(response)
    }

    async fn transfer(
        &self,
        request: TransferTransactionRequest,
    ) -> Result<Vec<Transaction>, String> {
        let transactions = request.to_transactions();
        let mut storage = self.storage.lock().await;

        let account_from = storage
            .get_account(request.from_account_id.clone())
            .await?
            .ok_or("Source account not found".to_string())?;

        if account_from.balance < request.amount {
            return Err("Insufficient balance".to_string());
        }

        storage
            .get_account(request.to_account_id.clone())
            .await?
            .ok_or("Destination account not found".to_string())?;

        let response = storage.save_transactions(transactions).await?;
        Ok(response)
    }
}
