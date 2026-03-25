//! Sub-Agent Orchestrator
//!
//! Coordinates multiple sub-agents for parallel execution of complex specifications.

mod id;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::id::{new_id, short_id};

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Queued,
    Running,
    Completed,
    Failed,
    Blocked,
}

/// Task priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
}

/// Task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub spec_id: String,
    pub name: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub agent_id: Option<Uuid>,
    pub dependencies: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub result: Option<String>,
    pub error: Option<String>,
}

impl Task {
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

    pub fn depends_on(&mut self, task_id: Uuid) {
        self.dependencies.push(task_id);
    }

    pub fn short_id(&self) -> String {
        short_id(&self.id, "task", 12)
    }

    pub fn is_ready(&self, completed: &[Uuid]) -> bool {
        self.status == TaskStatus::Pending
            && self.dependencies.iter().all(|id| completed.contains(id))
    }
    pub fn start(&mut self, agent_id: Uuid) {
        self.status = TaskStatus::Running;
        self.agent_id = Some(agent_id);
        self.started_at = Some(Utc::now());
    }
    pub fn complete(&mut self, result: &str) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.result = Some(result.to_string());
    }
    pub fn fail(&mut self, error: &str) {
        self.status = TaskStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error = Some(error.to_string());
    }
}

/// Agent status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    Idle,
    Busy,
    Offline,
}

/// Agent capability
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentCapability {
    CodeGen,
    Testing,
    Research,
    Documentation,
    Review,
    Deployment,
    General,
}

/// Agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub capabilities: Vec<AgentCapability>,
    pub status: AgentStatus,
    pub current_task_id: Option<Uuid>,
    pub tasks_completed: u32,
    pub tasks_failed: u32,
    pub created_at: DateTime<Utc>,
    pub last_active_at: Option<DateTime<Utc>>,
}

impl Agent {
    pub fn new(name: &str, capabilities: Vec<AgentCapability>) -> Self {
        Self {
            id: new_id(),
            name: name.to_string(),
            capabilities,
            status: AgentStatus::Idle,
            current_task_id: None,
            tasks_completed: 0,
            tasks_failed: 0,
            created_at: Utc::now(),
            last_active_at: None,
        }
    }
    pub fn is_available(&self) -> bool {
        self.status == AgentStatus::Idle
    }
    pub fn assign(&mut self, task_id: Uuid) {
        self.status = AgentStatus::Busy;
        self.current_task_id = Some(task_id);
        self.last_active_at = Some(Utc::now());
    }
    pub fn release(&mut self, success: bool) {
        self.status = AgentStatus::Idle;
        self.current_task_id = None;
        self.last_active_at = Some(Utc::now());
        if success {
            self.tasks_completed += 1;
        } else {
            self.tasks_failed += 1;
        }
    }
}

/// Root Manager
pub struct RootManager {
    agents: Arc<RwLock<Vec<Agent>>>,
    tasks: Arc<RwLock<VecDeque<Task>>>,
    completed: Arc<RwLock<Vec<Uuid>>>,
}

impl RootManager {
    pub fn new() -> Self {
        Self {
            agents: Arc::new(RwLock::new(Vec::new())),
            tasks: Arc::new(RwLock::new(VecDeque::new())),
            completed: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn register_agent(&self, name: &str, caps: Vec<AgentCapability>) -> Uuid {
        let agent = Agent::new(name, caps);
        let id = agent.id;
        self.agents.write().await.push(agent);
        id
    }

    pub async fn decompose(&self, spec_name: &str) -> Vec<Task> {
        let mut tasks = Vec::new();

        let mut main = Task::new(spec_name, spec_name, &format!("Execute: {}", spec_name));
        main.priority = TaskPriority::Critical;
        tasks.push(main);

        let verify = Task::new(
            &format!("{}-verify", spec_name),
            "verify",
            "Run verification",
        );
        tasks.push(verify);

        self.tasks.write().await.extend(tasks.clone());
        tasks
    }

    pub async fn execute(&self) -> Vec<String> {
        let mut results = Vec::new();

        while let Some(mut task) = self.tasks.write().await.pop_front() {
            // Check ready
            let ready = {
                let completed = self.completed.read().await;
                task.is_ready(&completed)
            };

            if !ready {
                continue;
            }

            // Get agent
            let agent_id = {
                let agents = self.agents.read().await;
                agents.iter().find(|a| a.is_available()).map(|a| a.id)
            };

            let Some(agent_id) = agent_id else {
                continue;
            };

            // Execute
            task.start(agent_id);
            results.push(format!("Task {} executed by agent {}", task.name, agent_id));

            // Complete
            task.complete("success");
            self.completed.write().await.push(task.id);

            // Release agent
            self.agents
                .write()
                .await
                .iter_mut()
                .find(|a| a.id == agent_id)
                .map(|a| a.release(true));
        }

        results
    }

    pub async fn list_agents(&self) -> Vec<Agent> {
        self.agents.read().await.clone()
    }
}

impl Default for RootManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register() {
        let manager = RootManager::new();
        let id = manager
            .register_agent("agent1", vec![AgentCapability::General])
            .await;
        assert!(!id.is_nil());
    }

    #[tokio::test]
    async fn test_decompose() {
        let manager = RootManager::new();
        let tasks = manager.decompose("test-spec").await;
        assert_eq!(tasks.len(), 2);
    }
}
