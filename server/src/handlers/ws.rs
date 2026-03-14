use crate::{errors::BubblzError, realtime::message_broker::MessageBroker};
use axum::extract::{
    Query, State,
    ws::{Message, WebSocket, WebSocketUpgrade},
};
use axum::response::IntoResponse;
use futures_util::StreamExt;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WsParams {
    pub user_id: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum FrameType {
    Subscribe,
    Unsubscribe,
    Message,
}

#[derive(Deserialize)]
pub struct WsFrame {
    #[serde(rename = "type")]
    frame_type: FrameType,
    room_id: i64,
    content: Option<String>,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(router): State<MessageBroker>,
) -> impl IntoResponse {
    tracing::info!(user_id = params.user_id, "websocket upgrade request received");
    ws.on_upgrade(move |web_socket| async move { handle_socket(web_socket, router, params.user_id).await })
}

async fn handle_socket(ws: WebSocket, router: MessageBroker, user_id: i64) {
    // Add user
    let (sink, mut stream) = ws.split();
    router.add_user(user_id, sink).await;
    tracing::info!(user_id = user_id, "listening on websocket connection");
    // Loop and look for messages
    loop {
        match stream.next().await {
            // TODO: log or handle errors from handle_frame
            Some(Ok(message)) => {
                tracing::debug!(user_id = user_id, "received message from websocket connection");
                let _ = handle_frame(&router, user_id, message).await;
            }
            Some(Err(_)) => {
                tracing::error!(user_id = user_id, "error receiving message from websocket connection");
                break;
            } // Some Error Occured. Break the loop and disconnect user.
            None => {
                tracing::info!(user_id = user_id, "websocket connection closed by client");
                break;
            } // User disconnected cleanly. Break the loop and Remove user.
        }
    }

    // Remove user
    router.remove_user(user_id).await;
}

async fn handle_frame(router: &MessageBroker, user_id: i64, message: Message) -> Result<(), BubblzError> {
    match message {
        Message::Text(text) => {
            tracing::debug!(user_id = user_id, "attempting to deserialize text message into WsFrame");
            let frame: WsFrame =
                serde_json::from_str(&text).map_err(|err| {
                    tracing::error!(user_id = user_id, text = ?text, error = %err, "failed to deserialize text message into WsFrame");
                    BubblzError::Deserialize(err.to_string())
                })?;

            match frame.frame_type {
                FrameType::Message => {
                    tracing::debug!(
                        user_id = user_id,
                        room_id = frame.room_id,
                        "received Message frame. publishing message to room"
                    );
                    router.publish(frame.room_id, Message::Text(text)).await
                }
                FrameType::Subscribe => {
                    tracing::debug!(
                        user_id = user_id,
                        room_id = frame.room_id,
                        "received Subscribe frame. subscribing user to room"
                    );
                    router.subscribe(user_id, frame.room_id).await
                }
                FrameType::Unsubscribe => {
                    tracing::debug!(
                        user_id = user_id,
                        room_id = frame.room_id,
                        "received Unsubscribe frame. unsubscribing user from room"
                    );
                    router.unsubscribe(user_id, frame.room_id).await
                }
            }
        }
        _ => Ok(()), // Ignore Binary, Ping, Pong, Close frames for now.
    }
}
