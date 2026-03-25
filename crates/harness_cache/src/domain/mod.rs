//! Domain entities for cache
//! Core business logic - framework agnostic

use std::time::{Duration, Instant};

/// Cache entry domain entity
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub key: String,
    pub value: Vec<u8>,
    pub created_at: Instant,
    pub expires_at: Option<Instant>,
    pub metadata: EntryMetadata,
}

#[derive(Debug, Clone, Default)]
pub struct EntryMetadata {
    pub tags: Vec<String>,
    pub priority: Priority,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for Priority {
    fn default() -> Self { Priority::Normal }
}

impl CacheEntry {
    pub fn new(key: String, value: Vec<u8>, ttl: Option<Duration>) -> Self {
        let now = Instant::now();
        Self {
            key,
            value,
            created_at: now,
            expires_at: ttl.map(|d| now + d),
            metadata: EntryMetadata::default(),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(false, |e| e < Instant::now())
    }

    pub fn ttl_remaining(&self) -> Option<Duration> {
        self.expires_at.map(|e| e.duration_since(Instant::now()))
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.metadata.priority = priority;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.metadata.tags = tags;
        self
    }
}

/// Cache configuration domain object
#[derive(Debug, Clone)]
pub struct CachePolicy {
    pub max_size: usize,
    pub default_ttl: Duration,
    pub eviction_policy: EvictionPolicy,
    pub write_mode: WritePolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionPolicy {
    LRU,      // Least Recently Used
    LFU,      // Least Frequently Used
    FIFO,     // First In First Out
    TTL,      // Time To Live
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WritePolicy {
    WriteThrough,
    WriteBack,
}

impl Default for CachePolicy {
    fn default() -> Self {
        Self {
            max_size: 10_000,
            default_ttl: Duration::from_secs(300),
            eviction_policy: EvictionPolicy::LRU,
            write_mode: WritePolicy::WriteThrough,
        }
    }
}

/// Cache operation result
#[derive(Debug, Clone)]
pub enum CacheResult {
    Hit(Vec<u8>),
    Miss,
    Error(String),
}

impl CacheResult {
    pub fn is_hit(&self) -> bool { matches!(self, CacheResult::Hit(_)) }
    pub fn value(self) -> Option<Vec<u8>> { match self { CacheResult::Hit(v) => Some(v), _ => None } }
}
