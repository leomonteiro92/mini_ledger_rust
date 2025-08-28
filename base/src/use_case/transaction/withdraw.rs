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
    pub fn new(storage: &Arc<Mutex<S>>) -> Self {
        WithdrawalUseCase {
            storage: Arc::clone(storage),
        }
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
        let tx = input.to_transaction(&from);

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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bigdecimal::{BigDecimal, FromPrimitive};
    use uuid::Uuid;

    use crate::storage::InMemoryStorage;

    use super::*;

    async fn setup(
        initial_accounts: HashMap<Uuid, Account>,
    ) -> (
        Arc<Mutex<InMemoryStorage>>,
        WithdrawalUseCase<InMemoryStorage>,
    ) {
        let storage = Arc::new(Mutex::new(InMemoryStorage::new()));
        storage.lock().await.set_accounts(initial_accounts).await;
        (storage.clone(), WithdrawalUseCase::new(&storage))
    }

    #[tokio::test]
    async fn test_execute_successful() {
        let mut account = Account::new(Uuid::new_v4(), &"BRL".to_string());
        account.balance = BigDecimal::from_f64(100.0).unwrap();
        let (storage, use_case) =
            setup(vec![(account.uuid, account.clone())].into_iter().collect()).await;

        let result = use_case
            .execute(WithdrawalTransactionDTO {
                idempotency_key: String::from("idemp_1"),
                account_id: account.uuid,
                amount: BigDecimal::from_f64(39.99).unwrap(),
            })
            .await;
        assert!(result.is_ok());
        let expected_txs = result.unwrap();
        assert_eq!(expected_txs.len(), 1);
        let tx = &expected_txs[0];
        assert_eq!(tx.amount, BigDecimal::from_f64(-39.99).unwrap());
        assert_eq!(tx.account_id, account.uuid);
        assert_eq!(tx.idempotency_key, String::from("idemp_1"));
        let storage = storage.lock().await;
        let account = storage.get_account(account.uuid).await.unwrap().unwrap();
        assert_eq!(account.balance, BigDecimal::from_f64(60.01).unwrap());
    }

    #[tokio::test]
    async fn test_account_not_found() {
        let (_, use_case) = setup(HashMap::new()).await;

        let result = use_case
            .execute(WithdrawalTransactionDTO {
                idempotency_key: String::from("idemp_1"),
                account_id: Uuid::new_v4(),
                amount: BigDecimal::from_f64(99.99).unwrap(),
            })
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Account not found");
    }

    #[tokio::test]
    async fn test_insufficient_balance() {
        let mut account = Account::new(Uuid::new_v4(), &"BRL".to_string());
        account.balance = BigDecimal::from_f64(100.0).unwrap();
        let (_, use_case) =
            setup(vec![(account.uuid, account.clone())].into_iter().collect()).await;

        let result = use_case
            .execute(WithdrawalTransactionDTO {
                idempotency_key: String::from("idemp_1"),
                account_id: account.uuid,
                amount: BigDecimal::from_f64(100.99).unwrap(),
            })
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient balance");
    }

    #[tokio::test]
    async fn test_execute_concurrent() {
        let mut account = Account::new(Uuid::new_v4(), &"BRL".to_string());
        account.balance = BigDecimal::from_f64(100.0).unwrap();
        let (_, use_case) =
            setup(vec![(account.uuid, account.clone())].into_iter().collect()).await;

        let use_case = Arc::new(use_case);
        let mut handles = vec![];

        for i in 0..10 {
            let use_case_clone = Arc::clone(&use_case);
            let account_id = account.uuid;
            let handle = tokio::spawn(async move {
                use_case_clone
                    .execute(WithdrawalTransactionDTO {
                        idempotency_key: format!("idemp_{}", i),
                        account_id,
                        amount: BigDecimal::from_f64(25.0).unwrap(),
                    })
                    .await
            });
            handles.push(handle);
        }

        let results = futures::future::join_all(handles).await;
        
        let mut successful_count = 0;
        let mut failed_count = 0;

        for result in results {
            let withdrawal_result = result.unwrap();
            if withdrawal_result.is_ok() {
                successful_count += 1;
            } else {
                failed_count += 1;
                assert_eq!(withdrawal_result.unwrap_err(), "Insufficient balance");
            }
        }

        assert_eq!(successful_count, 4);
        assert_eq!(failed_count, 6);
    }
}
