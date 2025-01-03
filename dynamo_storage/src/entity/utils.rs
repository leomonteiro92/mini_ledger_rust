use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use bigdecimal::{BigDecimal, Zero};
use uuid::Uuid;

pub fn get_string(value: &HashMap<String, AttributeValue>, key: &str) -> String {
    value
        .get(key)
        .and_then(|attr| attr.as_s().ok())
        .map(|s| s.to_string())
        .unwrap_or_default()
}

pub fn get_uuid(value: &HashMap<String, AttributeValue>, key: &str) -> Uuid {
    Uuid::parse_str(&get_string(value, key)).unwrap()
}

pub fn get_bigdecimal(value: &HashMap<String, AttributeValue>, key: &str) -> BigDecimal {
    value
        .get(key)
        .and_then(|attr| attr.as_n().ok())
        .and_then(|num| BigDecimal::parse_bytes(num.as_bytes(), 10))
        .unwrap_or_else(BigDecimal::zero)
}

pub fn get_i64(value: &HashMap<String, AttributeValue>, key: &str) -> i64 {
    value
        .get(key)
        .and_then(|attr| attr.as_n().ok())
        .and_then(|num| num.parse::<i64>().ok())
        .unwrap_or(0)
}
