//! Checkpoint storage

use crate::checkpoint::Checkpoint;
use crate::error::{CheckpointError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory checkpoint store
pub struct CheckpointStore {
    checkpoints: RwLock<HashMap<String, Checkpoint>>,
    by_spec: RwLock<HashMap<String, Vec<String>>>,
}

impl Default for CheckpointStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckpointStore {
    /// Create new store
    pub fn new() -> Self {
        Self {
            checkpoints: RwLock::new(HashMap::new()),
            by_spec: RwLock::new(HashMap::new()),
        }
    }

    /// Create new store with Arc for sharing
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self::new())
    }

    /// Save checkpoint
    pub async fn save(&self, checkpoint: Checkpoint) -> Result<()> {
        let id = checkpoint.id.to_string();
        let spec_id = checkpoint.spec_id.clone();

        self.checkpoints
            .write()
            .await
            .insert(id.clone(), checkpoint);

        // Update index
        let mut by_spec = self.by_spec.write().await;
        by_spec
            .entry(spec_id.clone())
            .or_insert_with(Vec::new)
            .push(id);

        Ok(())
    }

    /// Get checkpoint by ID
    pub async fn get(&self, id: &str) -> Result<Checkpoint> {
        self.checkpoints
            .read()
            .await
            .get(id)
            .cloned()
            .ok_or_else(|| CheckpointError::CheckpointNotFound(id.to_string()))
    }

    /// Get checkpoints by spec ID
    pub async fn get_by_spec(&self, spec_id: &str) -> Result<Vec<Checkpoint>> {
        let ids = self
            .by_spec
            .read()
            .await
            .get(spec_id)
            .cloned()
            .unwrap_or_default();

        let checkpoints = self.checkpoints.read().await;

        Ok(ids
            .iter()
            .filter_map(|id| checkpoints.get(id).cloned())
            .collect())
    }

    /// Get latest checkpoint for spec
    pub async fn get_latest(&self, spec_id: &str) -> Result<Checkpoint> {
        let checkpoints = self.get_by_spec(spec_id).await?;

        checkpoints
            .into_iter()
            .max_by_key(|c| c.created_at)
            .ok_or_else(|| {
                CheckpointError::CheckpointNotFound(format!("No checkpoints for spec: {}", spec_id))
            })
    }

    /// Delete checkpoint
    pub async fn delete(&self, id: &str) -> Result<()> {
        if self.checkpoints.write().await.remove(id).is_none() {
            return Err(CheckpointError::CheckpointNotFound(id.to_string()));
        }
        Ok(())
    }

    /// List all checkpoints
    pub async fn list(&self) -> Vec<Checkpoint> {
        self.checkpoints.read().await.values().cloned().collect()
    }

    /// Get count
    pub async fn count(&self) -> usize {
        self.checkpoints.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store() {
        let store = CheckpointStore::new();

        let checkpoint = Checkpoint {
            id: uuid::Uuid::new_v4(),
            spec_id: "test-spec".to_string(),
            git_sha: Some("abc123".to_string()),
            git_message: None,
            config_snapshot: None,
            db_snapshot_id: None,
            metrics_baseline: None,
            created_at: chrono::Utc::now(),
            status: crate::checkpoint::CheckpointStatus::Complete,
            metadata: std::collections::HashMap::new(),
        };

        store.save(checkpoint.clone()).await.unwrap();

        let retrieved = store.get(&checkpoint.id.to_string()).await.unwrap();
        assert_eq!(retrieved.spec_id, "test-spec");

        let by_spec = store.get_by_spec("test-spec").await.unwrap();
        assert_eq!(by_spec.len(), 1);
    }
}
