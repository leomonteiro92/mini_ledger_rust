use std::sync::Arc;

use base::{
    dto::{
        account::AccountCreationDTO,
        transaction::{DepositTransactionDTO, TransferTransactionDTO, WithdrawalTransactionDTO},
    },
    model::{Account, Transaction},
    use_case::UseCase,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub create_account_uc: Arc<dyn UseCase<AccountCreationDTO, Account>>,
    pub get_account_by_id_uc: Arc<dyn UseCase<Uuid, Option<Account>>>,
    pub deposit_uc: Arc<dyn UseCase<DepositTransactionDTO, Vec<Transaction>>>,
    pub withdrawal_uc: Arc<dyn UseCase<WithdrawalTransactionDTO, Vec<Transaction>>>,
    pub transfer_uc: Arc<dyn UseCase<TransferTransactionDTO, Vec<Transaction>>>,
}

impl AppState {
    pub fn new(
        create_account_uc: Arc<dyn UseCase<AccountCreationDTO, Account>>,
        get_account_by_id_uc: Arc<dyn UseCase<Uuid, Option<Account>>>,
        deposit_uc: Arc<dyn UseCase<DepositTransactionDTO, Vec<Transaction>>>,
        withdrawal_uc: Arc<dyn UseCase<WithdrawalTransactionDTO, Vec<Transaction>>>,
        transfer_uc: Arc<dyn UseCase<TransferTransactionDTO, Vec<Transaction>>>,
    ) -> Self {
        AppState {
            create_account_uc,
            get_account_by_id_uc,
            deposit_uc,
            withdrawal_uc,
            transfer_uc,
        }
    }
}
