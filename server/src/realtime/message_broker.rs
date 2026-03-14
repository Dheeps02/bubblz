use axum::extract::{
    FromRef,
    ws::{Message, WebSocket},
};
use futures_util::stream::SplitSink;

use crate::{
    errors::BubblzError,
    realtime::{connection_registry::ConnectionRegistry, subscription_registry::SubscriptionRegistry},
};

#[derive(Clone, FromRef)]
pub struct MessageBroker {
    connection_registry: ConnectionRegistry,
    subscription_registry: SubscriptionRegistry,
}

impl MessageBroker {
    pub fn new() -> Self {
        tracing::debug!("initialized new MessageBroker object");
        MessageBroker {
            connection_registry: ConnectionRegistry::new(),
            subscription_registry: SubscriptionRegistry::new(),
        }
    }

    pub async fn publish(&self, room_id: i64, message: Message) -> Result<(), BubblzError> {
        tracing::debug!(room_id = room_id, "attempting to publish message to room");
        for user_id in self.subscription_registry.get_users(room_id)? {
            self.connection_registry.send_to_user(user_id, message.clone()).await?;
        }
        tracing::debug!(room_id = room_id, "message published to room successfully");
        Ok(())
    }

    pub async fn add_user(&self, user_id: i64, sink: SplitSink<WebSocket, Message>) {
        tracing::debug!(user_id = user_id, "adding user to broker");
        self.connection_registry.add(user_id, sink).await
    }

    pub async fn remove_user(&self, user_id: i64) {
        tracing::debug!(user_id = user_id, "removing user from broker");
        self.connection_registry.remove(user_id).await
    }

    pub async fn subscribe(&self, user_id: i64, room_id: i64) -> Result<(), BubblzError> {
        self.subscription_registry.subscribe(user_id, room_id)
    }

    pub async fn unsubscribe(&self, user_id: i64, room_id: i64) -> Result<(), BubblzError> {
        self.subscription_registry.unsubscribe(user_id, room_id)
    }
}
