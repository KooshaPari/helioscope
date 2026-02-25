# heliosCLI

**High-performance multi-agent orchestration CLI** - Built on codex with enhancements.

## Architecture

```
heliosCLI/
├── rust_core/     # Codex fork (90% Rust) - Primary CLI
├── crates/        # thegent extensions (Rust)
├── transport/     # cliproxy++ (Go - HTTP/WS/Unix/gRPC)
└── python/       # heliosBench (Python - optimized runtimes)
```

## Components

### Rust (Primary)
- **rust_core/cli** - Main CLI (codex-rs fork)
- **rust_core/core** - Core agent logic
- **rust_core/exec** - Execution engine
- **rust_core/tui** - Terminal UI
- **crates/** - thegent extensions

### Python (Optimized)
- **heliosBench** - Benchmarking framework
- Run with: `pypy3.11` or `cpython3.14` (GIL-free)

### Go (Transport)
- **cliproxy++** - High-performance proxy
- HTTP/2, WebSocket, Unix Socket, gRPC

## Building

```bash
# Build Rust CLI
cd heliosHarness/heliosCLI
cargo build --release

# Run benchmarks (Python)
cd python
pypy3.11 -m heliosBench run --profile latency
```

## Features
- Unified execution
- Multi-agent orchestration
- Connection pooling (100 connections)
- Multi-transport (unix > ws > http)
- Shell snapshots
- Memory tool

---

## Phase 2: Performance Optimization

### Zig Integration
- Native extensions for hot paths: JSON parsing, token counting
- Zero-cost abstractions, no runtime
- Build as `.dylib`/`.so` from Rust

### Mojo Integration  
- High-performance compute kernels
- SIMD support for batch processing
- Python-compatible (can call from Python)

### Inline Assembly
- Rust `asm!` macro for critical loops
- Token parsing, hashing, encryption

### CPython 3.14 / PyPy 3.11
- GIL-free mode for Python components
- JIT compilation via PyPy

## Component Language Matrix

| Component | Current | Target |
|-----------|---------|--------|
| CLI Core | Rust | Rust |
| Extensions | Rust | Rust + Zig |
| Compute | Python | Mojo |
| Transport | Go | Go |
| Benchmarks | Python | PyPy 3.11 / CPython 3.14 |
| Hot Paths | - | Inline Assembly |

---

## Current Components

| Component | Language | Location |
|-----------|----------|-----------|
| CLI Core | Rust | rust_core/cli |
| Core | Rust | rust_core/core |
| Extensions | Rust | crates/thegent-* |
| Native | Rust | crates/harness-native |
| Zig | Zig | crates/harness_zig |
| Transport | Go | transport/ |
| Benchmarks | Python | python/ |
