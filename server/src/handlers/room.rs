use axum::{extract::{Json, State}, response::IntoResponse, http::StatusCode};
use sqlx::SqlitePool;
use crate::handlers::CreateRoom;

pub async fn create_room(State(pool): State<SqlitePool>, Json(room): Json<CreateRoom>) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = pool.acquire().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("INSERT INTO rooms (name, description, created_by, created_at) VALUES (?, ?, ?, ?)")
        .bind(room.name)
        .bind(room.description)
        .bind(room.owner_id)
        .bind(0)
        .execute(&mut *conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}