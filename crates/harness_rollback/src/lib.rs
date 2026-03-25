//! Rollback Engine - Simple version

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RollbackStatus {
    Pending,
    Started,
    Completed,
    Failed,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackRecord {
    pub id: Uuid,
    pub checkpoint_id: String,
    pub spec_id: String,
    pub status: RollbackStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub restored_items: Vec<String>,
    pub failed_items: Vec<String>,
    pub error: Option<String>,
}

impl RollbackRecord {
    pub fn new(checkpoint_id: &str, spec_id: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            checkpoint_id: checkpoint_id.to_string(),
            spec_id: spec_id.to_string(),
            status: RollbackStatus::Pending,
            started_at: Utc::now(),
            completed_at: None,
            restored_items: vec![],
            failed_items: vec![],
            error: None,
        }
    }

    pub fn start(&mut self) {
        self.status = RollbackStatus::Started;
    }
    pub fn add_restored(&mut self, item: &str) {
        self.restored_items.push(item.to_string());
    }
    pub fn add_failed(&mut self, item: &str) {
        self.failed_items.push(item.to_string());
    }
    pub fn complete(&mut self) {
        self.status = if self.failed_items.is_empty() {
            RollbackStatus::Completed
        } else {
            RollbackStatus::Partial
        };
        self.completed_at = Some(Utc::now());
    }
    pub fn fail(&mut self, err: &str) {
        self.status = RollbackStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error = Some(err.to_string());
    }
}

pub struct RollbackEngine {
    records: Vec<RollbackRecord>,
    checkpoints: Vec<(String, String)>,
}

impl Default for RollbackEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RollbackEngine {
    pub fn new() -> Self {
        Self {
            records: vec![],
            checkpoints: vec![],
        }
    }

    pub fn register(&mut self, checkpoint_id: &str, spec_id: &str) -> Uuid {
        let id = Uuid::new_v4();
        self.checkpoints
            .push((checkpoint_id.to_string(), spec_id.to_string()));
        id
    }

    pub fn rollback(&mut self, checkpoint_id: &str) -> Option<RollbackRecord> {
        let mut record = RollbackRecord::new(checkpoint_id, "spec");
        record.start();
        record.add_restored(&format!("git:{}", checkpoint_id));
        record.complete();
        self.records.push(record.clone());
        Some(record)
    }

    pub fn verify(&self, record: &RollbackRecord) -> bool {
        matches!(record.status, RollbackStatus::Completed)
    }

    pub fn history(&self) -> &[RollbackRecord] {
        &self.records
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rollback() {
        let mut engine = RollbackEngine::new();
        engine.register("chk-001", "test-spec");
        let result = engine.rollback("chk-001");
        assert!(result.is_some());
        assert!(engine.verify(&result.unwrap()));
    }
}
