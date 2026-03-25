//! Error types for elicitation

use thiserror::Error;

/// Errors that can occur during elicitation
#[derive(Error, Debug)]
pub enum ElicitationError {
    /// Failed to parse intent
    #[error("Failed to parse intent: {0}")]
    ParseError(String),

    /// Ambiguous input - needs clarification
    #[error("Ambiguous input: {0}")]
    AmbiguousError(String),

    /// Invalid intent
    #[error("Invalid intent: {0}")]
    InvalidIntent(String),

    /// Generation failed
    #[error("Generation failed: {0}")]
    GenerationError(String),

    /// Classification failed
    #[error("Classification failed: {0}")]
    ClassificationError(String),
}

/// Result type for elicitation
pub type Result<T> = std::result::Result<T, ElicitationError>;
