use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{
    dto::account::AccountCreationDTO, model::Account, storage::Storage, use_case::UseCase,
};

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
impl<S: Storage> UseCase<AccountCreationDTO, Account> for CreateAccountUseCase<S> {
    async fn execute(&self, input: AccountCreationDTO) -> Result<Account, String> {
        let storage = self.storage.lock().await;
        let account: Account = input.into();
        storage.save_account(account.clone()).await?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {

    use uuid::Uuid;

    use crate::storage::InMemoryStorage;

    use super::*;

    #[tokio::test]
    async fn test_execute_successful() {
        let test_uuid = Uuid::new_v4();
        let account = Account::new(test_uuid, &"BRL".to_string());
        let storage = Arc::new(Mutex::new(InMemoryStorage::new()));
        let use_case = CreateAccountUseCase::new(storage.clone());
        let input = AccountCreationDTO {
            uuid: test_uuid,
            currency: "BRL".to_string(),
        };
        let result = use_case.execute(input).await;
        assert!(result.is_ok());
        let storage = storage.lock().await;
        let stored_account = storage.get_account(test_uuid).await;
        assert!(stored_account.clone().is_ok());
        assert!(stored_account.clone().unwrap().is_some());
        let stored_account: Account = stored_account.unwrap().unwrap();
        assert_eq!(&stored_account.uuid, &account.uuid);
        assert_eq!(&stored_account.currency, &account.currency);
    }
}
