use actix_web::{web, App, HttpServer};
use base::use_case::{
    CreateAccountUseCase, DepositUseCase, GetAccountByUuidUseCase, TransferUseCase,
    WithdrawalUseCase,
};
use handler::AppState;
use std::{env, sync::Arc};
use tokio::sync::Mutex;

pub mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let config = cassandra_storage::CassandraConfig {
        contact_points: "127.0.0.1".to_owned(),
    };
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

    let state = AppState::new(
        create_account_uc,
        get_account_by_id_uc,
        deposit_uc,
        withdrawal_uc,
        transfer_uc,
    );
    let app_state = web::Data::new(state);
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(handler::account::create_account)
            .service(handler::account::get_account_by_id)
            .service(handler::transaction::create_deposit)
            .service(handler::transaction::create_withdrawal)
            .service(handler::transaction::create_transfer)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
