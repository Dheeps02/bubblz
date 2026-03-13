use axum::extract::{
    FromRef,
    ws::{self, WebSocket},
};
use futures_util::stream::SplitSink;

use crate::{
    models::message,
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

    pub fn publish(&self, room_id: i64, message: message::Message) -> Result<(), String> {
        for user in self.subscription_registry.get_users(room_id) {}
        Ok(())
    }

    pub fn add_user(&self, user_id: i64, sink: SplitSink<WebSocket, ws::Message>) {
        self.connection_registry.add(user_id, sink)
    }

    pub fn remove_user(&self, user_id: i64) -> Result<(), String> {
        self.connection_registry.remove(user_id)
    }
}
