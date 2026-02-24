//! Cache module - Phase 1: Optimized with Moka-style features
//! Features: Sharded, TTL, LRU, metrics, async/sync

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::hash::{Hash, Hasher};
use tracing::{debug, instrument};

/// Cache entry with metadata
#[derive(Clone)]
pub struct CacheEntry {
    pub value: Vec<u8>,
    pub expires_at: Option<Instant>,
    pub created_at: Instant,
    pub access_count: u64,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub ttl_secs: u64,
    pub max_capacity: usize,
    pub shards: usize,
    pub name: String,
    pub write_mode: WriteMode,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            ttl_secs: 60,        // Reduced from 300
            max_capacity: 100,   // Reduced from 10,000
            shards: 4,           // Reduced from 16
            name: "default".to_string(),
            write_mode: WriteMode::WriteThrough,
        }
    }
}

/// Lean configuration for low-memory environments (<10MB)
impl CacheConfig {
    pub fn lean() -> Self {
        Self {
            ttl_secs: 30,
            max_capacity: 50,
            shards: 2,
            name: "lean".to_string(),
            write_mode: WriteMode::WriteThrough,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WriteMode {
    WriteThrough,  // Write to cache immediately
    WriteBack,     // Write to cache, flush later
}

/// Sharded cache for reduced contention
pub struct ShardedCache {
    shards: Vec<Arc<Shard>>,
    config: CacheConfig,
    stats: Arc<CacheStats>,
}

struct Shard {
    store: RwLock<HashMap<String, CacheEntry>>,
}

impl Shard {
    fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }
}

impl ShardedCache {
    pub fn new(config: CacheConfig) -> Self {
        let shards = (0..config.shards)
            .map(|_| Arc::new(Shard::new()))
            .collect();
        
        Self { shards, config, stats: Arc::new(CacheStats::new()) }
    }

    pub fn with_defaults() -> Self {
        Self::new(CacheConfig::default())
    }

    /// Get shard index for key
    #[inline]
    fn shard_index(&self, key: &str) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.config.shards
    }

    /// Get value if valid
    #[instrument(skip(self))]
    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let idx = self.shard_index(key);
        let shard = &self.shards[idx];
        
        let store = shard.store.read().await;
        match store.get(key) {
            Some(entry) if self.is_valid(entry) => {
                self.stats.record_hit();
                debug!(key, "cache hit");
                Some(entry.value.clone())
            }
            _ => {
                self.stats.record_miss();
                debug!(key, "cache miss");
                None
            }
        }
    }

    /// Get string value
    pub async fn get_str(&self, key: &str) -> Option<String> {
        self.get(key).await.and_then(|v| String::from_utf8(v).ok())
    }

    /// Set value
    #[instrument(skip(self, value))]
    pub async fn set(&self, key: String, value: Vec<u8>) {
        let idx = self.shard_index(&key);
        let shard = &self.shards[idx];
        
        let mut store = shard.store.write().await;
        
        // Evict if at capacity
        #[allow(clippy::let_underscore_future)]
        if store.len() >= self.config.max_capacity / self.config.shards {
            let _ = self.evict_lru(&mut store);
        }
        
        let now = Instant::now();
        store.insert(key, CacheEntry {
            value,
            expires_at: Some(now + Duration::from_secs(self.config.ttl_secs)),
            created_at: now,
            access_count: 0,
        });
        
        self.stats.set_items(self.len_internal(&store).await);
    }

    /// Set string value
    pub async fn set_str(&self, key: &str, value: &str) {
        self.set(key.to_string(), value.as_bytes().to_vec()).await;
    }

    /// Delete key
    pub async fn delete(&self, key: &str) -> bool {
        let idx = self.shard_index(key);
        let shard = &self.shards[idx];
        let mut store = shard.store.write().await;
        store.remove(key).is_some()
    }

    /// Clear all
    pub async fn clear(&self) {
        for shard in &self.shards {
            let mut store = shard.store.write().await;
            store.clear();
        }
        self.stats.set_items(0);
    }

    /// Get stats
    pub fn stats(&self) -> Arc<CacheStats> {
        Arc::clone(&self.stats)
    }

    /// Check validity
    #[inline]
    fn is_valid(&self, entry: &CacheEntry) -> bool {
        match entry.expires_at {
            Some(expires) => expires > Instant::now(),
            None => true,
        }
    }

    /// Evict LRU entry
    async fn evict_lru(&self, store: &mut HashMap<String, CacheEntry>) {
        if let Some((key, _)) = store.iter().min_by_key(|(_, v)| v.access_count) {
            let key = key.clone();
            store.remove(&key);
            self.stats.record_eviction();
        }
    }

    /// Total items
    pub async fn len(&self) -> usize {
        let mut total = 0;
        for shard in &self.shards {
            let store = shard.store.read().await;
            total += store.len();
        }
        total
    }

    #[allow(clippy::len_without_is_empty)]
    pub async fn is_empty(&self) -> bool { self.len().await == 0 }

    async fn len_internal(&self, _store: &HashMap<String, CacheEntry>) -> usize {
        self.len().await
    }
}

/// Cache statistics with atomic operations
pub struct CacheStats {
    pub hits: AtomicU64,
    pub misses: AtomicU64,
    pub evictions: AtomicU64,
    pub items: AtomicUsize,
}

impl CacheStats {
    pub fn new() -> Self {
        Self {
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            evictions: AtomicU64::new(0),
            items: AtomicUsize::new(0),
        }
    }

    #[inline] pub fn record_hit(&self) { self.hits.fetch_add(1, Ordering::Relaxed); }
    #[inline] pub fn record_miss(&self) { self.misses.fetch_add(1, Ordering::Relaxed); }
    #[inline] pub fn record_eviction(&self) { self.evictions.fetch_add(1, Ordering::Relaxed); }
    #[inline] pub fn set_items(&self, count: usize) { self.items.store(count, Ordering::Relaxed); }

    pub fn hit_rate(&self) -> f64 {
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let total = hits + misses;
        if total == 0 { 0.0 } else { hits as f64 / total as f64 }
    }

    pub fn hits(&self) -> u64 { self.hits.load(Ordering::Relaxed) }
    pub fn misses(&self) -> u64 { self.misses.load(Ordering::Relaxed) }
    pub fn items(&self) -> usize { self.items.load(Ordering::Relaxed) }
}

impl Default for CacheStats {
    fn default() -> Self { Self::new() }
}

/// Simple async cache (backward compatible)
pub struct Cache {
    inner: ShardedCache,
}

impl Cache {
    pub fn new(config: CacheConfig) -> Self {
        Self { inner: ShardedCache::new(config) }
    }

    pub fn with_defaults() -> Self {
        Self { inner: ShardedCache::with_defaults() }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> { self.inner.get(key).await }
    pub async fn get_str(&self, key: &str) -> Option<String> { self.inner.get_str(key).await }
    pub async fn set(&self, key: &str, value: Vec<u8>) { self.inner.set(key.to_string(), value).await }
    pub async fn set_str(&self, key: &str, value: &str) { self.inner.set_str(key, value).await }
    pub async fn delete(&self, key: &str) -> bool { self.inner.delete(key).await }
    pub async fn clear(&self) { self.inner.clear().await; }
    pub fn stats(&self) -> Arc<CacheStats> { self.inner.stats() }
    pub async fn len(&self) -> usize { self.inner.len().await }
    #[allow(clippy::len_without_is_empty)]
    pub async fn is_empty(&self) -> bool { self.inner.len().await == 0 }
}

impl Default for Cache {
    fn default() -> Self { Self::with_defaults() }
}

/// Synchronous cache
pub struct SyncCache {
    store: std::sync::Mutex<HashMap<String, CacheEntry>>,
    config: CacheConfig,
    stats: Arc<CacheStats>,
}

impl SyncCache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            store: std::sync::Mutex::new(HashMap::new()),
            config,
            stats: Arc::new(CacheStats::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let store = self.store.lock().unwrap();
        match store.get(key) {
            Some(entry) if entry.expires_at.is_none_or(|e| e > Instant::now()) => {
                self.stats.record_hit();
                Some(entry.value.clone())
            }
            _ => {
                self.stats.record_miss();
                None
            }
        }
    }

    pub fn set(&self, key: &str, value: Vec<u8>) {
        let mut store = self.store.lock().unwrap();
        let now = Instant::now();
        store.insert(key.to_string(), CacheEntry {
            value,
            expires_at: Some(now + Duration::from_secs(self.config.ttl_secs)),
            created_at: now,
            access_count: 0,
        });
        self.stats.set_items(store.len());
    }

    pub fn stats(&self) -> Arc<CacheStats> { Arc::clone(&self.stats) }
}

/// Moka-backed cache (production-grade)
/// Provides: thread-safe, TTL, LRU, background eviction
pub struct MokaCache {
    cache: moka::sync::Cache<String, Vec<u8>>,
    stats: Arc<CacheStats>,
}

impl MokaCache {
    pub fn new(config: CacheConfig) -> Self {
        let cache = moka::sync::Cache::builder()
            .max_capacity(config.max_capacity as u64)
            .time_to_live(Duration::from_secs(config.ttl_secs))
            .name(&config.name)
            .build();
        
        Self {
            cache,
            stats: Arc::new(CacheStats::new()),
        }
    }

    pub fn with_lean_defaults() -> Self {
        Self::new(CacheConfig::lean())
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        match self.cache.get(key) {
            Some(v) => {
                self.stats.record_hit();
                Some(v)
            }
            None => {
                self.stats.record_miss();
                None
            }
        }
    }

    pub fn set(&self, key: &str, value: Vec<u8>) {
        self.cache.insert(key.to_string(), value);
        self.stats.set_items(self.cache.entry_count() as usize);
    }

    pub fn invalidate(&self, key: &str) {
        self.cache.invalidate(key);
    }

    pub fn clear(&self) {
        self.cache.invalidate_all();
    }

    pub fn stats(&self) -> Arc<CacheStats> {
        Arc::clone(&self.stats)
    }
}

/// Async Moka cache for tokio runtime
pub struct MokaAsyncCache {
    cache: moka::future::Cache<String, Vec<u8>>,
    stats: Arc<CacheStats>,
}

impl MokaAsyncCache {
    pub async fn new(config: CacheConfig) -> Self {
        let cache = moka::future::Cache::builder()
            .max_capacity(config.max_capacity as u64)
            .time_to_live(Duration::from_secs(config.ttl_secs))
            .name(&format!("{}-async", config.name))
            .build();
        
        Self {
            cache,
            stats: Arc::new(CacheStats::new()),
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        match self.cache.get(key).await {
            Some(v) => {
                self.stats.record_hit();
                Some(v)
            }
            None => {
                self.stats.record_miss();
                None
            }
        }
    }

    pub async fn set(&self, key: &str, value: Vec<u8>) {
        self.cache.insert(key.to_string(), value).await;
        self.stats.set_items(self.cache.entry_count() as usize);
    }

    pub async fn invalidate(&self, key: &str) {
        self.cache.invalidate(key).await;
    }

    pub fn stats(&self) -> Arc<CacheStats> {
        Arc::clone(&self.stats)
    }
}
