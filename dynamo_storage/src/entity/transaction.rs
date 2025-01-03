use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use base::model::Transaction;
use bigdecimal::BigDecimal;
use chrono::TimeZone;
use uuid::Uuid;

use super::utils::{get_bigdecimal, get_i64, get_string, get_uuid};

pub struct TransactionEntity {
    pub id: Uuid,
    pub idempotency_key: String,
    pub account_version: Uuid,
    pub account_id: Uuid,
    pub amount: BigDecimal,
    pub created_at_in_nanos: i64,
    pub currency: String,
}

impl Into<Transaction> for &TransactionEntity {
    fn into(self) -> Transaction {
        Transaction {
            id: self.id,
            idempotency_key: self.idempotency_key.clone(),
            account_version: self.account_version,
            account_id: self.account_id,
            amount: self.amount.clone(),
            created_at: chrono::Utc.timestamp_nanos(self.created_at_in_nanos),
            currency: self.currency.clone(),
        }
    }
}

impl From<&HashMap<String, AttributeValue>> for TransactionEntity {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        let transaction = TransactionEntity {
            id: get_uuid(value, "id"),
            idempotency_key: get_string(value, "idempotency_key"),
            account_version: get_uuid(value, "account_version"),
            account_id: get_uuid(value, "account_id"),
            amount: get_bigdecimal(value, "amount"),
            created_at_in_nanos: get_i64(value, "created_at_in_nanos"),
            currency: get_string(value, "currency"),
        };
        transaction
    }
}
