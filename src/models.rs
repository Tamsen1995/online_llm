use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}
