//! Error types for checkpoint operations

use thiserror::Error;

/// Errors that can occur during checkpoint operations
#[derive(Error, Debug)]
pub enum CheckpointError {
    /// Git operation failed
    #[error("Git error: {0}")]
    GitError(String),

    /// Repository not found
    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    /// Checkpoint not found
    #[error("Checkpoint not found: {0}")]
    CheckpointNotFound(String),

    /// Failed to create checkpoint
    #[error("Failed to create checkpoint: {0}")]
    CreateFailed(String),

    /// Failed to restore checkpoint
    #[error("Failed to restore checkpoint: {0}")]
    RestoreFailed(String),

    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Result type for checkpoint operations
pub type Result<T> = std::result::Result<T, CheckpointError>;
