use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Connection {
    pub connection_id: Box<str>,
    #[serde(with = "ts_seconds")]
    pub connected_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub ttl: DateTime<Utc>,
}
