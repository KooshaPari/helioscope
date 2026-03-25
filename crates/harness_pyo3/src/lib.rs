//! Python FFI module using PyO3 0.28
//! High-performance functions callable from Python

use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::collections::HashMap;
use ffi_utils::FfiMutex;
use std::time::{Duration, Instant};

static CACHE: FfiMutex<Option<PyCache>> = FfiMutex::new(None);

struct PyCache {
    store: HashMap<String, CacheItem>,
    hits: u64,
    misses: u64,
}

struct CacheItem { value: String, expires_at: Option<Instant> }

impl PyCache {
    fn new() -> Self { Self { store: HashMap::new(), hits: 0, misses: 0 } }
    
    fn get(&mut self, key: &str) -> Option<String> {
        match self.store.get(key) {
            Some(item) => {
                if let Some(expires) = item.expires_at {
                    if expires < Instant::now() { self.store.remove(key); self.misses += 1; return None; }
                }
                self.hits += 1; Some(item.value.clone())
            }
            None => { self.misses += 1; None }
        }
    }
    
    fn set(&mut self, key: String, value: String, ttl_secs: Option<u64>) {
        let expires_at = ttl_secs.map(|s| Instant::now() + Duration::from_secs(s));
        self.store.insert(key, CacheItem { value, expires_at });
    }
    
    fn delete(&mut self, key: &str) -> bool { self.store.remove(key).is_some() }
    fn clear(&mut self) { self.store.clear(); }
    fn hit_rate(&self) -> f64 { let t = self.hits + self.misses; if t == 0 { 0.0 } else { self.hits as f64 / t as f64 } }
    fn stats(&self) -> (u64, u64, f64, usize) { (self.hits, self.misses, self.hit_rate(), self.store.len()) }
}

#[pyfunction]
fn init_cache() {
    let mut guard = CACHE.lock();
    *guard = Some(PyCache::new());
}

#[pyfunction]
fn cache_get(key: &str) -> Option<String> {
    let mut guard = CACHE.lock();
    guard.as_mut().and_then(|c| c.get(key))
}

#[pyfunction]
fn cache_set(key: String, value: String, ttl_secs: Option<u64>) {
    let mut guard = CACHE.lock();
    if let Some(c) = guard.as_mut() { c.set(key, value, ttl_secs); }
}

#[pyfunction]
fn cache_delete(key: &str) -> bool {
    let mut guard = CACHE.lock();
    guard.as_mut().map(|c| c.delete(key)).unwrap_or(false)
}

#[pyfunction]
fn cache_clear() {
    let mut guard = CACHE.lock();
    if let Some(c) = guard.as_mut() { c.clear(); }
}

#[pyfunction]
fn cache_stats() -> (u64, u64, f64, usize) {
    let guard = CACHE.lock();
    guard.as_ref().map(|c| c.stats()).unwrap_or((0, 0, 0.0, 0))
}

#[pyfunction]
fn fast_hash(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in s.bytes() { hash = hash.wrapping_mul(33).wrapping_add(byte as u64); }
    hash
}

#[pymodule]
#[pyo3(name = "harness_pyo3")]
fn harness_pyo3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_cache, m)?)?;
    m.add_function(wrap_pyfunction!(cache_get, m)?)?;
    m.add_function(wrap_pyfunction!(cache_set, m)?)?;
    m.add_function(wrap_pyfunction!(cache_delete, m)?)?;
    m.add_function(wrap_pyfunction!(cache_clear, m)?)?;
    m.add_function(wrap_pyfunction!(cache_stats, m)?)?;
    m.add_function(wrap_pyfunction!(fast_hash, m)?)?;
    Ok(())
}
