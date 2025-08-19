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

        let (from_tx, to_tx) = input.to_transactions(&from, &to);
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

#[cfg(test)]
mod tests {

    use bigdecimal::{BigDecimal, FromPrimitive};
    use uuid::Uuid;

    use crate::storage::InMemoryStorage;

    use super::*;

    async fn setup() -> (
        Arc<Mutex<InMemoryStorage>>,
        TransferUseCase<InMemoryStorage>,
        Account,
        Account,
    ) {
        let mut from = Account::new(Uuid::new_v4(), &"BRL".to_string());
        from.balance = BigDecimal::from_f64(100.0).unwrap();
        let to = Account::new(Uuid::new_v4(), &"BRL".to_string());
        let storage = Arc::new(Mutex::new(InMemoryStorage::new()));
        storage
            .lock()
            .await
            .set_accounts(
                vec![(from.uuid, from.clone()), (to.uuid, to.clone())]
                    .into_iter()
                    .collect(),
            )
            .await;
        (storage.clone(), TransferUseCase::new(storage), from, to)
    }

    async fn assert_balances(
        storage: Arc<Mutex<InMemoryStorage>>,
        (from_uuid, to_uuid): (Uuid, Uuid),
        (from_balance, to_balance): (f64, f64),
    ) {
        let storage = storage.lock().await;
        let final_from = storage.get_account(from_uuid).await.unwrap().unwrap();
        assert_eq!(
            final_from.balance,
            BigDecimal::from_f64(from_balance).unwrap()
        );
        let final_to = storage.get_account(to_uuid).await.unwrap().unwrap();
        assert_eq!(final_to.balance, BigDecimal::from_f64(to_balance).unwrap());
    }

    #[tokio::test]
    async fn test_execute_successful() {
        let (storage, use_case, from, to) = setup().await;

        let result = use_case
            .execute(TransferTransactionDTO {
                idempotency_key: String::from("idemp_1"),
                from_account_id: from.uuid,
                to_account_id: to.uuid,
                amount: BigDecimal::from_f64(39.99).unwrap(),
            })
            .await;
        assert!(result.is_ok());
        let expected_txs = result.unwrap();
        assert_eq!(expected_txs.len(), 2);
        let tx_from = &expected_txs[0];
        assert_eq!(tx_from.amount, BigDecimal::from_f64(-39.99).unwrap());
        assert_eq!(tx_from.account_id, from.uuid);
        let tx_to = &expected_txs[1];
        assert_eq!(tx_to.amount, BigDecimal::from_f64(39.99).unwrap());
        assert_eq!(tx_to.account_id, to.uuid);
        assert_balances(storage.clone(), (from.uuid, to.uuid), (60.01, 39.99)).await;
    }

    #[tokio::test]
    async fn test_from_account_not_found() {
        let (storage, use_case, from, to) = setup().await;

        let result = use_case
            .execute(TransferTransactionDTO {
                idempotency_key: String::from("idemp_1"),
                from_account_id: Uuid::new_v4(),
                to_account_id: to.uuid,
                amount: BigDecimal::from_f64(99.99).unwrap(),
            })
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Source account not found");
        assert_balances(storage.clone(), (from.uuid, to.uuid), (100.00, 0.00)).await;
    }

    #[tokio::test]
    async fn test_to_account_not_found() {
        let (storage, use_case, from, to) = setup().await;

        let result = use_case
            .execute(TransferTransactionDTO {
                idempotency_key: String::from("idemp_1"),
                from_account_id: from.uuid,
                to_account_id: Uuid::new_v4(),
                amount: BigDecimal::from_f64(99.99).unwrap(),
            })
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Destination account not found");
        assert_balances(storage.clone(), (from.uuid, to.uuid), (100.00, 0.00)).await;
    }

    #[tokio::test]
    async fn test_insufficient_balance() {
        let (storage, use_case, from, to) = setup().await;

        let result = use_case
            .execute(TransferTransactionDTO {
                idempotency_key: String::from("idemp_1"),
                from_account_id: from.uuid,
                to_account_id: to.uuid,
                amount: BigDecimal::from_f64(100.99).unwrap(),
            })
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient balance");
        assert_balances(storage.clone(), (from.uuid, to.uuid), (100.00, 0.00)).await;
    }
}
