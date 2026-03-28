//! Benchmark tests for harness_cache

use criterion::{criterion_group, criterion_main, Criterion};
use harness_cache::Cache;
use std::hint::black_box;

fn bench_cache_get(c: &mut Criterion) {
    let cache = Cache::with_defaults();
    cache.set("key".to_string(), vec![1, 2, 3]);

    c.bench_function("cache_get", |b| {
        b.iter(|| {
            let _ = black_box(cache.get("key"));
        });
    });
}

fn bench_cache_set(c: &mut Criterion) {
    let cache = Cache::with_defaults();
    c.bench_function("cache_set", |b| {
        b.iter(|| {
            cache.set(black_box("key".to_string()), black_box(vec![1, 2, 3]));
        });
    });
}

criterion_group!(benches, bench_cache_get, bench_cache_set);
criterion_main!(benches);
