use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use super::account::Account;

use crate::utils::serialize_datetime;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub idempotency_key: String,
    pub account_version: Uuid,
    pub account_id: Uuid,
    pub amount: BigDecimal,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
    pub currency: String,
}

impl Transaction {
    pub fn new(account: &Account, idempotency_key: &str, amount: &BigDecimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            idempotency_key: idempotency_key.to_owned(),
            account_version: account.version,
            account_id: account.uuid,
            currency: account.currency.clone(),
            amount: amount.clone(),
            created_at: Utc::now(),
        }
    }
}
