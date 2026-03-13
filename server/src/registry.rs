use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ConnectionRegistry {
    connections: Arc<Mutex<HashMap<i64, ()>>>
}

impl ConnectionRegistry {
    pub fn new() -> Self {
        ConnectionRegistry {
            connections: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn add(&self, user_id: i64, sender: ()) {
        let mut map = self.connections.lock().unwrap();
        map.insert(user_id, sender);
    }

    pub fn remove(&self, user_id: i64) {
        let mut map = self.connections.lock().unwrap();
        map.remove(&user_id);
    }
}
