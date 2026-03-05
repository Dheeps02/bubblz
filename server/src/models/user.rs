pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: i64,
}

impl User {
    pub fn new(id: i64, username: String, email: String, password_hash: String) -> Self {
        User {
            id,
            username,
            email,
            password_hash,
            created_at: 0,
        }
    }

    pub fn is_valid_username(username: &str) -> bool {
        !username.is_empty() && username.len() >= 3 && username.len() <= 32
    }
}
