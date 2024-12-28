use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    dto::transaction::{
        DepositTransactionRequest, TransferTransactionRequest, WithdrawalTransactionRequest,
    },
    model::{account::Account, transaction::Transaction},
};

#[async_trait]
pub trait AccountService: Send + Sync {
    async fn create_one(&self, account: Account) -> Result<Account, String>;
    async fn get_by_uuid(&self, uuid: Uuid) -> Result<Option<Account>, String>;
}

#[async_trait]
pub trait TransactionService: Send + Sync {
    async fn deposit(&self, request: DepositTransactionRequest)
        -> Result<Vec<Transaction>, String>;
    async fn withdrawal(
        &self,
        request: WithdrawalTransactionRequest,
    ) -> Result<Vec<Transaction>, String>;
    async fn transfer(
        &self,
        request: TransferTransactionRequest,
    ) -> Result<Vec<Transaction>, String>;
}
