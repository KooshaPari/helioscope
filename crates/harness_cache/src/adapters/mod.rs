//! Adapters - Concrete implementations of ports
//! Secondary (driven) adapters

use crate::domain::{CacheEntry, CachePolicy, EvictionPolicy, WritePolicy};
use crate::ports::{CachePort, MetricsPort, StoragePort};
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// In-memory storage adapter
pub struct MemoryStorage {
    store: Arc<RwLock<HashMap<String, CacheEntry>>>,
    policy: CachePolicy,
}

impl MemoryStorage {
    pub fn new(policy: CachePolicy) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            policy,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(CachePolicy::default())
    }
}

impl StoragePort for MemoryStorage {
    fn store(&self, entry: CacheEntry) -> Result<(), String> {
        let mut store = self.store.write().map_err(|e| e.to_string())?;
        
        // Evict if needed
        if store.len() >= self.policy.max_size {
            self.evict_lru(&mut store);
        }
        
        store.insert(entry.key.clone(), entry);
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Option<CacheEntry> {
        let store = self.store.read().ok()?;
        store.get(key).cloned()
    }

    fn remove(&self, key: &str) -> bool {
        let mut store = self.store.write().ok()?;
        store.remove(key).is_some()
    }

    fn keys(&self) -> Vec<String> {
        self.store.read().ok().map(|s| s.keys().cloned().collect()).unwrap_or_default()
    }

    fn clear(&self) -> Result<(), String> {
        let mut store = self.store.write().map_err(|e| e.to_string())?;
        store.clear();
        Ok(())
    }

    fn exists(&self, key: &str) -> bool {
        self.store.read().ok().map(|s| s.contains_key(key)).unwrap_or(false)
    }

    fn find(&self, pattern: &str) -> Vec<CacheEntry> {
        let store = self.store.read().ok();
        match store {
            Some(s) => s.values()
                .filter(|e| e.key.contains(pattern))
                .cloned()
                .collect(),
            None => vec![],
        }
    }
}

impl MemoryStorage {
    fn evict_lru(&self, store: &mut HashMap<String, CacheEntry>) {
        if let Some((key, _)) = store.iter().min_by_key(|(_, v)| v.created_at) {
            let key = key.clone();
            store.remove(&key);
        }
    }
}

/// Metrics adapter with atomic counters
pub struct AtomicMetrics {
    hits: std::sync::atomic::AtomicU64,
    misses: std::sync::atomic::AtomicU64,
    evictions: std::sync::atomic::AtomicU64,
}

impl AtomicMetrics {
    pub fn new() -> Self {
        Self {
            hits: std::sync::atomic::AtomicU64::new(0),
            misses: std::sync::atomic::AtomicU64::new(0),
            evictions: std::sync::atomic::AtomicU64::new(0),
        }
    }
}

impl MetricsPort for AtomicMetrics {
    fn record_hit(&self) {
        self.hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn record_miss(&self) {
        self.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn record_eviction(&self) {
        self.evictions.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn record_latency(&self, _duration: Duration) {
        // Could add histogram tracking here
    }

    fn hit_rate(&self) -> f64 {
        let hits = self.hits.load(std::sync::atomic::Ordering::Relaxed);
        let misses = self.misses.load(std::sync::atomic::Ordering::Relaxed);
        let total = hits + misses;
        if total == 0 { 0.0 } else { hits as f64 / total as f64 }
    }

    fn total_hits(&self) -> u64 {
        self.hits.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn total_misses(&self) -> u64 {
        self.misses.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for AtomicMetrics {
    fn default() -> Self { Self::new() }
}

/// Cache service implementing the inbound port
pub struct CacheService {
    storage: Arc<dyn StoragePort>,
    metrics: Arc<AtomicMetrics>,
    policy: CachePolicy,
}

impl CacheService {
    pub fn new(storage: Arc<dyn StoragePort>, policy: CachePolicy) -> Self {
        Self {
            storage,
            metrics: Arc::new(AtomicMetrics::new()),
            policy,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(
            Arc::new(MemoryStorage::with_defaults()),
            CachePolicy::default(),
        )
    }

    pub fn metrics(&self) -> Arc<AtomicMetrics> {
        Arc::clone(&self.metrics)
    }
}

impl CachePort for CacheService {
    async fn get(&self, key: &str) -> CacheResult {
        match self.storage.retrieve(key) {
            Some(entry) if !entry.is_expired() => {
                self.metrics.record_hit();
                CacheResult::Hit(entry.value)
            }
            _ => {
                self.metrics.record_miss();
                CacheResult::Miss
            }
        }
    }

    async fn set(&self, key: String, value: Vec<u8>, ttl: Option<Duration>) -> Result<(), String> {
        let ttl = ttl.unwrap_or(self.policy.default_ttl);
        let entry = CacheEntry::new(key, value, Some(ttl));
        self.storage.store(entry)?;
        Ok(())
    }

    async fn contains(&self, key: &str) -> bool {
        self.storage.exists(key)
    }

    async fn delete(&self, key: &str) -> bool {
        self.storage.remove(key)
    }

    async fn clear(&self) {
        let _ = self.storage.clear();
    }

    async fn ttl(&self, key: &str) -> Option<Duration> {
        self.storage.retrieve(key).and_then(|e| e.ttl_remaining())
    }
}
