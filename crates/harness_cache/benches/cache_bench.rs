//! Benchmark tests for harness_cache

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use harness_cache::{AsyncCache, Cache};
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

fn bench_cache_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_throughput");
    for size in [100, 1000, 10000] {
        group.bench_with_input(BenchmarkId::new("set_get", size), &size, |b, &size| {
            b.iter_batched(
                || {
                    let cache = Cache::with_defaults();
                    let keys: Vec<String> = (0..size).map(|i| format!("key_{i}")).collect();
                    let values: Vec<Vec<u8>> = (0..size)
                        .map(|i| vec![(i % 256) as u8; 64])
                        .collect();
                    (cache, keys, values)
                },
                |(cache, keys, values)| {
                    for (k, v) in keys.iter().zip(values.iter()) {
                        cache.set(k.clone(), v.clone());
                    }
                    for k in &keys {
                        let _ = cache.get(k);
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_cache_eviction(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_eviction");
    for capacity in [100, 1000] {
        group.bench_with_input(BenchmarkId::new("evict_stress", capacity), &capacity, |b, &cap| {
            b.iter_batched(
                || Cache::with_defaults(),
                |cache| {
                    for i in 0..cap * 2 {
                        cache.set(format!("key_{i}"), vec![i as u8; 32]);
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_async_cache_get(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = rt.block_on(async {
        let cache = AsyncCache::with_defaults();
        cache.set("key".to_string(), vec![1, 2, 3]).await;
        cache
    });

    c.bench_function("async_cache_get", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _ = black_box(cache.get("key").await);
            });
        });
    });
}

fn bench_concurrent_access(c: &mut Criterion) {
    use std::sync::Arc;

    let cache = Arc::new(Cache::with_defaults());
    for i in 0..1000 {
        cache.set(format!("key_{i}"), vec![i as u8; 16]);
    }

    let mut group = c.benchmark_group("concurrent_access");
    for threads in [1, 2, 4, 8] {
        group.bench_with_input(BenchmarkId::new("read_parallel", threads), &threads, |b, &n| {
            b.iter(|| {
                let handles: Vec<_> = (0..n)
                    .map(|t| {
                        let cache = Arc::clone(&cache);
                        std::thread::spawn(move || {
                            let mut sum = 0u64;
                            for i in 0..1000 {
                                if cache.get(&format!("key_{}", (i + t * 100) % 1000)).is_some() {
                                    sum += 1;
                                }
                            }
                            sum
                        })
                    })
                    .collect();
                handles.into_iter().map(|h| h.join().unwrap()).sum::<u64>()
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_cache_get,
    bench_cache_set,
    bench_cache_throughput,
    bench_cache_eviction,
    bench_async_cache_get,
    bench_concurrent_access,
);
criterion_main!(benches);
