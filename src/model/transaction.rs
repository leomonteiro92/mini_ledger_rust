use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub amount: f64,
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
