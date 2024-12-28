use uuid::Uuid;

use crate::{
    dto::transaction::{
        DepositTransactionRequest, TransferTransactionRequest, WithdrawalTransactionRequest,
    },
    model::{account::Account, transaction::Transaction},
};

pub trait AccountService: Send + Sync {
    fn create_one(&self, account: Account) -> Result<Account, String>;
    fn get_by_uuid(&self, uuid: Uuid) -> Result<Option<Account>, String>;
}

pub trait TransactionService: Send + Sync {
    fn deposit(&self, request: DepositTransactionRequest) -> Result<Vec<Transaction>, String>;
    fn withdrawal(&self, request: WithdrawalTransactionRequest)
        -> Result<Vec<Transaction>, String>;
    fn transfer(&self, request: TransferTransactionRequest) -> Result<Vec<Transaction>, String>;
}
