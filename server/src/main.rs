mod models;
mod db;

#[tokio::main]
async fn main() {
    let _pool = db::establish_db_connection("sqlite:bubblz.db?mode=rwc", "schema.sql").await.expect("Failed to establish DB connection with bubblz.db");
}
