use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    dto::transaction::{DepositTransactionDTO, TransferTransactionDTO, WithdrawalTransactionDTO},
    model::{Account, Transaction},
};

#[async_trait]
pub trait AccountService: Send + Sync {
    async fn create_one(&self, account: Account) -> Result<Account, String>;
    async fn get_by_uuid(&self, uuid: Uuid) -> Result<Option<Account>, String>;
}

#[async_trait]
pub trait TransactionService: Send + Sync {
    async fn deposit(&self, request: DepositTransactionDTO) -> Result<Vec<Transaction>, String>;
    async fn withdrawal(
        &self,
        request: WithdrawalTransactionDTO,
    ) -> Result<Vec<Transaction>, String>;
    async fn transfer(&self, request: TransferTransactionDTO) -> Result<Vec<Transaction>, String>;
}

mod account;
mod transaction;
pub use account::AccountServiceImpl;
pub use transaction::TransactionServiceImpl;
