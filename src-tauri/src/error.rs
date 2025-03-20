use serde::{Serialize, Deserialize};
use thiserror::Error;

/// Custom error types for JanusLens
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum JanusError {
    /// Git-related errors
    #[error("Git error: {0}")]
    GitError(String),
    
    /// File system I/O errors
    #[error("IO error: {0}")]
    IoError(String),
    
    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    /// Authentication errors
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    /// Network errors
    #[error("Network error: {0}")]
    NetworkError(String),
    
    /// Data parsing errors
    #[error("Parse error: {0}")]
    ParseError(String),
    
    /// Validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// Unknown or unexpected errors
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

impl From<serde_json::Error> for JanusError {
    fn from(error: serde_json::Error) -> Self {
        JanusError::ParseError(error.to_string())
    }
}

/// Convert Option to Result with a JanusError
pub trait IntoResult<T> {
    /// Convert an Option to a Result, using the provided error message if None
    fn into_result(self, error_message: &str) -> Result<T, JanusError>;
}

impl<T> IntoResult<T> for Option<T> {
    fn into_result(self, error_message: &str) -> Result<T, JanusError> {
        self.ok_or_else(|| JanusError::ValidationError(error_message.to_string()))
    }
}

// Implement From for any other error types as needed 