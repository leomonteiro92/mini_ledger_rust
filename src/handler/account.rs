use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::dto::account::AccountCreationRequest;

use super::state::AppState;

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
