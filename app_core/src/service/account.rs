use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{model::Account, storage::Storage};

use super::AccountService;

#[derive(Debug, Clone)]
pub struct AccountServiceImpl<S: Storage> {
    storage: Arc<Mutex<S>>,
}

impl<S: Storage> AccountServiceImpl<S> {
    pub fn new(storage: Arc<Mutex<S>>) -> Self {
        AccountServiceImpl { storage }
    }
}

#[async_trait]
impl<S: Storage> AccountService for AccountServiceImpl<S> {
    async fn create_one(&self, account: Account) -> Result<Account, String> {
        let storage = self.storage.lock().await;
        storage.save_account(account.clone()).await?;
        Ok(account)
    }

    async fn get_by_uuid(&self, uuid: Uuid) -> Result<Option<Account>, String> {
        let storage = self.storage.lock().await;
        let result = storage.get_account(uuid).await;
        match result {
            Ok(account) => Ok(account),
            Err(e) => Err(e),
        }
    }
}
