use axum::extract::{
    Query, State,
    ws::{WebSocket, WebSocketUpgrade},
};
use axum::response::IntoResponse;
use futures_util::StreamExt;
use serde::Deserialize;

use crate::realtime::message_broker::MessageBroker;

#[derive(Deserialize)]
pub struct WsParams {
    pub user_id: i64,
}
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(router): State<MessageBroker>,
) -> impl IntoResponse {
    ws.on_upgrade(move |web_socket| async move { handle_socket(web_socket, router, params.user_id).await })
}

async fn handle_socket(mut ws: WebSocket, router: MessageBroker, user_id: i64) {
    // Add user
    let (sink, mut stream) = ws.split();
    router.add_user(user_id, sink).await;

    // Loop and look for messages
    loop {
        match stream.next().await {
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
    router.remove_user(user_id).await;
}

fn handle_message() {
    // PLACEHOLDER FUNCTION
    // PLACEHODLER IMPLEMENTATION
    // TODO: forward message to pub/sub router (Issue #9)
}
