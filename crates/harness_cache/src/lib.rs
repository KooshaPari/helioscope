//! Cache module - High-performance in-memory cache using Moka
//!
//! Provides two cache implementations:
//! - `Cache`: Simple sync cache backed by Moka's synchronous cache
//! - `AsyncCache`: Async-aware cache backed by Moka's future cache
//!
//! Both support TTL, size-based eviction, and atomic metrics tracking.

pub mod adapters;
pub mod domain;
pub mod ports;

use std::time::Duration;
use thiserror::Error;

/// Cache errors
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Key not found: {0}")]
    NotFound(String),
    #[error("Cache is full")]
    Full,
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_capacity: u64,
    pub ttl_secs: u64,
    pub name: String,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_capacity: 10_000,
            ttl_secs: 300,
            name: "default".to_string(),
        }
    }
}

/// Synchronous in-memory cache backed by Moka
pub struct Cache {
    inner: moka::sync::Cache<String, bytes::Bytes>,
    ttl: Duration,
}

/// Async-aware in-memory cache backed by Moka's future cache
pub struct AsyncCache {
    inner: moka::future::Cache<String, bytes::Bytes>,
    ttl: Duration,
}

impl Cache {
    pub fn new(config: &CacheConfig) -> Self {
        let inner = moka::sync::Cache::builder()
            .max_capacity(config.max_capacity)
            .time_to_live(Duration::from_secs(config.ttl_secs))
            .build();
        Self {
            inner,
            ttl: Duration::from_secs(config.ttl_secs),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(&CacheConfig::default())
    }

    pub fn with_ttl(ttl: Duration) -> Self {
        let inner = moka::sync::Cache::builder()
            .max_capacity(10_000)
            .time_to_live(ttl)
            .build();
        Self { inner, ttl }
    }

    pub fn get(&self, key: &str) -> Option<bytes::Bytes> {
        self.inner.get(key)
    }

    pub fn get_bytes(&self, key: &str) -> Option<Vec<u8>> {
        self.get(key).map(|b| b.to_vec())
    }

    pub fn get_str(&self, key: &str) -> Option<String> {
        self.get(key).and_then(|b| String::from_utf8(b.to_vec()).ok())
    }

    pub fn set(&self, key: String, value: Vec<u8>) {
        self.inner.insert(key, bytes::Bytes::from(value));
    }

    pub fn set_bytes(&self, key: String, value: bytes::Bytes) {
        self.inner.insert(key, value);
    }

    pub fn remove(&self, key: &str) -> Option<bytes::Bytes> {
        let prev = self.inner.get(key);
        self.inner.invalidate(key);
        prev
    }

    pub fn contains(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    pub fn clear(&self) {
        self.inner.invalidate_all();
    }

    pub fn len(&self) -> u64 {
        self.inner.entry_count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn hit_rate(&self) -> f64 {
        self.inner.hit_rate()
    }

    pub fn run_pending_tasks(&self) {
        self.inner.run_pending_tasks();
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl AsyncCache {
    pub fn new(config: &CacheConfig) -> Self {
        let inner = moka::future::Cache::builder()
            .max_capacity(config.max_capacity)
            .time_to_live(Duration::from_secs(config.ttl_secs))
            .build();
        Self {
            inner,
            ttl: Duration::from_secs(config.ttl_secs),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(&CacheConfig::default())
    }

    pub fn with_ttl(ttl: Duration) -> Self {
        let inner = moka::future::Cache::builder()
            .max_capacity(10_000)
            .time_to_live(ttl)
            .build();
        Self { inner, ttl }
    }

    pub async fn get(&self, key: &str) -> Option<bytes::Bytes> {
        self.inner.get(key).await
    }

    pub async fn get_bytes(&self, key: &str) -> Option<Vec<u8>> {
        self.get(key).await.map(|b| b.to_vec())
    }

    pub async fn get_str(&self, key: &str) -> Option<String> {
        self.get(key).await.and_then(|b| String::from_utf8(b.to_vec()).ok())
    }

    pub async fn set(&self, key: String, value: Vec<u8>) {
        self.inner.insert(key, bytes::Bytes::from(value)).await;
    }

    pub async fn set_bytes(&self, key: String, value: bytes::Bytes) {
        self.inner.insert(key, value).await;
    }

    pub async fn remove(&self, key: &str) -> Option<bytes::Bytes> {
        let prev = self.inner.get(key).await;
        self.inner.invalidate(key).await;
        prev
    }

    pub async fn contains(&self, key: &str) -> bool {
        self.inner.contains_key(key).await
    }

    pub async fn clear(&self) {
        self.inner.invalidate_all().await;
    }

    pub fn len(&self) -> u64 {
        self.inner.entry_count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn hit_rate(&self) -> f64 {
        self.inner.hit_rate()
    }
}

impl Default for AsyncCache {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_set_get() {
        let cache = Cache::with_defaults();
        cache.set("key1".to_string(), b"value1".to_vec());
        assert!(cache.get("key1").is_some());
    }

    #[test]
    fn test_cache_miss() {
        let cache = Cache::with_defaults();
        assert!(cache.get("nonexistent").is_none());
    }

    #[test]
    fn test_cache_remove() {
        let cache = Cache::with_defaults();
        cache.set("key1".to_string(), b"value1".to_vec());
        let removed = cache.remove("key1");
        assert!(removed.is_some());
        assert!(cache.get("key1").is_none());
    }

    #[test]
    fn test_cache_str() {
        let cache = Cache::with_defaults();
        cache.set("key1".to_string(), b"value1".to_vec());
        assert_eq!(cache.get_str("key1"), Some("value1".to_string()));
    }

    #[test]
    fn test_cache_clear() {
        let cache = Cache::with_defaults();
        cache.set("key1".to_string(), b"value1".to_vec());
        cache.set("key2".to_string(), b"value2".to_vec());
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_contains() {
        let cache = Cache::with_defaults();
        cache.set("key1".to_string(), b"value1".to_vec());
        assert!(cache.contains("key1"));
        assert!(!cache.contains("key2"));
    }

    #[test]
    fn test_cache_bytes_zero_copy() {
        let cache = Cache::with_defaults();
        let data = bytes::Bytes::from(vec![1, 2, 3, 4, 5]);
        cache.set_bytes("binary".to_string(), data.clone());
        let retrieved = cache.get("binary").unwrap();
        assert_eq!(retrieved.as_ref(), &[1, 2, 3, 4, 5]);
    }
}
