use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use base::model::Account;
use bigdecimal::BigDecimal;
use chrono::TimeZone;
use uuid::Uuid;

use super::utils::{get_bigdecimal, get_i64, get_string, get_uuid};

pub struct AccountEntity {
    pub uuid: Uuid,
    pub currency: String,
    pub balance: BigDecimal,
    pub created_at_in_millis: i64,
    pub last_updated_at_in_millis: i64,
    pub version: Uuid,
}

impl Into<Account> for &AccountEntity {
    fn into(self) -> Account {
        Account {
            uuid: self.uuid,
            currency: self.currency.clone(),
            balance: self.balance.clone(),
            created_at: chrono::Utc
                .timestamp_millis_opt(self.created_at_in_millis)
                .unwrap(),
            last_updated_at: chrono::Utc
                .timestamp_millis_opt(self.last_updated_at_in_millis)
                .unwrap(),
            version: self.version,
        }
    }
}

impl From<&HashMap<String, AttributeValue>> for AccountEntity {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        let account = AccountEntity {
            uuid: get_uuid(value, "uuid"),
            currency: get_string(value, "currency"),
            balance: get_bigdecimal(value, "balance"),
            created_at_in_millis: get_i64(value, "created_at_in_millis"),
            last_updated_at_in_millis: get_i64(value, "last_updated_at_in_millis"),
            version: get_uuid(value, "version"),
        };
        account
    }
}
