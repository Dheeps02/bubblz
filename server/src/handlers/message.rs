use crate::handlers::CreateMessage;
use crate::models::message::Message;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::SqlitePool;

pub async fn create_message(
    State(pool): State<SqlitePool>,
    Path(room_id): Path<i64>,
    Json(message): Json<CreateMessage>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::debug!(room_id = room_id, "adding message to databse for the room");
    let mut conn = pool.acquire().await.map_err(|err| {
        tracing::error!(error = %err, "failed to acquire database connection");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    sqlx::query("INSERT INTO messages (message_type, sender, room, timestamp, content) VALUES (?, ?, ?, ?, ?)")
        .bind("Text") // Message Type texts are the only ones hitting the endpoint. Any system type would be auto generated.
        .bind(message.sender_id)
        .bind(room_id)
        .bind(0)
        .bind(message.content)
        .execute(&mut *conn)
        .await
        .map_err(|err| {
            tracing::error!(error = %err, "failed to execute query to insert message into database");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tracing::debug!(room_id = room_id, "message successfully added to database for room_id");
    Ok(StatusCode::CREATED)
}

pub async fn get_messages(
    State(pool): State<SqlitePool>,
    Path(room_id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::debug!(room_id = room_id, "fetching messages for the room");
    let mut conn = pool.acquire().await.map_err(|err| {
        tracing::error!(error = %err, "failed to acquire database connection");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let messages = sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE room = ?")
        .bind(room_id)
        .fetch_all(&mut *conn)
        .await
        .map_err(|err| {
            tracing::error!(error = %err, "failed to execute query to fetch messages from database");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tracing::debug!(
        len_messages = messages.len(),
        room_id = room_id,
        "successfully fetched messages for room_id",
    );

    Ok(Json(messages))
}
