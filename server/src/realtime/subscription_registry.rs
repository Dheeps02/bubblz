use crate::errors::BubblzError;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SubscriptionRegistry {
    participants_room: Arc<Mutex<HashMap<i64, HashSet<i64>>>>,
}

impl SubscriptionRegistry {
    pub fn new() -> Self {
        SubscriptionRegistry {
            participants_room: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn subscribe(&self, user_id: i64, room_id: i64) -> Result<(), BubblzError> {
        let mut map = self
            .participants_room
            .lock()
            .map_err(|err| BubblzError::LockPoisoned(err.to_string()))?;

        map.entry(room_id).or_insert_with(HashSet::new).insert(user_id);

        Ok(())
    }

    pub fn unsubscribe(&self, user_id: i64, room_id: i64) -> Result<(), BubblzError> {
        let mut map = self
            .participants_room
            .lock()
            .map_err(|err| BubblzError::LockPoisoned(err.to_string()))?;

        if let Some(users) = map.get_mut(&room_id) {
            users.retain(|id| id != &user_id);
        }

        Ok(())
    }
}
