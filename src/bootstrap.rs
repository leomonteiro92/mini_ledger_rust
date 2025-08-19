use std::{env, sync::Arc};

use base::{
    storage::InMemoryStorage,
    use_case::{
        CreateAccountUseCase, DepositUseCase, GetAccountByUuidUseCase, TransferUseCase,
        WithdrawalUseCase,
    },
};
use cassandra_storage::{CassandraConfig, CassandraStorage};
use dynamo_storage::DynamoStorage;
use tokio::sync::Mutex;

use crate::handler::AppState;

async fn _get_cassandra_storage() -> Result<Arc<Mutex<CassandraStorage>>, String> {
    let contact_points = env::var("CASSANDRA_CONTACT_POINTS")
        .map_err(|_| "CASSANDRA_CONTACT_POINTS must be set".to_string())?;

    let config = CassandraConfig { contact_points };
    let session = cassandra_storage::connect(config)
        .await
        .map_err(|err| format!("Failed to connect to Cassandra: {}", err))?;

    Ok(Arc::new(Mutex::new(CassandraStorage::new(Arc::new(
        session,
    )))))
}

async fn _get_dynamo_storage() -> Result<Arc<Mutex<DynamoStorage>>, String> {
    let client = dynamo_storage::utils::connect()
        .await
        .map_err(|err| format!("Failed to connect to DynamoDB: {}", err))?;

    // if first run, create the tables in local dynamo
    // match utils::create_table(&client).await {
    //     Ok(_) => println!("Tables created successfully"),
    //     Err(_) => println!("Error creating tables"),
    // };

    Ok(Arc::new(Mutex::new(DynamoStorage::new(Arc::new(client)))))
}

async fn get_in_memory_storage() -> Arc<Mutex<InMemoryStorage>> {
    Arc::new(Mutex::new(InMemoryStorage::new()))
}

pub async fn bootstrap() -> AppState {
    // Uncomment to use dynamo storage
    // let storage = match get_dynamo_storage().await {
    //     Ok(storage) => storage,
    //     Err(err) => panic!("{}", err),
    // };
    let storage = get_in_memory_storage().await;

    let create_account_uc = Arc::new(CreateAccountUseCase::new(&storage));
    let get_account_by_id_uc = Arc::new(GetAccountByUuidUseCase::new(&storage));
    let deposit_uc = Arc::new(DepositUseCase::new(&storage));
    let withdrawal_uc = Arc::new(WithdrawalUseCase::new(&storage));
    let transfer_uc = Arc::new(TransferUseCase::new(&storage));

    AppState::new(
        create_account_uc,
        get_account_by_id_uc,
        deposit_uc,
        withdrawal_uc,
        transfer_uc,
    )
}
