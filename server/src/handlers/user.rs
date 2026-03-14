use crate::handlers::CreateUser;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::SqlitePool;

pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(user): Json<CreateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_info = user.clone();
    tracing::debug!(username = %user.username, "creating a new user in the database");
    let mut conn = pool.acquire().await.map_err(|err| {
        tracing::error!(error = %err, "failed to acquire database connection");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    sqlx::query("INSERT INTO users (username, password_hash, email, created_at) VALUES (?, ?, ?, ?)")
        .bind(user.username)
        .bind(user.password)
        .bind(user.email)
        .bind(0)
        .execute(&mut *conn)
        .await
        .map_err(|err| {
            tracing::error!(error = %err, "failed to execute query to insert user into database");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tracing::info!(
        user_name = %user_info.username,
        "new user successfully created in database with username"
    );
    Ok(StatusCode::CREATED)
}
