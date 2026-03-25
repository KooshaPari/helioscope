//! Ports (Interfaces/Traits) for Hexagonal Architecture
//! Define contracts between layers

use crate::domain::{CacheEntry, CachePolicy, CacheResult};
use std::future::Future;
use std::time::Duration;

/// Inbound port - driving port for cache operations
/// Implemented by the application layer
pub trait CachePort: Send + Sync {
    /// Get value by key
    fn get(&self, key: &str) -> impl Future<Output = CacheResult> + Send;
    
    /// Set value with optional TTL
    fn set(&self, key: String, value: Vec<u8>, ttl: Option<Duration>) -> impl Future<Output = Result<(), String>> + Send;
    
    /// Check if key exists
    fn contains(&self, key: &str) -> impl Future<Output = bool> + Send;
    
    /// Delete key
    fn delete(&self, key: &str) -> impl Future<Output = bool> + Send;
    
    /// Clear all entries
    fn clear(&self) -> impl Future<Output = ()> + Send;
    
    /// Get remaining TTL
    fn ttl(&self, key: &str) -> impl Future<Output = Option<Duration>> + Send;
}

/// Outbound port - driven port for cache storage
/// Implemented by adapters (memory, redis, etc.)
pub trait StoragePort: Send + Sync {
    /// Store entry
    fn store(&self, entry: CacheEntry) -> Result<(), String>;
    
    /// Retrieve entry
    fn retrieve(&self, key: &str) -> Option<CacheEntry>;
    
    /// Remove entry
    fn remove(&self, key: &str) -> bool;
    
    /// List all keys
    fn keys(&self) -> Vec<String>;
    
    /// Clear all
    fn clear(&self) -> Result<(), String>;
    
    /// Check if key exists
    fn exists(&self, key: &str) -> bool;
    
    /// Get entries matching pattern
    fn find(&self, pattern: &str) -> Vec<CacheEntry>;
}

/// Outbound port for metrics/observability
pub trait MetricsPort: Send + Sync {
    fn record_hit(&self);
    fn record_miss(&self);
    fn record_eviction(&self);
    fn record_latency(&self, duration: Duration);
    
    fn hit_rate(&self) -> f64;
    fn total_hits(&self) -> u64;
    fn total_misses(&self) -> u64;
}

/// Outbound port for cache invalidation events
pub trait InvalidationPort: Send + Sync {
    fn invalidate(&self, key: &str) -> impl Future<Output = ()> + Send;
    fn invalidate_pattern(&self, pattern: &str) -> impl Future<Output = usize> + Send;
}
