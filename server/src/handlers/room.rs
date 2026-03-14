use crate::handlers::CreateRoom;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::SqlitePool;

pub async fn create_room(
    State(pool): State<SqlitePool>,
    Json(room): Json<CreateRoom>,
) -> Result<impl IntoResponse, StatusCode> {
    let room_info = room.clone();
    tracing::debug!(room_name = %room.name, "creating a new room in the database");
    let mut conn = pool.acquire().await.map_err(|err| {
        tracing::error!(error = %err, "failed to acquire database connection");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    sqlx::query("INSERT INTO rooms (name, description, created_by, created_at) VALUES (?, ?, ?, ?)")
        .bind(room.name)
        .bind(room.description)
        .bind(room.owner_id)
        .bind(0)
        .execute(&mut *conn)
        .await
        .map_err(|err| {
            tracing::error!(error = %err, "failed to execute query to insert room into database");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tracing::info!(
        room_name = %q1qroom_info.name,
        "room successfully created in database with name"
    );
    Ok(StatusCode::CREATED)
}
