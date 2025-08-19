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
    pub fn to_transaction(&self, account: &Account) -> Transaction {
        Transaction::new(account, &self.idempotency_key, &self.amount)
    }
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawalTransactionDTO {
    pub idempotency_key: String,
    pub account_id: Uuid,
    pub amount: BigDecimal,
}

impl WithdrawalTransactionDTO {
    pub fn to_transaction(&self, account: &Account) -> Transaction {
        let negative_amount = -&self.amount;
        Transaction::new(account, &self.idempotency_key, &negative_amount)
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
    pub fn to_transactions(&self, from: &Account, to: &Account) -> (Transaction, Transaction) {
        let negative_amount = -&self.amount;
        (
            Transaction::new(from, &self.idempotency_key, &negative_amount),
            Transaction::new(to, &self.idempotency_key, &self.amount),
        )
    }
}
