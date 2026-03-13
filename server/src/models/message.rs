use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "PascalCase")]
pub enum MessageType {
    System,
    Text,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: i64,
    pub message_type: MessageType,
    pub sender: i64,
    pub room: i64,
    pub timestamp: i64,
    pub content: String,
}

impl Message {
    pub fn new(id: i64, message_type: MessageType, sender: i64, room: i64, content: String) -> Self {
        Message {
            id,
            message_type,
            sender,
            room,
            timestamp: 0,
            content,
        }
    }
}
