use uuid::Uuid;

use crate::{
    dto::transaction::{
        DepositTransactionRequest, TransferTransactionRequest, WithdrawalTransactionRequest,
    },
    model::account::Account,
};

pub trait AccountService: Send + Sync {
    fn create_one(&self, account: Account) -> Result<Account, String>;
    fn get_by_uuid(&self, uuid: Uuid) -> Result<Option<Account>, String>;
}

pub trait TransactionService: Send + Sync {
    fn deposit(&self, request: DepositTransactionRequest) -> Result<(), String>;
    fn withdrawal(&self, request: WithdrawalTransactionRequest) -> Result<(), String>;
    fn transfer(&self, request: TransferTransactionRequest) -> Result<(), String>;
}
