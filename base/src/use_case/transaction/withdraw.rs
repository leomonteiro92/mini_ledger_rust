use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    dto::transaction::WithdrawalTransactionDTO,
    model::{Account, Transaction},
    storage::Storage,
    use_case::UseCase,
};

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
