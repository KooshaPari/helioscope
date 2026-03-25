//! Ports - Interface definitions for teammates

use crate::domain::{DelegationRequest, DelegationResult, HealthStatus, Teammate};

/// Inbound port - Teammate registry operations
pub trait TeammateRegistryPort: Send + Sync {
    fn register(&self, teammate: Teammate);
    fn get(&self, id: &str) -> Option<Teammate>;
    fn list(&self) -> Vec<Teammate>;
    fn find_by_role(&self, role: &str) -> Vec<Teammate>;
    fn unregister(&self, id: &str) -> bool;
}

/// Inbound port - Delegation operations
pub trait DelegationPort: Send + Sync {
    fn submit(&self, request: DelegationRequest) -> DelegationResult;
    fn status(&self, delegation_id: &str) -> Option<DelegationResult>;
    fn cancel(&self, delegation_id: &str) -> bool;
}

/// Outbound port - Health checking
pub trait HealthCheckPort: Send + Sync {
    fn check_health(&self, teammate_id: &str) -> HealthStatus;
    fn healthy_teammates(&self) -> Vec<Teammate>;
}
