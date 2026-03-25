//! Verification result types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Unique ID
    pub id: Uuid,

    /// Spec ID
    pub spec_id: String,

    /// Verification type
    pub verification_type: VerificationType,

    /// Status
    pub status: VerificationStatus,

    /// Started at
    pub started_at: DateTime<Utc>,

    /// Completed at
    pub completed_at: Option<DateTime<Utc>>,

    /// Duration ms
    pub duration_ms: u64,

    /// Output
    pub output: String,

    /// Errors
    pub errors: Vec<String>,

    /// Metrics
    pub metrics: VerificationMetrics,
}

/// Verification type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationType {
    /// Test execution
    Test,
    /// Security scan
    Security,
    /// Performance benchmark
    Performance,
    /// Custom
    Custom,
}

/// Verification status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    /// Pending
    Pending,
    /// Running
    Running,
    /// Passed
    Passed,
    /// Failed
    Failed,
    /// Skipped
    Skipped,
    /// Timeout
    Timeout,
}

/// Verification metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerificationMetrics {
    /// Test count
    pub test_count: Option<u32>,

    /// Passed count
    pub passed_count: Option<u32>,

    /// Failed count
    pub failed_count: Option<u32>,

    /// Coverage percent
    pub coverage_percent: Option<f64>,

    /// Lines per second
    pub lines_per_second: Option<f64>,

    /// Latency p50 ms
    pub latency_p50_ms: Option<f64>,

    /// Latency p99 ms
    pub latency_p99_ms: Option<f64>,

    /// Security findings
    pub security_findings: Option<u32>,

    /// Critical findings
    pub critical_findings: Option<u32>,
}

/// Gate result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateResult {
    /// Gate name
    pub name: String,

    /// Passed
    pub passed: bool,

    /// Message
    pub message: String,

    /// Details
    pub details: Vec<GateDetail>,
}

/// Gate detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateDetail {
    /// Check name
    pub check: String,

    /// Passed
    pub passed: bool,

    /// Message
    pub message: String,
}
