use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::transaction::Transaction;

#[derive(Serialize, Deserialize)]
pub struct DepositTransactionRequest {
    pub account_id: Uuid,
    pub amount: f64,
}

impl DepositTransactionRequest {
    pub fn to_transactions(&self) -> Vec<Transaction> {
        vec![Transaction::new(self.account_id, self.amount)]
    }
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawalTransactionRequest {
    pub account_id: Uuid,
    pub amount: f64,
}

impl WithdrawalTransactionRequest {
    pub fn to_transactions(&self) -> Vec<Transaction> {
        vec![Transaction::new(self.account_id, -self.amount)]
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransferTransactionRequest {
    pub from_account_id: Uuid,
    pub to_account_id: Uuid,
    pub amount: f64,
}

impl TransferTransactionRequest {
    pub fn to_transactions(&self) -> Vec<Transaction> {
        vec![
            Transaction::new(self.from_account_id, -self.amount),
            Transaction::new(self.to_account_id, self.amount),
        ]
    }
}
