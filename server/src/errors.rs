#[derive(Debug)]
pub enum BubblzError {
    LockPoisoned(String),
    SendFailed(String),
}

impl std::fmt::Display for BubblzError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BubblzError::LockPoisoned(msg) => write!(f, "Lock poisoned: {}", msg),
            BubblzError::SendFailed(msg) => write!(f, "Failed to write to sender: {}", msg),
        }
    }
}

impl std::error::Error for BubblzError {}
