use async_trait::async_trait;
use uuid::Uuid;

use crate::model::{account::Account, transaction::Transaction};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_account(&self, account: Account) -> Result<(), String>;
    async fn get_account(&self, uuid: Uuid) -> Result<Option<Account>, String>;
    async fn save_transactions(
        &mut self,
        transactions: Vec<Transaction>,
    ) -> Result<Vec<Transaction>, String>;
}
