//! Teammates module - Team coordination for heliosHarness
//! 
//! Provides teammate registry, delegation, and health checking.
//!
//! # Example
//!
//! ```rust
//! use harness_teammates::{Teammate, TeammateRegistry, TeammateRegistryPort};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() {
//!     let registry = Arc::new(TeammateRegistry::new());
//!     let teammate = Teammate::new("1", "test", "engineer", "Test teammate");
//!     registry.register(teammate).await;
//!     let all = registry.list().await;
//!     println!("Found {} teammates", all.len());
//! }
//! ```

pub mod domain;
pub mod ports;
pub mod adapters;

pub use domain::{Priority, DelegationStatus, HealthStatus, Teammate, DelegationRequest, DelegationResult};
pub use ports::{TeammateRegistryPort, DelegationPort, HealthCheckPort};
pub use adapters::{InMemoryTeammateRegistry, SimpleDelegationAdapter, HealthCheckAdapter};

use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

/// Teammate registry (backward compatible)
pub struct TeammateRegistry {
    teammates: Arc<RwLock<HashMap<String, Teammate>>>,
}

impl TeammateRegistry {
    pub fn new() -> Self {
        Self {
            teammates: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, teammate: Teammate) {
        let mut t = self.teammates.write().await;
        t.insert(teammate.id.clone(), teammate);
    }

    pub async fn get(&self, id: &str) -> Option<Teammate> {
        let t = self.teammates.read().await;
        t.get(id).cloned()
    }

    pub async fn list(&self) -> Vec<Teammate> {
        let t = self.teammates.read().await;
        t.values().cloned().collect()
    }

    pub async fn find_by_role(&self, role: &str) -> Vec<Teammate> {
        let t = self.teammates.read().await;
        t.values().filter(|tm| tm.role == role).cloned().collect()
    }

    pub async fn unregister(&self, id: &str) -> bool {
        let mut t = self.teammates.write().await;
        t.remove(id).is_some()
    }
}

impl Default for TeammateRegistry {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_teammate_creation() {
        let t = Teammate::new("1", "test", "engineer", "Test teammate");
        assert_eq!(t.id, "1");
        assert_eq!(t.name, "test");
    }

    #[test]
    fn test_delegation_request() {
        let req = DelegationRequest::new("teammate1", "do something");
        assert_eq!(req.priority, Priority::Normal);
    }

    #[tokio::test]
    async fn test_registry() {
        let reg = TeammateRegistry::new();
        let t = Teammate::new("1", "test", "engineer", "desc");
        reg.register(t).await;
        let found = reg.get("1").await;
        assert!(found.is_some());
    }
}
