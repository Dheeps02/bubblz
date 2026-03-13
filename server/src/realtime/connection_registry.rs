use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, stream::SplitSink};
use tokio::sync::Mutex;

use crate::errors::BubblzError;

#[derive(Clone)]

pub struct ConnectionRegistry {
    connections: Arc<Mutex<HashMap<i64, SplitSink<WebSocket, Message>>>>,
}

impl ConnectionRegistry {
    pub fn new() -> Self {
        ConnectionRegistry {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add(&self, user_id: i64, sender: SplitSink<WebSocket, Message>) {
        let mut map = self.connections.lock().await;
        map.insert(user_id, sender);
    }

    pub async fn remove(&self, user_id: i64) {
        let mut map = self.connections.lock().await;
        map.remove(&user_id);
    }

    pub async fn send_to_user(&self, user_id: i64, message: Message) -> Result<(), BubblzError> {
        let mut map = self.connections.lock().await;

        if let Some(sink) = map.get_mut(&user_id) {
            sink.send(message)
                .await
                .map_err(|err| BubblzError::SendFailed(err.to_string()))?;
        }

        Ok(())
    }
}
