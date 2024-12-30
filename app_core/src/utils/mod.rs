use chrono::{DateTime, Utc};
use serde::Serializer;

pub fn serialize_datetime<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let str = dt.to_rfc3339();
    serializer.serialize_str(&str)
}
