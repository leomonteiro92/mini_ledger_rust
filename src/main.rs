use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dto::account::AccountCreationRequest;
use handler::state::AppState;
use service::account::AccountServiceImpl;
use std::{
    env,
    sync::{Arc, Mutex},
};
use storage::in_memory::InMemoryStorage;
use uuid::Uuid;

mod dto;
mod handler;
mod model;
mod service;
mod storage;

#[post("/accounts")]
pub async fn create_account(
    state: web::Data<AppState>,
    account_creation_request: web::Json<AccountCreationRequest>,
) -> impl Responder {
    let account = account_creation_request.to_account();
    let created_account_result = state.account_service.create_one(account);
    created_account_result
        .map(|created_account| HttpResponse::Ok().json(created_account))
        .unwrap_or_else(|error| HttpResponse::InternalServerError().body(error))
}

#[get("/accounts/{param_uuid}")]
pub async fn get_account_by_id(
    state: web::Data<AppState>,
    param_uuid: web::Path<Uuid>,
) -> impl Responder {
    let account_result = state.account_service.get_by_uuid(param_uuid.into_inner());
    account_result
        .map(|account| HttpResponse::Ok().json(account))
        .unwrap_or_else(|error| HttpResponse::NotFound().body(error))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let storage = Arc::new(Mutex::new(InMemoryStorage::new()));
    let account_service = Arc::new(AccountServiceImpl::new(storage));
    let state = AppState::new(account_service);
    let app_state = web::Data::new(state);
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(create_account)
            .service(get_account_by_id)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
