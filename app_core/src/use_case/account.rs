use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{model::Account, storage::Storage};

use super::UseCase;

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
