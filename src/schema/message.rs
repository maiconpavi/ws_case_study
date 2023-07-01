#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Message {
    action: String,
    username: String,
    content: Content,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Content {
    Message(String),
}
