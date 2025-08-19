use actix_web::{post, web, HttpResponse, Responder};
use base::dto::transaction::{
    DepositTransactionDTO, TransferTransactionDTO, WithdrawalTransactionDTO,
};

use super::state::AppState;

#[post("/deposits")]
pub async fn create_deposit(
    state: web::Data<AppState>,
    deposit_request: web::Json<DepositTransactionDTO>,
) -> impl Responder {
    let result = state.deposit_uc.execute(deposit_request.into_inner()).await;
    result
        .map(|txs| HttpResponse::Created().json(txs))
        .unwrap_or_else(|error| HttpResponse::InternalServerError().body(error))
}

#[post("/withdrawals")]
pub async fn create_withdrawal(
    state: web::Data<AppState>,
    withdrawal_request: web::Json<WithdrawalTransactionDTO>,
) -> impl Responder {
    let result = state
        .withdrawal_uc
        .execute(withdrawal_request.into_inner())
        .await;
    result
        .map(|txs| HttpResponse::Created().json(txs))
        .unwrap_or_else(|error| HttpResponse::InternalServerError().body(error))
}

#[post("/transfers")]
pub async fn create_transfer(
    state: web::Data<AppState>,
    transfer_request: web::Json<TransferTransactionDTO>,
) -> impl Responder {
    let result = state
        .transfer_uc
        .execute(transfer_request.into_inner())
        .await;
    result
        .map(|txs| HttpResponse::Created().json(txs))
        .unwrap_or_else(|error| HttpResponse::InternalServerError().body(error))
}
