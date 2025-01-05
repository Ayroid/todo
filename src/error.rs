use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Failed to read storage file: {0}")]
    StorageReadErrorr(#[from] std::io::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Task with ID {0} not found")]
    TaskNotFound(u32),
}

pub type Result<T> = std::result::Result<T, TodoError>;
