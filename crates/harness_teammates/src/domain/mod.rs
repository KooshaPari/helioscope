//! Domain layer - Core entities for teammates

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Priority levels for tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Critical,
    High,
    Normal,
    Low,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Normal
    }
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Critical => "critical",
            Priority::High => "high",
            Priority::Normal => "normal",
            Priority::Low => "low",
        }
    }
}

/// Status of a delegation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DelegationStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Health status of a teammate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Slow,
    Unhealthy,
    Crashed,
    Unknown,
}

impl Default for HealthStatus {
    fn default() -> Self {
        HealthStatus::Unknown
    }
}

/// Teammate definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Teammate {
    pub id: String,
    pub name: String,
    pub role: String,
    pub description: String,
    #[serde(default)]
    pub system_prompt: String,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default = "default_max_concurrent")]
    pub max_concurrent: usize,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

fn default_max_concurrent() -> usize {
    1
}
fn default_timeout() -> u64 {
    300
}

impl Teammate {
    pub fn new(id: &str, name: &str, role: &str, description: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            role: role.to_string(),
            description: description.to_string(),
            system_prompt: String::new(),
            tools: Vec::new(),
            max_concurrent: 1,
            timeout_seconds: 300,
        }
    }

    pub fn with_tools(mut self, tools: Vec<String>) -> Self {
        self.tools = tools;
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout_seconds = timeout;
        self
    }
}

/// Delegation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationRequest {
    pub teammate_id: String,
    pub task_description: String,
    #[serde(default)]
    pub priority: Priority,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    #[serde(default)]
    pub context_files: Vec<String>,
}

impl DelegationRequest {
    pub fn new(teammate_id: &str, task: &str) -> Self {
        Self {
            teammate_id: teammate_id.to_string(),
            task_description: task.to_string(),
            priority: Priority::Normal,
            timeout_seconds: 300,
            context_files: Vec::new(),
        }
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
}

/// Delegation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationResult {
    pub delegation_id: String,
    pub teammate_id: String,
    pub status: DelegationStatus,
    pub result: Option<String>,
    pub error: Option<String>,
    #[serde(default)]
    pub duration_ms: u64,
    #[serde(default)]
    pub evidence: Vec<String>,
}

impl DelegationResult {
    pub fn success(teammate_id: &str, result: String) -> Self {
        Self {
            delegation_id: Uuid::new_v4().to_string(),
            teammate_id: teammate_id.to_string(),
            status: DelegationStatus::Completed,
            result: Some(result),
            error: None,
            duration_ms: 0,
            evidence: Vec::new(),
        }
    }

    pub fn failure(teammate_id: &str, error: String) -> Self {
        Self {
            delegation_id: Uuid::new_v4().to_string(),
            teammate_id: teammate_id.to_string(),
            status: DelegationStatus::Failed,
            result: None,
            error: Some(error),
            duration_ms: 0,
            evidence: Vec::new(),
        }
    }
}
