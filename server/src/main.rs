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
    realtime::message_broker::MessageBroker,
};
use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};

use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tracing_subscriber;

#[derive(Clone, FromRef)]
struct AppState {
    pool: SqlitePool,
    router: MessageBroker,
}

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let pool = db::establish_db_connection("sqlite:bubblz.db?mode=rwc", "schema.sql")
        .await
        .expect("Failed to establish DB connection with bubblz.db");
    let state = AppState {
        pool,
        router: MessageBroker::new(),
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
