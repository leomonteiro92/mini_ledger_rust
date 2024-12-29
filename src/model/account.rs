use bigdecimal::{BigDecimal, Zero};
use chrono::{self, DateTime, TimeZone, Utc};
use serde::{self, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Account {
    pub uuid: Uuid,
    pub currency: String,
    pub balance: BigDecimal,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_datetime")]
    pub last_updated_at: DateTime<Utc>,
    pub version: Uuid,
}

impl Account {
    pub fn new(uuid: Uuid, currency: &String) -> Self {
        Account {
            uuid,
            currency: currency.clone(),
            balance: BigDecimal::zero().with_scale(2),
            created_at: chrono::Utc::now(),
            last_updated_at: chrono::Utc::now(),
            version: Uuid::new_v4(),
        }
    }

    pub fn from_storage(
        uuid: Uuid,
        currency: String,
        balance: BigDecimal,
        created_at_in_nanos: i64,
        last_updated_at_in_nanos: i64,
        version: Uuid,
    ) -> Self {
        Account {
            uuid,
            currency: currency.to_string(),
            balance,
            created_at: chrono::Utc.timestamp_nanos(created_at_in_nanos),
            last_updated_at: chrono::Utc.timestamp_nanos(last_updated_at_in_nanos),
            version,
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
