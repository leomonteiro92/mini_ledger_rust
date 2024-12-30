use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    dto::transaction::TransferTransactionDTO,
    model::{Account, Transaction},
    storage::Storage,
    use_case::UseCase,
};

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
