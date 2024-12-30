use std::{env, sync::Arc};

use base::use_case::{
    CreateAccountUseCase, DepositUseCase, GetAccountByUuidUseCase, TransferUseCase,
    WithdrawalUseCase,
};
use tokio::sync::Mutex;

use crate::handler::AppState;

pub async fn bootstrap() -> AppState {
    let contact_points = env::var("CASSANDRA_CONTACT_POINTS")
        .unwrap_or_else(|_| panic!("CASSANDRA_CONTACT_POINTS must be set"));
    let config = cassandra_storage::CassandraConfig { contact_points };
    let session = cassandra_storage::connect(config)
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to connect to Cassandra: {}", err);
        });
    cassandra_storage::migrate(&session)
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to create tables: {}", err);
        });
    let storage = Arc::new(Mutex::new(cassandra_storage::CassandraStorage::new(
        Arc::new(session),
    )));

    // let storage = Arc::new(Mutex::new(InMemoryStorage::new()));
    let create_account_uc = Arc::new(CreateAccountUseCase::new(storage.clone()));
    let get_account_by_id_uc = Arc::new(GetAccountByUuidUseCase::new(storage.clone()));
    let deposit_uc = Arc::new(DepositUseCase::new(storage.clone()));
    let withdrawal_uc = Arc::new(WithdrawalUseCase::new(storage.clone()));
    let transfer_uc = Arc::new(TransferUseCase::new(storage.clone()));

    AppState::new(
        create_account_uc,
        get_account_by_id_uc,
        deposit_uc,
        withdrawal_uc,
        transfer_uc,
    )
}
