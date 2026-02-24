//! Task queue

use crate::task::{Task, TaskPriority};
use uuid::Uuid;
use std::collections::BinaryHeap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Priority queue for tasks
pub struct TaskQueue {
    queue: RwLock<BinaryHeap<QueuedTask>>,
}

struct QueuedTask {
    priority: TaskPriority,
    task_id: uuid::Uuid,
}

impl PartialEq for QueuedTask {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.task_id == other.task_id
    }
}

impl Eq for QueuedTask {}

impl PartialOrd for QueuedTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueuedTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskQueue {
    /// Create new queue
    pub fn new() -> Self {
        Self {
            queue: RwLock::new(BinaryHeap::new()),
        }
    }
    
    /// Enqueue task
    pub async fn enqueue(&self, task: &Task) {
        let queued = QueuedTask {
            priority: task.priority.clone(),
            task_id: task.id,
        };
        self.queue.write().await.push(queued);
    }
    
    /// Dequeue next task
    pub async fn dequeue(&self) -> Option<Uuid> {
        self.queue.write().await.pop().map(|t| t.task_id)
    }
    
    /// Peek without removing
    pub async fn peek(&self) -> Option<Uuid> {
        self.queue.read().await.peek().map(|t| t.task_id)
    }
    
    /// Check if empty
    pub async fn is_empty(&self) -> bool {
        self.queue.read().await.is_empty()
    }
    
    /// Size
    pub async fn len(&self) -> usize {
        self.queue.read().await.len()
    }
}
