#[derive(Debug)]
pub enum BubblzError {
    LockPoisoned(String),
}

impl std::fmt::Display for BubblzError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BubblzError::LockPoisoned(msg) => write!(f, "Lock poisoned: {}", msg),
        }
    }
}

impl std::error::Error for BubblzError {}
