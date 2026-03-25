//! Error types for specification parsing and validation

use thiserror::Error;

/// Errors that can occur during specification processing
#[derive(Error, Debug)]
pub enum SpecError {
    /// Failed to parse YAML content
    #[error("Failed to parse YAML: {0}")]
    ParseError(String),

    /// Failed to parse JSON content
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(String),

    /// Specification is missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Specification has invalid value
    #[error("Invalid value for {field}: {message}")]
    InvalidValue { field: String, message: String },

    /// Validation failed
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Version not found
    #[error("Version not found: {0}")]
    VersionNotFound(String),

    /// Unsupported format
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
}

/// Result type for specification operations
pub type Result<T> = std::result::Result<T, SpecError>;
