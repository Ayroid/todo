use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Failed to read storage file: {0}")]
    StorageError(#[from] std::io::Error),

    #[error("Failed to parse task data: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Task with id {0} not found")]
    TaskNotFound(u32),

    #[error("Invalid task data: {0}")]
    ValidationError(String),
}

pub type Result<T> = std::result::Result<T, TodoError>;
