use axum::extract::{
    Query, State,
    ws::{WebSocket, WebSocketUpgrade},
};
use axum::response::IntoResponse;
use serde::Deserialize;

use crate::realtime::registry::ConnectionRegistry;

#[derive(Deserialize)]
pub struct WsParams {
    pub user_id: i64,
}
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(registry): State<ConnectionRegistry>,
) -> impl IntoResponse {
    ws.on_upgrade(move |web_socket| async move { handle_socket(web_socket, registry, params.user_id).await })
}

async fn handle_socket(mut ws: WebSocket, registry: ConnectionRegistry, user_id: i64) {
    // Add user
    registry.add(user_id, ());

    // Loop and look for messages
    loop {
        match ws.recv().await {
            Some(Ok(message)) => handle_message(), // Successful Message Reception. Handle it.
            Some(Err(_)) => {
                break;
            } // Some Error Occured. Break the loop and disconnect user.
            None => {
                break;
            } // User disconnected cleanly. Break the loop and Remove user.
        }
    }

    // Remove user
    registry.remove(user_id);
}

fn handle_message() {
    // PLACEHOLDER FUNCTION
    // PLACEHODLER IMPLEMENTATION
    // TODO: forward message to pub/sub router (Issue #9)ttt
}
