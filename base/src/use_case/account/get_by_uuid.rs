use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{model::Account, storage::Storage, use_case::UseCase};

#[derive(Debug, Clone)]
pub struct GetAccountByUuidUseCase<S: Storage> {
    storage: Arc<Mutex<S>>,
}

impl<S: Storage> GetAccountByUuidUseCase<S> {
    pub fn new(storage: Arc<Mutex<S>>) -> Self {
        GetAccountByUuidUseCase { storage }
    }
}

#[async_trait]
impl<S: Storage> UseCase<Uuid, Option<Account>> for GetAccountByUuidUseCase<S> {
    async fn execute(&self, input: Uuid) -> Result<Option<Account>, String> {
        let storage = self.storage.lock().await;
        let result = storage.get_account(input).await?;
        Ok(result)
    }
}

mod tests {

    use crate::storage::InMemoryStorage;

    use super::*;

    #[tokio::test]
    async fn test_execute_successful() {
        let test_id = Uuid::new_v4();
        let account = Account::new(test_id, &"BRL".to_string());
        let storage = Arc::new(Mutex::new(InMemoryStorage::new()));
        storage
            .lock()
            .await
            .set_accounts(vec![(test_id, account.clone())].into_iter().collect())
            .await;
        let use_case = GetAccountByUuidUseCase::new(storage.clone());
        let result = use_case.execute(test_id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(account));
    }

    #[tokio::test]
    async fn test_execute_no_account_found() {
        let test_id = Uuid::new_v4();
        let storage = Arc::new(Mutex::new(InMemoryStorage::new()));
        let use_case = GetAccountByUuidUseCase::new(storage.clone());
        let result = use_case.execute(test_id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }
}
