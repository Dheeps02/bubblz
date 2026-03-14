use thiserror::Error;

#[derive(Debug, Error)]
pub enum BubblzError {
    #[error("Mutex lock poisoned: {0}")]
    LockPoisoned(String),

    #[error("WebSocket send failed: {0}")]
    SendFailed(String),

    #[error("Deserialization failed: {0}")]
    Deserialize(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),
}
