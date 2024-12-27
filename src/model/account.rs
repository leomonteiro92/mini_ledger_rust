use chrono::{self, DateTime, Utc};
use serde::{self, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Account {
    pub uuid: Uuid,
    pub currency: String,
    pub balance: f64,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_datetime")]
    pub last_updated_at: DateTime<Utc>,
}

impl Account {
    pub fn new(uuid: Uuid, currency: String) -> Self {
        Account {
            uuid,
            currency,
            balance: 0.0,
            created_at: chrono::Utc::now(),
            last_updated_at: chrono::Utc::now(),
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
