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
    ws.on_upgrade(move |web_socket| async move { handle_socket(web_socket, router, params.user_id).await })
}

async fn handle_socket(mut ws: WebSocket, router: MessageBroker, user_id: i64) {
    // Add user
    let (sink, mut stream) = ws.split();
    router.add_user(user_id, sink).await;

    // Loop and look for messages
    loop {
        match stream.next().await {
            // TODO: log or handle errors from handle_frame
            Some(Ok(message)) => {
                let _ = handle_frame(&router, user_id, message).await;
            }
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

async fn handle_frame(router: &MessageBroker, user_id: i64, message: Message) -> Result<(), BubblzError> {
    match message {
        Message::Text(text) => {
            let frame: WsFrame =
                serde_json::from_str(&text).map_err(|err| BubblzError::Deserialize(err.to_string()))?;

            match frame.frame_type {
                FrameType::Message => router.publish(frame.room_id, Message::Text(text)).await,
                FrameType::Subscribe => router.subscribe(user_id, frame.room_id).await,
                FrameType::Unsubscribe => router.unsubscribe(user_id, frame.room_id).await,
            }
        }
        _ => Ok(()), // Ignore Binary, Ping, Pong, Close frames for now.
    }
}
