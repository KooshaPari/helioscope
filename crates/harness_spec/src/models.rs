//! Specification data models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Root specification container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specification {
    pub spec: SpecContent,
}

/// Core specification content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecContent {
    /// Unique name for this specification
    pub name: String,

    /// Version (semver format)
    #[serde(default = "default_version")]
    pub version: String,

    /// Owner/team responsible
    #[serde(default)]
    pub owner: String,

    /// Verification rules
    pub verification: Vec<VerificationRule>,

    /// Rollback configuration
    #[serde(default)]
    pub rollback: RollbackConfig,

    /// Success criteria
    #[serde(default)]
    pub success_criteria: Vec<SuccessCriterion>,

    /// Behavior (BDD-style)
    #[serde(default)]
    pub behavior: Option<BehaviorSpec>,

    /// Resources required
    #[serde(default)]
    pub resources: Option<Resources>,

    /// Metadata
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

/// Verification rule for execution gates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VerificationRule {
    /// Test verification
    Test {
        name: String,
        #[serde(default)]
        timeout_seconds: u32,
    },

    /// Security scan
    Security {
        scanner: String,
        #[serde(default)]
        critical_only: bool,
    },

    /// Performance benchmark
    Performance { metric: String, threshold: String },

    /// Custom verification
    Custom {
        command: String,
        #[serde(default)]
        expected_exit_code: i32,
    },
}

/// Rollback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackConfig {
    /// Rollback strategy
    #[serde(default)]
    pub strategy: RollbackStrategy,

    /// Whether checkpoint is required before execution
    #[serde(default = "default_true")]
    pub checkpoint_required: bool,

    /// Rollback timeout
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u32,
}

fn default_true() -> bool {
    true
}
fn default_timeout() -> u32 {
    30
}

impl Default for RollbackConfig {
    fn default() -> Self {
        Self {
            strategy: RollbackStrategy::GitRevert,
            checkpoint_required: true,
            timeout_seconds: 30,
        }
    }
}

/// Rollback strategy
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackStrategy {
    /// Revert git changes
    #[default]
    GitRevert,
    /// Restore from snapshot
    Snapshot,
    /// Hybrid approach
    Hybrid,
}

/// Success criterion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriterion {
    /// Metric name
    pub metric: String,

    /// Expected value or threshold
    pub threshold: Option<String>,

    /// Minimum value (for metrics that should increase)
    pub minimum: Option<f64>,

    /// Maximum value (for metrics that should decrease)
    pub maximum: Option<f64>,
}

/// Behavior specification (BDD-style)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorSpec {
    /// Given clause
    pub given: String,

    /// When clause  
    pub when: String,

    /// Then clause
    pub then: String,

    /// Additional outcomes
    #[serde(default)]
    pub and: Vec<String>,
}

/// Resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    /// CPU cores required
    #[serde(default)]
    pub cpu_cores: Option<u32>,

    /// Memory in MB
    #[serde(default)]
    pub memory_mb: Option<u64>,

    /// Timeout in seconds
    #[serde(default)]
    pub timeout_seconds: Option<u32>,
}

/// Execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    /// Unique execution ID
    pub id: Uuid,

    /// Specification ID
    pub spec_id: String,

    /// Checkpoint ID
    pub checkpoint_id: Option<String>,

    /// Status
    pub status: ExecutionStatus,

    /// Started at
    pub started_at: DateTime<Utc>,

    /// Completed at
    pub completed_at: Option<DateTime<Utc>>,

    /// Results
    pub results: Vec<ExecutionResult>,
}

/// Execution status
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    #[default]
    Pending,
    Running,
    Verifying,
    Completed,
    Failed,
    RolledBack,
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Step name
    pub step: String,

    /// Success
    pub success: bool,

    /// Message
    pub message: String,

    /// Duration ms
    pub duration_ms: u64,

    /// Error (if failed)
    #[serde(default)]
    pub error: Option<String>,
}

/// Checkpoint record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Unique checkpoint ID
    pub id: Uuid,

    /// Associated spec ID
    pub spec_id: String,

    /// Git SHA
    #[serde(default)]
    pub git_sha: Option<String>,

    /// Config snapshot
    #[serde(default)]
    pub config_snapshot: Option<serde_json::Value>,

    /// Created at
    pub created_at: DateTime<Utc>,

    /// Status
    #[serde(default)]
    pub status: CheckpointStatus,
}

/// Checkpoint status
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointStatus {
    #[default]
    Pending,
    Complete,
    Failed,
}

/// Specification parse options
#[derive(Debug, Clone, Default)]
pub struct ParseOptions {
    /// Enable strict validation
    pub strict: bool,

    /// Enable version resolution
    pub resolve_version: bool,

    /// Default version if not specified
    pub default_version: String,
}

impl ParseOptions {
    pub fn strict() -> Self {
        Self {
            strict: true,
            resolve_version: true,
            default_version: "1.0.0".to_string(),
        }
    }
}
