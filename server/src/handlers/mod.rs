use serde::Deserialize;

pub mod message;
pub mod room;
pub mod user;
pub mod ws;

#[derive(Deserialize, Clone)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Clone)]
pub struct CreateRoom {
    pub name: String,
    pub description: String,
    pub owner_id: i64,
}

#[derive(Deserialize)]
pub struct CreateMessage {
    pub content: String,
    pub sender_id: i64,
}
