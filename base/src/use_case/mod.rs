use async_trait::async_trait;

#[async_trait]
pub trait UseCase<I, O>: Send + Sync {
    async fn execute(&self, input: I) -> Result<O, String>;
}

mod account;
mod transaction;
pub use account::{CreateAccountUseCase, GetAccountByUuidUseCase};
pub use transaction::{DepositUseCase, TransferUseCase, WithdrawalUseCase};
