use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum JanusError {
    #[error("Git error: {0}")]
    GitError(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl From<git2::Error> for JanusError {
    fn from(error: git2::Error) -> Self {
        JanusError::GitError(error.to_string())
    }
}

impl From<std::io::Error> for JanusError {
    fn from(error: std::io::Error) -> Self {
        JanusError::IoError(error.to_string())
    }
}

// Implement From for any other error types as needed 