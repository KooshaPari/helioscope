//! Task definitions

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::id::{new_id, short_id};

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// Pending
    Pending,
    /// Queued
    Queued,
    /// Running
    Running,
    /// Completed
    Completed,
    /// Failed
    Failed,
    /// Blocked
    Blocked,
}

/// Task priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    /// Critical
    Critical = 0,
    /// High
    High = 1,
    /// Normal
    Normal = 2,
    /// Low
    Low = 3,
}

/// Executable task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique ID
    pub id: Uuid,
    
    /// Parent spec ID
    pub spec_id: String,
    
    /// Task name
    pub name: String,
    
    /// Task description
    pub description: String,
    
    /// Status
    pub status: TaskStatus,
    
    /// Priority
    pub priority: TaskPriority,
    
    /// Assigned agent ID
    pub agent_id: Option<Uuid>,
    
    /// Dependencies (task IDs)
    pub dependencies: Vec<Uuid>,
    
    /// Created at
    pub created_at: DateTime<Utc>,
    
    /// Started at
    pub started_at: Option<DateTime<Utc>>,
    
    /// Completed at
    pub completed_at: Option<DateTime<Utc>>,
    
    /// Result
    pub result: Option<TaskResult>,
    
    /// Error message
    pub error: Option<String>,
}

/// Task result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Success flag
    pub success: bool,
    
    /// Output
    pub output: String,
    
    /// Metrics
    pub metrics: TaskMetrics,
}

/// Task metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskMetrics {
    /// Duration ms
    pub duration_ms: u64,
    
    /// Tokens used
    pub tokens_used: Option<u64>,
    
    /// Cost USD
    pub cost_usd: Option<f64>,
}

impl Task {
    /// Create new task
    pub fn new(spec_id: &str, name: &str, description: &str) -> Self {
        Self {
            id: new_id(),
            spec_id: spec_id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            agent_id: None,
            dependencies: Vec::new(),
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            result: None,
            error: None,
        }
    }

    pub fn short_id(&self) -> String {
        short_id(&self.id, "task", 12)
    }
    
    /// Add dependency
    pub fn depends_on(&mut self, task_id: Uuid) {
        self.dependencies.push(task_id);
    }
    
    /// Check if ready to run
    pub fn is_ready(&self, completed_tasks: &[Uuid]) -> bool {
        self.status == TaskStatus::Pending &&
        self.dependencies.iter().all(|id| completed_tasks.contains(id))
    }
    
    /// Mark as queued
    pub fn queue(&mut self) {
        self.status = TaskStatus::Queued;
    }
    
    /// Mark as running
    pub fn start(&mut self, agent_id: Uuid) {
        self.status = TaskStatus::Running;
        self.agent_id = Some(agent_id);
        self.started_at = Some(Utc::now());
    }
    
    /// Mark as completed
    pub fn complete(&mut self, result: TaskResult) {
        self.status = if result.success {
            TaskStatus::Completed
        } else {
            TaskStatus::Failed
        };
        self.completed_at = Some(Utc::now());
        self.result = Some(result);
    }
    
    /// Mark as failed
    pub fn fail(&mut self, error: &str) {
        self.status = TaskStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error = Some(error.to_string());
    }
}
