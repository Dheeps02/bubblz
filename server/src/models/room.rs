#[derive(Debug)]
pub struct Room {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub created_by: i64,
    pub created_at: i64,
}

impl Room {
    pub fn new(id: i64, name: String, description: String, created_by: i64) -> Self {
        Room {
            id,
            name,
            description,
            created_by,
            created_at: 0,
        }
    }

    pub fn is_valid_name(&self) -> bool {
        !self.name.is_empty() && self.name.len() >= 2 && self.name.len() < 32
    }
}
