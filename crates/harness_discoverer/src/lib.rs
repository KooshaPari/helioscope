//! Discoverer module - Service discovery for heliosHarness
//! Find and register available services

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Service descriptor
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub metadata: HashMap<String, String>,
    pub healthy: bool,
}

/// Service registry
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a service
    pub async fn register(&self, info: ServiceInfo) {
        let mut services = self.services.write().await;
        services.insert(info.name.clone(), info);
    }

    /// Unregister a service
    pub async fn unregister(&self, name: &str) -> bool {
        let mut services = self.services.write().await;
        services.remove(name).is_some()
    }

    /// Get service by name
    pub async fn get(&self, name: &str) -> Option<ServiceInfo> {
        let services = self.services.read().await;
        services.get(name).cloned()
    }

    /// List all services
    pub async fn list(&self) -> Vec<ServiceInfo> {
        let services = self.services.read().await;
        services.values().cloned().collect()
    }

    /// Get healthy services
    pub async fn healthy(&self) -> Vec<ServiceInfo> {
        let services = self.services.read().await;
        services.values().filter(|s| s.healthy).cloned().collect()
    }

    /// Update service health
    pub async fn set_healthy(&self, name: &str, healthy: bool) -> bool {
        let mut services = self.services.write().await;
        if let Some(service) = services.get_mut(name) {
            service.healthy = healthy;
            true
        } else {
            false
        }
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
