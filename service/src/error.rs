use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Could not delete file: {0}")]
    CouldNotDeleteFile(String),
}
