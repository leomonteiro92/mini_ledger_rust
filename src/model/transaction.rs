use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use super::account::Account;

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
    pub fn new(account: Account, idempotency_key: String, amount: BigDecimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            idempotency_key,
            account_version: account.version,
            account_id: account.uuid,
            currency: account.currency,
            amount,
            created_at: Utc::now(),
        }
    }
}

fn serialize_datetime<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let str = dt.to_rfc3339();
    serializer.serialize_str(&str)
}
