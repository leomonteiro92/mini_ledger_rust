use actix_web::{post, web, HttpResponse, Responder};

use crate::dto::transaction::{
    DepositTransactionRequest, TransferTransactionRequest, WithdrawalTransactionRequest,
};

use super::state::AppState;

#[post("/deposits")]
pub async fn create_deposit(
    state: web::Data<AppState>,
    deposit_request: web::Json<DepositTransactionRequest>,
) -> impl Responder {
    let result = state
        .transaction_service
        .deposit(deposit_request.into_inner());
    result
        .map(|txs| HttpResponse::Ok().json(txs))
        .unwrap_or_else(|error| HttpResponse::InternalServerError().body(error))
}

#[post("/withdrawals")]
pub async fn create_withdrawal(
    state: web::Data<AppState>,
    withdrawal_request: web::Json<WithdrawalTransactionRequest>,
) -> impl Responder {
    let result = state
        .transaction_service
        .withdrawal(withdrawal_request.into_inner());
    result
        .map(|txs| HttpResponse::Ok().json(txs))
        .unwrap_or_else(|error| HttpResponse::InternalServerError().body(error))
}

#[post("/transfers")]
pub async fn create_transfer(
    state: web::Data<AppState>,
    transfer_request: web::Json<TransferTransactionRequest>,
) -> impl Responder {
    let result = state
        .transaction_service
        .transfer(transfer_request.into_inner());
    result
        .map(|txs| HttpResponse::Ok().json(txs))
        .unwrap_or_else(|error| HttpResponse::InternalServerError().body(error))
}
