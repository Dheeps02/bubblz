mod models;
mod db;

// use models::user::User;
// use models::room::Room;
// use models::message::{Message, MessageType};

#[tokio::main]
async fn main() {
    let pool = db::establish_db_connection("sqlite:bubblz.db?mode=rwc", "schema.sql").await.expect("Failed to establish DB connection with bubblz.db");
}
