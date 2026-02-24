//! Ports - Interface definitions for teammates

use crate::domain::{Teammate, DelegationRequest, DelegationResult, HealthStatus};

/// Inbound port - Teammate registry operations
pub trait TeammateRegistryPort: Send + Sync {
    /// Register a teammate
    fn register(&self, teammate: Teammate);
    
    /// Get teammate by ID
    fn get(&self, id: &str) -> Option<Teammate>;
    
    /// List all teammates
    fn list(&self) -> Vec<Teammate>;
    
    /// Find by role
    fn find_by_role(&self, role: &str) -> Vec<Teammate>;
    
    /// Unregister
    fn unregister(&self, id: &str) -> bool;
}

/// Inbound port - Delegation operations
pub trait DelegationPort: Send + Sync {
    /// Submit a delegation request
    fn submit(&self, request: DelegationRequest) -> DelegationResult;
    
    /// Get delegation status
    fn status(&self, delegation_id: &str) -> Option<DelegationResult>;
    
    /// Cancel a delegation
    fn cancel(&self, delegation_id: &str) -> bool;
}

/// Outbound port - Health checking
pub trait HealthCheckPort: Send + Sync {
    /// Check teammate health
    fn check_health(&self, teammate_id: &str) -> HealthStatus;
    
    /// Get all healthy teammates
    fn healthy_teammates(&self) -> Vec<Teammate>;
}
