//! Error types for orchestration

use thiserror::Error;

/// Errors that can occur during orchestration
#[derive(Error, Debug)]
pub enum OrchestratorError {
    /// Task decomposition failed
    #[error("Task decomposition failed: {0}")]
    DecompositionError(String),
    
    /// Agent execution failed
    #[error("Agent execution failed: {0}")]
    ExecutionError(String),
    
    /// Agent not available
    #[error("Agent not available: {0}")]
    AgentNotAvailable(String),
    
    /// Queue error
    #[error("Queue error: {0}")]
    QueueError(String),
    
    /// Timeout
    #[error("Timeout: {0}")]
    Timeout(String),
    
    /// Invalid state
    #[error("Invalid state: {0}")]
    InvalidState(String),
}

/// Result type for orchestration
pub type Result<T> = std::result::Result<T, OrchestratorError>;
