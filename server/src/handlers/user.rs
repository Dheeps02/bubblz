use axum::{extract::{Json, State}, response::IntoResponse, http::StatusCode};
use sqlx::SqlitePool;
use crate::handlers::CreateUser;

pub async fn create_user(State(pool): State<SqlitePool>, Json(user): Json<CreateUser>) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = pool.acquire().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("INSERT INTO users (username, password_hash, email, created_at) VALUES (?, ?, ?, ?)")
        .bind(user.username)
        .bind(user.password)
        .bind(user.email)
        .bind(0)
        .execute(&mut *conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}