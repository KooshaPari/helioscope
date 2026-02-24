//! Adapters - Concrete implementations

use crate::domain::{Teammate, DelegationRequest, DelegationResult, HealthStatus};
use crate::ports::{TeammateRegistryPort, DelegationPort, HealthCheckPort};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, instrument};

/// In-memory teammate registry
pub struct InMemoryTeammateRegistry {
    teammates: Arc<RwLock<HashMap<String, Teammate>>>,
}

impl InMemoryTeammateRegistry {
    pub fn new() -> Self {
        Self {
            teammates: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryTeammateRegistry {
    fn default() -> Self { Self::new() }
}

#[instrument(skip(self))]
impl TeammateRegistryPort for InMemoryTeammateRegistry {
    async fn register(&self, teammate: Teammate) {
        debug!(id = %teammate.id, name = %teammate.name, "registering teammate");
        let mut t = self.teammates.write().await;
        t.insert(teammate.id.clone(), teammate);
    }

    async fn get(&self, id: &str) -> Option<Teammate> {
        let t = self.teammates.read().await;
        t.get(id).cloned()
    }

    async fn list(&self) -> Vec<Teammate> {
        let t = self.teammates.read().await;
        t.values().cloned().collect()
    }

    async fn find_by_role(&self, role: &str) -> Vec<Teammate> {
        let t = self.teammates.read().await;
        t.values()
            .filter(|tm| tm.role == role)
            .cloned()
            .collect()
    }

    async fn unregister(&self, id: &str) -> bool {
        let mut t = self.teammates.write().await;
        t.remove(id).is_some()
    }
}

/// Simple delegation adapter (stub implementation)
pub struct SimpleDelegationAdapter;

impl DelegationPort for SimpleDelegationAdapter {
    async fn submit(&self, request: DelegationRequest) -> DelegationResult {
        debug!(teammate = %request.teammate_id, task = %request.task_description, "submitting delegation");
        DelegationResult::success(&request.teammate_id, format!("Completed: {}", request.task_description))
    }

    async fn status(&self, _delegation_id: &str) -> Option<DelegationResult> {
        None
    }

    async fn cancel(&self, _delegation_id: &str) -> bool {
        false
    }
}

/// Health check adapter
pub struct HealthCheckAdapter {
    registry: Arc<RwLock<HashMap<String, Teammate>>>,
}

impl HealthCheckAdapter {
    pub fn new(registry: Arc<RwLock<HashMap<String, Teammate>>>) -> Self {
        Self { registry }
    }
}

#[instrument(skip(self))]
impl HealthCheckPort for HealthCheckAdapter {
    async fn check_health(&self, _teammate_id: &str) -> HealthStatus {
        HealthStatus::Healthy
    }

    async fn healthy_teammates(&self) -> Vec<Teammate> {
        let t = self.registry.read().await;
        t.values().cloned().collect()
    }
}
