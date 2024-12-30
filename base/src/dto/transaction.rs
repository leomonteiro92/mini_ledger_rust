use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::{Account, Transaction};

#[derive(Serialize, Deserialize)]
pub struct DepositTransactionDTO {
    pub idempotency_key: String,
    pub account_id: Uuid,
    pub amount: BigDecimal,
}

impl DepositTransactionDTO {
    pub fn to_transaction(&self, account: Account) -> Transaction {
        Transaction::new(account, self.idempotency_key.clone(), self.amount.clone())
    }
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawalTransactionDTO {
    pub idempotency_key: String,
    pub account_id: Uuid,
    pub amount: BigDecimal,
}

impl WithdrawalTransactionDTO {
    pub fn to_transaction(&self, account: Account) -> Transaction {
        Transaction::new(account, self.idempotency_key.clone(), -self.amount.clone())
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransferTransactionDTO {
    pub idempotency_key: String,
    pub from_account_id: Uuid,
    pub to_account_id: Uuid,
    pub amount: BigDecimal,
}

impl TransferTransactionDTO {
    pub fn to_transactions(&self, from: Account, to: Account) -> (Transaction, Transaction) {
        (
            Transaction::new(from, self.idempotency_key.clone(), -self.amount.clone()),
            Transaction::new(to, self.idempotency_key.clone(), self.amount.clone()),
        )
    }
}
