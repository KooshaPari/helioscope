//! Root Manager - orchestrates sub-agents

use crate::error::{OrchestratorError, Result};
use crate::task::{Task, TaskStatus, TaskPriority, TaskResult, TaskMetrics};
use crate::agent::{Agent, AgentPool, AgentCapability, AgentStatus};
use crate::queue::TaskQueue;
use harness_spec::models::Specification;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;

/// Root Manager - coordinates sub-agents for autonomous execution
pub struct RootManager {
    /// Agent pool
    agents: Arc<AgentPool>,
    
    /// Task queue
    queue: Arc<TaskQueue>,
    
    /// Active tasks
    tasks: RwLock<Vec<Task>>,
    
    /// Completed task IDs
    completed: RwLock<Vec<Uuid>>,
}

impl Default for RootManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RootManager {
    /// Create new root manager
    pub fn new() -> Self {
        Self {
            agents: Arc::new(AgentPool::new()),
            queue: Arc::new(TaskQueue::new()),
            tasks: RwLock::new(Vec::new()),
            completed: RwLock::new(Vec::new()),
        }
    }
    
    /// Register an agent
    pub async fn register_agent(&self, name: &str, capabilities: Vec<AgentCapability>) -> Result<Uuid> {
        let agent = Agent::new(name, capabilities);
        let id = self.agents.register(agent).await;
        Ok(id)
    }
    
    /// Decompose specification into tasks
    pub async fn decompose(&self, spec: &Specification) -> Result<Vec<Task>> {
        // Simple decomposition - in production this would be more sophisticated
        let mut tasks = Vec::new();
        
        // Create a main task
        let mut main_task = Task::new(
            &spec.spec.name,
            &spec.spec.name,
            &format!("Execute spec: {}", spec.spec.name),
        );
        main_task.priority = TaskPriority::Critical;
        
        // Create verification task
        if !spec.spec.verification.is_empty() {
            let verify_task = Task::new(
                &format!("{}-verify", spec.spec.name),
                "verification",
                "Run verification pipeline",
            );
            tasks.push(verify_task);
        }
        
        tasks.push(main_task);
        
        // Store tasks
        self.tasks.write().await.extend(tasks.clone());
        
        // Enqueue all tasks
        for task in &tasks {
            self.queue.enqueue(task).await;
        }
        
        Ok(tasks)
    }
    
    /// Execute tasks
    pub async fn execute(&self) -> Result<Vec<TaskResult>> {
        let mut results = Vec::new();
        
        loop {
            // Get next task
            let task_id = match self.queue.dequeue().await {
                Some(id) => id,
                None => break,
            };
            
            // Find the task
            let task_idx = {
                let tasks = self.tasks.read().await;
                tasks.iter().position(|t| t.id == task_id)
            };
            
            let task_idx = match task_idx {
                Some(idx) => idx,
                None => continue,
            };
            
            // Check dependencies
            let ready = {
                let task = &self.tasks.read().await[task_idx];
                let completed = self.completed.read().await;
                task.is_ready(&completed)
            };
            
            if !ready {
                // Re-queue
                self.queue.enqueue(&self.tasks.read().await[task_idx]).await;
                continue;
            }
            
            // Get available agent
            let agent = match self.agents.get_available().await {
                Some(a) => a,
                None => {
                    // Re-queue and wait
                    self.queue.enqueue(&self.tasks.read().await[task_idx]).await;
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    continue;
                }
            };
            
            // Assign task to agent
            {
                let mut t = self.tasks.write().await[task_idx].clone();
                let agent_id = agent.read().await.id;
                t.start(agent_id);
                self.tasks.write().await[task_idx] = t;
                agent.write().await.assign_task(task_id);
            }
            
            // Simulate task execution
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            // Complete task
            let success = true;
            let result = TaskResult {
                success,
                output: "Task completed".to_string(),
                metrics: TaskMetrics {
                    duration_ms: 100,
                    tokens_used: Some(1000),
                    cost_usd: Some(0.01),
                },
            };
            
            // Update task status
            {
                let mut t = self.tasks.write().await[task_idx];
                t.complete(result.clone());
            }
            
            // Update agent
            agent.write().await.complete_task(success);
            
            // Track completion
            self.completed.write().await.push(task_id);
            results.push(result);
            
            // Check if all done
            if self.queue.is_empty().await {
                break;
            }
        }
        
        Ok(results)
    }
    
    /// Get task status
    pub async fn get_task(&self, task_id: &Uuid) -> Option<Task> {
        let tasks = self.tasks.read().await;
        tasks.iter().find(|t| &t.id == task_id).cloned()
    }
    
    /// List agents
    pub async fn list_agents(&self) -> Vec<Agent> {
        self.agents.list().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_register_agent() {
        let manager = RootManager::new();
        let id = manager.register_agent("test-agent", vec![AgentCapability::General]).await.expect("register_agent failed");
        assert!(!id.is_nil());
    }
    
    #[tokio::test]
    async fn test_decompose() {
        let manager = RootManager::new();
        
        let spec = Specification {
            spec: harness_spec::models::SpecContent {
                name: "test-spec".to_string(),
                version: "1.0.0".to_string(),
                owner: "test".to_string(),
                verification: vec![],
                rollback: Default::default(),
                success_criteria: vec![],
                behavior: None,
                resources: None,
                metadata: Default::default(),
            },
        };
        
        let tasks = manager.decompose(&spec).await.unwrap();
        assert!(!tasks.is_empty());
    }
}
