use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{model::Account, storage::Storage, use_case::UseCase};

#[derive(Debug, Clone)]
pub struct CreateAccountUseCase<S: Storage> {
    storage: Arc<Mutex<S>>,
}

impl<S: Storage> CreateAccountUseCase<S> {
    pub fn new(storage: Arc<Mutex<S>>) -> Self {
        CreateAccountUseCase { storage }
    }
}

#[async_trait]
impl<S: Storage> UseCase<Account, Account> for CreateAccountUseCase<S> {
    async fn execute(&self, input: Account) -> Result<Account, String> {
        let storage = self.storage.lock().await;
        storage.save_account(input.clone()).await?;
        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::storage::InMemoryStorage;

    use super::*;

    #[tokio::test]
    async fn test_execute_successful() {
        let account = Account::new(Uuid::new_v4(), &"BRL".to_string());
        let storage = Arc::new(Mutex::new(InMemoryStorage::new()));
        let use_case = CreateAccountUseCase::new(storage.clone());
        let result = use_case.execute(account.clone()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), account);
        let storage = storage.lock().await;
        assert_eq!(storage.get_account(account.uuid).await, Ok(Some(account)));
    }
}
