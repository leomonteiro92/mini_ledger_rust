use async_trait::async_trait;
use uuid::Uuid;

use crate::model::{Account, Transaction};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_account(&self, account: Account) -> Result<(), String>;
    async fn get_account(&self, uuid: Uuid) -> Result<Option<Account>, String>;
    async fn save_transactions(
        &self,
        created_transactions: Vec<Transaction>,
        updated_accounts: Vec<Account>,
    ) -> Result<Vec<Transaction>, String>;
}

pub mod in_memory;
pub use in_memory::InMemoryStorage;
