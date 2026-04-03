#[derive(Debug)]
pub enum CacheError {
    Redis(String),
    Serialization(String),
    NotFound,
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::Redis(msg) => write!(f, "Redis error: {}", msg),
            CacheError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            CacheError::NotFound => write!(f, "Cache entry not found"),
        }
    }
}

impl std::error::Error for CacheError {}
