use chrono::NaiveDateTime;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Connection {
    pub connection_id: Box<str>,
    pub connected_at: NaiveDateTime,
}
