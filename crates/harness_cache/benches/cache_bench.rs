//! Benchmark tests for harness_cache

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harness_cache::{Cache, CacheConfig};

fn bench_cache_get(c: &mut Criterion) {
    let cache = Cache::with_defaults();
    cache.set("key".to_string(), vec![1, 2, 3]);

    c.bench_function("cache_get", |b| {
        b.iter(|| cache.get(black_box("key")));
    });
}

fn bench_cache_set(c: &mut Criterion) {
    let key = "benchmark_key";

    c.bench_function("cache_set", |b| {
        b.iter(|| {
            let cache = Cache::with_defaults();
            cache.set(black_box(key.to_string()), black_box(vec![1, 2, 3]));
        });
    });
}

fn bench_cache_with_config(c: &mut Criterion) {
    c.bench_function("cache_new_default_config", |b| {
        b.iter(|| Cache::new(&CacheConfig::default()));
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

criterion_group!(
    benches,
    bench_cache_get,
    bench_cache_set,
    bench_cache_with_config,
    bench_hash
);
criterion_main!(benches);
