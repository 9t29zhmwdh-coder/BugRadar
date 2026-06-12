use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("AI provider error: {0}")]
    Ai(String),
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

impl serde::Serialize for BrError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

pub type BrResult<T> = Result<T, BrError>;
