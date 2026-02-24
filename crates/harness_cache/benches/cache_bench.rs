//! Benchmark tests for harness_cache

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harness_cache::{Cache, MokaCache, CacheConfig};

fn bench_cache_get(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = Cache::with_defaults();
    rt.block_on(async {
        cache.set("key", vec![1, 2, 3]).await;
    });
    
    c.bench_function("cache_get", |b| {
        b.iter(|| {
            let _ = rt.block_on(async {
                cache.get("key").await
            });
        });
    });
}

fn bench_moka_cache(c: &mut Criterion) {
    // Benchmark Moka sync cache
    c.bench_function("moka_sync_get", |b| {
        let cache = MokaCache::with_lean_defaults();
        cache.set("key", vec![1, 2, 3]);
        
        b.iter(|| {
            cache.get(black_box("key"))
        });
    });
    
    c.bench_function("moka_sync_set", |b| {
        let cache = MokaCache::with_lean_defaults();
        
        b.iter(|| {
            cache.set(black_box("key"), black_box(vec![1, 2, 3]));
        });
    });
}

fn bench_cache_set(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let key = "benchmark_key";
    
    c.bench_function("cache_set", |b| {
        b.iter(|| {
            let _ = rt.block_on(async {
                let cache = Cache::with_defaults();
                cache.set(black_box(key), black_box(vec![1, 2, 3])).await;
            });
        });
    });
}

fn bench_hash(c: &mut Criterion) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    c.bench_function("fxhash", |b| {
        let data = "benchmark_string_for_hashing";
        b.iter(|| {
            let mut hasher = DefaultHasher::new();
            data.hash(&mut hasher);
            hasher.finish()
        });
    });
}

fn bench_lean_config(c: &mut Criterion) {
    c.bench_function("lean_cache_new", |b| {
        b.iter(|| {
            MokaCache::new(CacheConfig::lean())
        });
    });
}

criterion_group!(benches, 
    bench_cache_get, 
    bench_cache_set, 
    bench_moka_cache,
    bench_hash,
    bench_lean_config
);
criterion_main!(benches);
