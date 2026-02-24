# heliosHarness

High-performance harness system built with Rust, following hexagonal architecture principles.

## Crates

| Crate | Purpose | Status |
|-------|---------|--------|
| `harness_cache` | Sharded TTL cache with LRU eviction | ✅ |
| `harness_runner` | Process execution with timeout | ✅ |
| `harness_scaling` | Auto-scaling with predictive algorithms | ✅ |
| `harness_schema` | Schema validation | ✅ |
| `harness_discoverer` | Service discovery | ✅ |
| `harness_utils` | Common utilities | ✅ |

## Architecture

```
src/
├── domain/          # Core business logic (framework agnostic)
├── ports/          # Interface definitions (traits)
├── adapters/       # Concrete implementations
└── lib.rs          # Main entry point
```

## Features

- **Sharded Cache**: Reduced lock contention for concurrent access
- **TTL + LRU**: Time-based expiration with least-recently-used eviction
- **Predictive Scaling**: Linear regression for resource prediction
- **Circuit Breaker**: Fault tolerance with half-open state
- **Token Bucket**: Rate limiting
- **Service Discovery**: Dynamic service registration

## Usage

```rust
use harness_cache::{Cache, CacheConfig};

let config = CacheConfig::default();
let cache = Cache::new(config);

cache.set("key", "value").await;
let value = cache.get("key").await;
```

## Development

```bash
# Build
cargo build

# Test
cargo test

# Format
cargo fmt

# Clippy
cargo clippy -- -D warnings
```

## Benchmarking

```bash
cargo bench
```

## License

MIT
