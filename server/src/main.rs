mod db;
mod errors;
mod handlers;
mod models;
mod realtime;

use crate::{
    handlers::{
        message::{create_message, get_messages},
        room::create_room,
        user::create_user,
        ws::websocket_handler,
    },
    realtime::connection_registry::ConnectionRegistry,
};
use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};
use sqlx::SqlitePool;
use tokio::net::TcpListener;

#[derive(Clone, FromRef)]
struct AppState {
    pool: SqlitePool,
    registry: ConnectionRegistry,
}

#[tokio::main]
async fn main() {
    let pool = db::establish_db_connection("sqlite:bubblz.db?mode=rwc", "schema.sql")
        .await
        .expect("Failed to establish DB connection with bubblz.db");
    let state = AppState {
        pool,
        registry: ConnectionRegistry::new(),
    };
    let router = Router::new()
        .route("/users", post(create_user))
        .route("/rooms", post(create_room))
        .route("/rooms/{id}/messages", get(get_messages).post(create_message))
        .route("/ws", get(websocket_handler))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    axum::serve(listener, router).await.expect("Failed to run server");
}
