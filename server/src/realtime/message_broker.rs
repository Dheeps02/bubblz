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
        MessageBroker {
            connection_registry: ConnectionRegistry::new(),
            subscription_registry: SubscriptionRegistry::new(),
        }
    }

    pub async fn publish(&self, room_id: i64, message: Message) -> Result<(), BubblzError> {
        for user_id in self.subscription_registry.get_users(room_id)? {
            self.connection_registry.send_to_user(user_id, message.clone()).await?;
        }
        Ok(())
    }

    pub async fn add_user(&self, user_id: i64, sink: SplitSink<WebSocket, Message>) {
        self.connection_registry.add(user_id, sink).await
    }

    pub async fn remove_user(&self, user_id: i64) {
        self.connection_registry.remove(user_id).await
    }
}
