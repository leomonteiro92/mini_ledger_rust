use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    dto::transaction::{DepositTransactionDTO, TransferTransactionDTO, WithdrawalTransactionDTO},
    model::{Account, Transaction},
    storage::Storage,
};

use super::UseCase;

#[derive(Debug, Clone)]
pub struct DepositUseCase<S: Storage> {
    storage: Arc<Mutex<S>>,
}

impl<S: Storage> DepositUseCase<S> {
    pub fn new(storage: Arc<Mutex<S>>) -> Self {
        DepositUseCase { storage }
    }
}

#[async_trait]
impl<S: Storage> UseCase<DepositTransactionDTO, Vec<Transaction>> for DepositUseCase<S> {
    async fn execute(&self, input: DepositTransactionDTO) -> Result<Vec<Transaction>, String> {
        let storage = self.storage.lock().await;
        let to = storage
            .get_account(input.account_id.clone())
            .await?
            .ok_or("Account not found".to_string())?;
        let tx = input.to_transaction(to.clone());
        let updated_account = Account {
            balance: to.balance + tx.amount.clone(),
            ..to
        };

        let result = storage
            .save_transactions(vec![tx], vec![updated_account])
            .await?;
        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct WithdrawalUseCase<S: Storage> {
    storage: Arc<Mutex<S>>,
}

impl<S: Storage> WithdrawalUseCase<S> {
    pub fn new(storage: Arc<Mutex<S>>) -> Self {
        WithdrawalUseCase { storage }
    }
}

#[async_trait]
impl<S: Storage> UseCase<WithdrawalTransactionDTO, Vec<Transaction>> for WithdrawalUseCase<S> {
    async fn execute(&self, input: WithdrawalTransactionDTO) -> Result<Vec<Transaction>, String> {
        let storage = self.storage.lock().await;
        let from = storage
            .get_account(input.account_id.clone())
            .await?
            .ok_or("Account not found".to_string())?;
        let tx = input.to_transaction(from.clone());

        if from.balance < input.amount {
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
}

#[derive(Debug, Clone)]
pub struct TransferUseCase<S: Storage> {
    storage: Arc<Mutex<S>>,
}

impl<S: Storage> TransferUseCase<S> {
    pub fn new(storage: Arc<Mutex<S>>) -> Self {
        TransferUseCase { storage }
    }
}

#[async_trait]
impl<S: Storage> UseCase<TransferTransactionDTO, Vec<Transaction>> for TransferUseCase<S> {
    async fn execute(&self, input: TransferTransactionDTO) -> Result<Vec<Transaction>, String> {
        let storage: tokio::sync::MutexGuard<'_, S> = self.storage.lock().await;
        let from = storage
            .get_account(input.from_account_id.clone())
            .await?
            .ok_or("Source account not found".to_string())?;
        if from.balance < input.amount {
            return Err("Insufficient balance".to_string());
        }

        let to = storage
            .get_account(input.to_account_id.clone())
            .await?
            .ok_or("Destination account not found".to_string())?;

        let (from_tx, to_tx) = input.to_transactions(from.clone(), to.clone());
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
