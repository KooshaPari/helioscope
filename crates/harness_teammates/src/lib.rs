//! Teammates module

pub mod adapters;
pub mod domain;
pub mod ports;

pub use adapters::{HealthCheckAdapter, InMemoryTeammateRegistry, SimpleDelegationAdapter};
pub use domain::{
    DelegationRequest, DelegationResult, DelegationStatus, HealthStatus, Priority, Teammate,
};
pub use ports::{DelegationPort, HealthCheckPort, TeammateRegistryPort};

use std::collections::HashMap;
use std::sync::RwLock;

pub struct TeammateRegistry {
    teammates: RwLock<HashMap<String, Teammate>>,
}

impl Default for TeammateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TeammateRegistry {
    pub fn new() -> Self {
        Self {
            teammates: RwLock::new(HashMap::new()),
        }
    }
    pub fn register(&self, t: Teammate) {
        if let Ok(mut m) = self.teammates.write() {
            m.insert(t.id.clone(), t);
        }
    }
    pub fn get(&self, id: &str) -> Option<Teammate> {
        self.teammates.read().ok()?.get(id).cloned()
    }
    pub fn list(&self) -> Vec<Teammate> {
        self.teammates
            .read()
            .ok()
            .map(|m| m.values().cloned().collect())
            .unwrap_or_default()
    }
    pub fn find_by_role(&self, role: &str) -> Vec<Teammate> {
        self.teammates
            .read()
            .ok()
            .map(|m| m.values().filter(|t| t.role == role).cloned().collect())
            .unwrap_or_default()
    }
    pub fn unregister(&self, id: &str) -> bool {
        self.teammates
            .write()
            .ok()
            .map(|mut m| m.remove(id).is_some())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reg() {
        let reg = TeammateRegistry::new();
        reg.register(Teammate::new("1", "test", "eng", "desc"));
        assert!(reg.get("1").is_some());
    }
}
