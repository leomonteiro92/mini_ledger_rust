use actix_web::{get, post, web, HttpResponse, Responder};
use base::dto::account::AccountCreationDTO;
use uuid::Uuid;

use super::state::AppState;

#[post("/accounts")]
pub async fn create_account(
    state: web::Data<AppState>,
    account_creation_request: web::Json<AccountCreationDTO>,
) -> impl Responder {
    let input = account_creation_request.into_inner();
    let created_account_result = state.create_account_uc.execute(input).await;
    created_account_result
        .map(|created_account| HttpResponse::Ok().json(created_account))
        .unwrap_or_else(|error| HttpResponse::BadRequest().body(error))
}

#[get("/accounts/{param_uuid}")]
pub async fn get_account_by_id(
    state: web::Data<AppState>,
    param_uuid: web::Path<Uuid>,
) -> impl Responder {
    let account_result = state
        .get_account_by_id_uc
        .execute(param_uuid.into_inner())
        .await;
    account_result
        .map(|account| HttpResponse::Ok().json(account))
        .unwrap_or_else(|error| HttpResponse::NotFound().body(error))
}
