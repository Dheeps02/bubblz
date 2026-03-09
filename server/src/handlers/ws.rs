use axum::extract::ws::{WebSocketUpgrade, WebSocket};
use axum::response::IntoResponse;

pub async fn websocket_handler (ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|web_socket| async move {
        handle_socket(web_socket).await
    })
}

async fn handle_socket(_ws: WebSocket) {

}
