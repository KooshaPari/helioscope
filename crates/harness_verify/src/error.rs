//! Verification error types

use thiserror::Error;

/// Errors that can occur during verification
#[derive(Error, Debug)]
pub enum VerifyError {
    /// Test runner failed
    #[error("Test runner failed: {0}")]
    TestRunnerError(String),

    /// Security scan failed
    #[error("Security scan failed: {0}")]
    SecurityScanError(String),

    /// Performance benchmark failed
    #[error("Performance benchmark failed: {0}")]
    PerformanceError(String),

    /// Verification timeout
    #[error("Verification timeout: {0}")]
    Timeout(String),

    /// Verification failed
    #[error("Verification failed: {0}")]
    Failed(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type for verification operations
pub type Result<T> = std::result::Result<T, VerifyError>;
