use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    dto::transaction::{DepositTransactionDTO, TransferTransactionDTO, WithdrawalTransactionDTO},
    model::{Account, Transaction},
    storage::Storage,
};

use super::TransactionService;

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
    async fn deposit(&self, request: DepositTransactionDTO) -> Result<Vec<Transaction>, String> {
        let storage = self.storage.lock().await;
        let to = storage
            .get_account(request.account_id.clone())
            .await?
            .ok_or("Account not found".to_string())?;
        let tx = request.to_transaction(to.clone());
        let updated_account = Account {
            balance: to.balance + tx.amount.clone(),
            ..to
        };

        let result = storage
            .save_transactions(vec![tx], vec![updated_account])
            .await?;
        Ok(result)
    }

    async fn withdrawal(
        &self,
        request: WithdrawalTransactionDTO,
    ) -> Result<Vec<Transaction>, String> {
        let storage = self.storage.lock().await;
        let from = storage
            .get_account(request.account_id.clone())
            .await?
            .ok_or("Account not found".to_string())?;
        let tx = request.to_transaction(from.clone());

        if from.balance < request.amount {
            return Err("Insufficient balance".to_string());
        }

        let updated_account = Account {
            balance: from.balance + tx.amount.clone(),
            ..from
        };

        let response = storage
            .save_transactions(vec![tx], vec![updated_account])
            .await?;
        Ok(response)
    }

    async fn transfer(&self, request: TransferTransactionDTO) -> Result<Vec<Transaction>, String> {
        let storage: tokio::sync::MutexGuard<'_, S> = self.storage.lock().await;
        let from = storage
            .get_account(request.from_account_id.clone())
            .await?
            .ok_or("Source account not found".to_string())?;
        if from.balance < request.amount {
            return Err("Insufficient balance".to_string());
        }

        let to = storage
            .get_account(request.to_account_id.clone())
            .await?
            .ok_or("Destination account not found".to_string())?;

        let (from_tx, to_tx) = request.to_transactions(from.clone(), to.clone());
        let updated_from = Account {
            balance: from.balance + from_tx.amount.clone(),
            ..from
        };
        let updated_to = Account {
            balance: to.balance + to_tx.amount.clone(),
            ..to
        };

        let transactions = storage
            .save_transactions(
                vec![from_tx.clone(), to_tx.clone()],
                vec![updated_from, updated_to],
            )
            .await?;
        Ok(transactions)
    }
}
