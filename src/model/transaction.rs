use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub amount: f64,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
}

impl Transaction {
    pub fn new(account_id: Uuid, amount: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            account_id,
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
