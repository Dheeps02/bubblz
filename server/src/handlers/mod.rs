use serde::Deserialize;

pub mod user;
pub mod room;
pub mod message;
pub mod ws;

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
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
