use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    dto::transaction::DepositTransactionDTO,
    model::{Account, Transaction},
    storage::Storage,
    use_case::UseCase,
};

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
        let tx = input.to_transaction(&to);
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

#[cfg(test)]
mod tests {
    use bigdecimal::{BigDecimal, FromPrimitive};
    use uuid::Uuid;

    use crate::storage::InMemoryStorage;

    use super::*;

    #[tokio::test]
    async fn test_execute_successful() {
        let account = Account::new(Uuid::new_v4(), &"BRL".to_string());
        let storage = Arc::new(Mutex::new(InMemoryStorage::new()));
        storage
            .lock()
            .await
            .set_accounts(vec![(account.uuid, account.clone())].into_iter().collect())
            .await;
        let use_case = DepositUseCase::new(storage.clone());

        let result = use_case
            .execute(DepositTransactionDTO {
                idempotency_key: String::from("idemp_1"),
                account_id: account.uuid,
                amount: BigDecimal::from_f64(99.99).unwrap(),
            })
            .await;
        assert!(result.is_ok());
        let expected_txs = result.unwrap();
        assert_eq!(expected_txs.len(), 1);
        let tx = &expected_txs[0];
        assert_eq!(tx.amount, BigDecimal::from_f64(99.99).unwrap());
        assert_eq!(tx.account_id, account.uuid);
        assert_eq!(tx.idempotency_key, String::from("idemp_1"));
        let storage = storage.lock().await;
        let account = storage.get_account(account.uuid).await.unwrap().unwrap();
        assert_eq!(account.balance, BigDecimal::from_f64(99.99).unwrap());
    }

    #[tokio::test]
    async fn test_account_not_found() {
        let account = Account::new(Uuid::new_v4(), &"BRL".to_string());
        let storage = Arc::new(Mutex::new(InMemoryStorage::new()));
        let use_case = DepositUseCase::new(storage.clone());

        let result = use_case
            .execute(DepositTransactionDTO {
                idempotency_key: String::from("idemp_1"),
                account_id: account.uuid,
                amount: BigDecimal::from_f64(99.99).unwrap(),
            })
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Account not found");
    }
}
