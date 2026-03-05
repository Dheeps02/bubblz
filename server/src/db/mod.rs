use sqlx::{SqlitePool, Error};

pub async fn establish_db_connection(db_path: &str, schema_path: &str) -> Result<SqlitePool, Error> {

    let sqlite_pool: SqlitePool = SqlitePool::connect(db_path).await?;
    let schema_str: String = std::fs::read_to_string(schema_path)?;
    sqlx::query(&schema_str).execute(&sqlite_pool).await?;
    Ok(sqlite_pool)

}
