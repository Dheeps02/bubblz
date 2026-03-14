use crate::errors::BubblzError;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SubscriptionRegistry {
    participants_room: Arc<Mutex<HashMap<i64, HashSet<i64>>>>,
}

impl SubscriptionRegistry {
    pub fn new() -> Self {
        tracing::debug!("initialized new SubscriptionRegistry object");
        SubscriptionRegistry {
            participants_room: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn subscribe(&self, user_id: i64, room_id: i64) -> Result<(), BubblzError> {
        let mut map = self.participants_room.lock().map_err(|err| {
            tracing::error!(error = %err, "failed to acquire lock on subscription registry due to mutex poisoning");
            BubblzError::LockPoisoned(err.to_string())
        })?;

        map.entry(room_id).or_insert_with(HashSet::new).insert(user_id);
        tracing::debug!(user_id = user_id, room_id = room_id, "user subscribed to room");
        Ok(())
    }

    pub fn unsubscribe(&self, user_id: i64, room_id: i64) -> Result<(), BubblzError> {
        let mut map = self.participants_room.lock().map_err(|err| {
            tracing::error!(error = %err, "failed to acquire lock on subscription registry due to mutex poisoning");
            BubblzError::LockPoisoned(err.to_string())
        })?;

        if let Some(users) = map.get_mut(&room_id) {
            users.retain(|id| id != &user_id);
        }
        tracing::debug!(user_id = user_id, room_id = room_id, "user unsubscribed from room");
        Ok(())
    }

    pub fn get_users(&self, room_id: i64) -> Result<HashSet<i64>, BubblzError> {
        let map = self.participants_room.lock().map_err(|err| {
            tracing::error!(error = %err, "failed to acquire lock on subscription registry due to mutex poisoning");
            BubblzError::LockPoisoned(err.to_string())
        })?;
        let users = map.get(&room_id).cloned().unwrap_or_default();
        tracing::debug!(
            room_id = room_id,
            user_count = users.len(),
            "retrieved users subscribed to room"
        );
        Ok(users)
    }
}
