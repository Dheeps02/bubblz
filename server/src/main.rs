mod models;
mod db;
mod handlers;

use tokio::net::TcpListener;
use axum::{Router, routing::{post, get}};
use crate::handlers::{message::{create_message, get_messages},
                      room::create_room,
                      user::create_user};

#[tokio::main]
async fn main() {
    let pool = db::establish_db_connection("sqlite:bubblz.db?mode=rwc", "schema.sql").await.expect("Failed to establish DB connection with bubblz.db");
    let router = Router::new()
        .route("/users", post(create_user))
        .route("/rooms", post(create_room))
        .route("/rooms/{id}/messages", get(get_messages).post(create_message))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind to port 3000");
    axum::serve(listener, router).await.expect("Failed to run server");
}
