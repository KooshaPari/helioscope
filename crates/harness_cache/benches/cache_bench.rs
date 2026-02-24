//! Benchmark tests for harness_cache

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use harness_cache::Cache;

fn bench_cache_get(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("cache_get", |b| {
        b.to_async(&rt).iter(|| async {
            let cache = Cache::with_defaults();
            cache.set("key", vec![1, 2, 3]).await;
            cache.get("key").await
        });
    });
}

fn bench_cache_set(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let key = "benchmark_key";
    
    c.bench_function("cache_set", |b| {
        b.to_async(&rt).iter(|| async {
            let cache = Cache::with_defaults();
            cache.set(black_box(key), black_box(vec![1, 2, 3])).await;
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

criterion_group!(benches, bench_cache_get, bench_cache_set, bench_hash);
criterion_main!(benches);
