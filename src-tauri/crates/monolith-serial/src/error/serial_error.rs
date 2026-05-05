use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerialError {
    #[error("Failed to open port: {0}")]
    OpenError(String),

    #[error("IO error: {0}")]
    IoError(String),
}
