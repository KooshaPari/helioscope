# ADR-004: Async Runtime Selection

**Date**: 2026-04-02  
**Status**: Accepted  
**Deciders**: Agent  

## Context

heliosCLI requires an async runtime for concurrent operations across multiple domains:

- **TUI Operations**: Real-time event handling, keyboard input, and UI refresh cycles
- **Backend Operations**: Concurrent Docker API calls, Kubernetes watch streams, and health checks
- **Log Streaming**: Multiple concurrent log tailing operations from different containers
- **Deployment Pipelines**: Parallel deployment stages across different backends
- **WebSocket Communication**: Async message handling for the app-server protocol
- **Background Tasks**: Checkpoint saving, metrics collection, and heartbeat operations

The async runtime choice is foundational as it affects:
- Performance characteristics (latency, throughput)
- Ecosystem compatibility (libraries, middleware)
- Developer experience (debugging, profiling)
- Binary size and compile times
- Future maintainability

## Decision Drivers

- **Ecosystem compatibility**: Integration with ratatui, tokio-tungstenite, and other dependencies
- **Performance characteristics**: Task scheduling, I/O efficiency, memory usage
- **Library availability**: HTTP clients, database drivers, WebSocket libraries
- **Documentation quality**: Learning resources, API stability
- **Long-term viability**: Community support, corporate backing
- **Debugging capabilities**: Tracing, profiling, task inspection
- **Compile time and binary size**: Build performance and distribution footprint

## Options Considered

### Option A: tokio

**GitHub**: [tokio-rs/tokio](https://github.com/tokio-rs/tokio)  
**Stars**: 27k+ | **Downloads**: 100M+ | **Maintainer**: AWS/Amazon

**Key Facts**:
- De facto standard for Rust async (90%+ market share in production Rust services)
- Work-stealing scheduler optimized for network I/O
- Rich ecosystem of compatible libraries (hyper, tonic, sqlx, etc.)
- Native ratatui integration through `tokio::select!` patterns
- Production-grade with extensive battle-testing

**Architecture**:
```
┌─────────────────────────────────────────────────────────────────────────┐
│                         tokio Runtime Architecture                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌─────────────────────────────────────────────────────────────────────┐│
│  │                    Multi-Threaded Scheduler                          ││
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐        ┌─────────┐              ││
│  │  │ Worker  │ │ Worker  │ │ Worker  │  ...   │ Worker  │              ││
│  │  │ Thread 1│ │ Thread 2│ │ Thread 3│        │ Thread N│              ││
│  │  │(Core 0) │ │(Core 1) │ │(Core 2) │        │(Core N) │              ││
│  │  └────┬────┘ └────┬────┘ └────┬────┘        └────┬────┘              ││
│  │       │            │            │                │                   ││
│  │       └────────────┴────────────┴────────────────┘                   ││
│  │                    Global Task Queue                                  ││
│  │  • Work-stealing for load balancing                                   ││
│  │  • NUMA-aware thread pinning                                          ││
│  │  • Core affinity optimization                                          ││
│  └─────────────────────────────────────────────────────────────────────┘│
│                                    │                                     │
│                                    ▼                                     │
│  ┌─────────────────────────────────────────────────────────────────────┐│
│  │                     I/O Driver (epoll/kqueue/IOCP)                   ││
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐    ││
│  │  │   epoll    │  │   kqueue   │  │   IOCP     │  │ io_uring   │    ││
│  │  │  (Linux)   │  │  (macOS)   │  │ (Windows)  │  │ (Linux 5.1+│    ││
│  │  └────────────┘  └────────────┘  └────────────┘  └────────────┘    ││
│  │  Zero-cost async I/O with platform-native backends                    ││
│  └─────────────────────────────────────────────────────────────────────┘│
│                                    │                                     │
│                                    ▼                                     │
│  ┌─────────────────────────────────────────────────────────────────────┐│
│  │                      Timer & Delay Queue                               ││
│  │  • Efficient hierarchical timing wheel                                ││
│  │  • Sleep, timeout, interval support                                   ││
│  │  • Resolution: 1ms (default)                                          ││
│  └─────────────────────────────────────────────────────────────────────┘│
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Performance Characteristics**:
| Metric | Value | Notes |
|--------|-------|-------|
| Task spawn latency | ~100ns | Ultra-fast task creation |
| Context switch | ~5ns | Work-stealing efficiency |
| TCP accept rate | 1M+ / sec | High-throughput servers |
| Memory per task | ~0.5KB | Minimal memory overhead |
| Thread pool scaling | Automatic | Based on CPU cores |
| Max concurrent I/O | 1M+ handles | File descriptors |

**Pros**:
- **Ecosystem dominance**: 90%+ of production Rust async code uses tokio
- **ratatui integration**: Established patterns for async TUI applications
- **Library compatibility**: Virtually all async libraries target tokio
- **Production proven**: AWS, Discord, Fly.io, and thousands of services
- **Comprehensive runtime**: Channels, timers, synchronization primitives
- **Observability**: Built-in tracing integration, task dumps, metrics
- **Performance**: Optimized work-stealing scheduler, io_uring support
- **Documentation**: Extensive guides, examples, and API docs

**Cons**:
- **Binary size**: ~300KB+ overhead for full runtime
- **Compile time**: Slower than minimal runtimes due to complexity
- **Learning curve**: Many concepts (tasks, runtimes, handles) to learn
- **Blocking task gotchas**: Must use `spawn_blocking` for CPU-bound work

### Option B: async-std

**GitHub**: [async-rs/async-std](https://github.com/async-rs/async-std)  
**Stars**: 4k+ | **Downloads**: 10M+

**Key Facts**:
- Standard library API parity for async operations
- Single-threaded and multi-threaded runtime options
- Smaller ecosystem than tokio

**Architecture**:
```
┌─────────────────────────────────────────────────────────────────┐
│                    async-std Architecture                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │              smol-based Executor                             ││
│  │                                                              ││
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐                ││
│  │  │  Task 1  │  │  Task 2  │  │  Task 3  │  ...             ││
│  │  │ (async)  │  │ (async)  │  │ (async)  │                  ││
│  │  └──────────┘  └──────────┘  └──────────┘                  ││
│  │        │            │            │                         ││
│  │        └────────────┼────────────┘                         ││
│  │                     ▼                                        ││
│  │           ┌───────────────┐                               ││
│  │           │   ThreadPool  │  (optional multi-thread)      ││
│  │           │   (async-io)   │                               ││
│  │           └───────────────┘                               ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**Pros**:
- **API familiarity**: Mirrors std library interfaces
- **Smaller than tokio**: Less complex runtime
- **Good performance**: Competitive for many use cases
- **Async ecosystem**: Growing set of compatible libraries

**Cons**:
- **Smaller ecosystem**: Fewer third-party libraries support it
- **Limited ratatui support**: Requires compatibility shims
- **Maintenance concerns**: Smaller team than tokio
- **Fragmentation**: Forces ecosystem split

**Verdict**: Good alternative but ecosystem fragmentation is a concern

### Option C: smol

**GitHub**: [smol-rs/smol](https://github.com/smol-rs/smol)  
**Stars**: 3k+ | **Downloads**: 5M+

**Key Facts**:
- Small, fast runtime built on async-io and async-executor
- Minimal dependencies and binary size
- Composable with other async runtimes

**Architecture**:
```
┌─────────────────────────────────────────────────────────────────┐
│                      smol Architecture                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │               smol - Small & Fast Runtime                    ││
│  │                                                              ││
│  │   ┌─────────┐   ┌─────────┐   ┌─────────┐                   ││
│  │   │ async-io│   │ polling │   │futures-lite│                 ││
│  │   │(I/O driver)│ │ (epoll) │   │(minimal    │                ││
│  │   │          │   │         │   │ futures)   │                 ││
│  │   └────┬────┘   └────┬────┘   └────┬────┘                   ││
│  │        │             │             │                        ││
│  │        └─────────────┼─────────────┘                        ││
│  │                       ▼                                      ││
│  │           ┌───────────────────┐                             ││
│  │           │  async-executor   │                             ││
│  │           │  (task scheduler) │                             ││
│  │           └───────────────────┘                             ││
│  │                                                              ││
│  │  Binary size: ~50KB  |  Compile: Fast  |  Runtime: Efficient ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**Performance Characteristics**:
| Metric | Value | Comparison to tokio |
|--------|-------|---------------------|
| Binary overhead | ~50KB | 6x smaller than tokio |
| Compile time | Fast | 3x faster than tokio |
| Task latency | ~200ns | 2x slower than tokio |
| Throughput | High | Comparable for I/O |

**Pros**:
- **Minimal size**: Very small binary footprint
- **Fast compile**: Quick build times
- **Composable**: Can work alongside tokio if needed
- **Efficient**: Good performance for I/O-bound work

**Cons**:
- **Limited ecosystem**: Very few libraries target smol directly
- **Manual integration**: Requires more glue code
- **No native ratatui**: Requires compatibility layers
- **Smaller community**: Harder to find help and examples

**Verdict**: Excellent for embedded/resource-constrained, insufficient for heliosCLI's ecosystem needs

## Decision

**Adopt tokio as the async runtime for heliosCLI**.

### Rationale

| Factor | tokio | async-std | smol | Winner |
|--------|-------|-----------|------|--------|
| ratatui integration | ✅ Native | ⚠️ Shim | ❌ Manual | tokio |
| Ecosystem size | 90%+ | 5% | 2% | tokio |
| Library support | Complete | Good | Limited | tokio |
| Production usage | Extensive | Moderate | Minimal | tokio |
| Documentation | Excellent | Good | Limited | tokio |
| Performance | Excellent | Good | Excellent | Tie |
| Binary size | ~300KB | ~200KB | ~50KB | smol |
| Compile time | Slow | Medium | Fast | smol |

**Decision drivers**:
1. **ratatui ecosystem**: All ratatui async examples use tokio patterns
2. **Dependency alignment**: hyper, tonic, sqlx, reqwest all default to tokio
3. **Team expertise**: Most Rust developers know tokio
4. **Future-proofing**: Clear ecosystem winner, stable long-term
5. **Production support**: AWS backing ensures continued investment

### Implementation Example

**Entry point with tokio::main**:
```rust
use tokio::signal;
use tracing::{info, error};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing for structured logging
    tracing_subscriber::fmt::init();
    
    info!("Starting heliosCLI runtime");
    
    // Parse CLI arguments
    let args = Cli::parse();
    
    // Initialize configuration
    let config = Config::load(args.config_path).await?;
    
    // Spawn the main application task
    let app_handle = tokio::spawn(run_app(args, config));
    
    // Spawn background tasks
    let metrics_handle = tokio::spawn(metrics_collector());
    let checkpoint_handle = tokio::spawn(checkpoint_worker());
    
    // Wait for shutdown signal
    tokio::select! {
        result = app_handle => {
            info!("Application task completed");
            result??;
        }
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
    }
    
    // Graceful shutdown
    metrics_handle.abort();
    checkpoint_handle.abort();
    
    info!("heliosCLI shutdown complete");
    Ok(())
}
```

**TUI integration with tokio**:
```rust
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use tokio::sync::{mpsc, RwLock};
use tokio::time::{interval, Duration};
use std::sync::Arc;

/// Application state shared between async tasks
pub struct AppState {
    apps: RwLock<Vec<Application>>,
    logs: RwLock<Vec<LogEntry>>,
    should_quit: RwLock<bool>,
}

/// Run the async TUI main loop
pub async fn run_tui_app() -> anyhow::Result<()> {
    // Setup terminal
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;
    
    // Shared state
    let state = Arc::new(AppState {
        apps: RwLock::new(Vec::new()),
        logs: RwLock::new(Vec::new()),
        should_quit: RwLock::new(false),
    });
    
    // Channels for cross-task communication
    let (ui_tx, mut ui_rx) = mpsc::channel::<UiEvent>(100);
    let (backend_tx, backend_rx) = mpsc::channel::<BackendEvent>(100);
    
    // Spawn background data fetchers
    let apps_fetcher = tokio::spawn(fetch_apps_loop(state.clone(), ui_tx.clone()));
    let logs_fetcher = tokio::spawn(fetch_logs_loop(state.clone(), ui_tx.clone()));
    
    // Main UI loop
    let mut tick = interval(Duration::from_millis(250));
    
    loop {
        tokio::select! {
            // Periodic UI refresh
            _ = tick.tick() => {
                terminal.draw(|f| draw_ui(f, &state)).await?;
            }
            
            // Handle UI events
            Some(event) = ui_rx.recv() => {
                match event {
                    UiEvent::AppsUpdated(apps) => {
                        *state.apps.write().await = apps;
                    }
                    UiEvent::LogsUpdated(logs) => {
                        *state.logs.write().await = logs;
                    }
                    UiEvent::Key(key) => {
                        if key == KeyCode::Char('q') {
                            *state.should_quit.write().await = true;
                        }
                        handle_input(&state, key).await?;
                    }
                }
            }
            
            // Handle backend events
            Some(event) = backend_rx.recv() => {
                handle_backend_event(&state, event).await?;
            }
            
            // Check quit condition
            _ = tokio::time::sleep(Duration::from_millis(16)) => {
                if *state.should_quit.read().await {
                    break;
                }
            }
        }
    }
    
    // Cleanup
    apps_fetcher.abort();
    logs_fetcher.abort();
    
    Ok(())
}

/// Background task: periodically fetch application status
async fn fetch_apps_loop(
    state: Arc<AppState>,
    tx: mpsc::Sender<UiEvent>,
) -> anyhow::Result<()> {
    let mut interval = interval(Duration::from_secs(5));
    
    loop {
        interval.tick().await;
        
        match fetch_all_apps().await {
            Ok(apps) => {
                tx.send(UiEvent::AppsUpdated(apps)).await.ok();
            }
            Err(e) => {
                tracing::warn!("Failed to fetch apps: {}", e);
            }
        }
    }
}

/// Background task: stream logs from running containers
async fn fetch_logs_loop(
    state: Arc<AppState>,
    tx: mpsc::Sender<UiEvent>,
) -> anyhow::Result<()> {
    let mut streams = LogStreamManager::new();
    
    // Add streams for all running apps
    for app in state.apps.read().await.iter() {
        if app.status == AppStatus::Running {
            streams.add_stream(&app.id).await?;
        }
    }
    
    while let Some(log_line) = streams.next().await {
        tx.send(UiEvent::LogsUpdated(vec![log_line])).await.ok();
    }
    
    Ok(())
}

/// CPU-bound work delegation
async fn process_large_dataset(data: Vec<Record>) -> Vec<ProcessedRecord> {
    // Use spawn_blocking for CPU-intensive work
    tokio::task::spawn_blocking(move || {
        data.into_par_iter()  // rayon parallel iterator
            .map(|record| process_record(record))
            .collect()
    }).await
    .expect("Blocking task panicked")
}
```

**WebSocket server with tokio**:
```rust
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures::{SinkExt, StreamExt};

/// Start the WebSocket app server
pub async fn start_ws_server(bind_addr: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(bind_addr).await?;
    info!("WebSocket server listening on {}", bind_addr);
    
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }
    
    Ok(())
}

async fn handle_connection(stream: TcpStream, addr: std::net::SocketAddr) {
    info!("New WebSocket connection from {}", addr);
    
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("WebSocket handshake failed: {}", e);
            return;
        }
    };
    
    let (mut write, mut read) = ws_stream.split();
    
    // Handle messages concurrently
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let response = handle_protocol_message(&text).await;
                if write.send(Message::Text(response)).await.is_err() {
                    break;
                }
            }
            Ok(Message::Close(_)) | Err(_) => break,
            _ => {}
        }
    }
    
    info!("WebSocket connection from {} closed", addr);
}
```

### Dependencies

**Cargo.toml**:
```toml
[dependencies]
# Core async runtime
tokio = { 
    version = "1.36", 
    features = [
        "rt-multi-thread",    # Multi-threaded scheduler
        "macros",             # #[tokio::main], #[tokio::test]
        "sync",               # Channels, locks, barriers
        "time",               # Delays, timeouts, intervals
        "signal",             # ctrl_c(), process signals
        "process",            # Async process spawning
        "fs",                 # Async filesystem operations
        "io-util",            # AsyncRead/AsyncWrite utilities
        "net",                # TCP, UDP, Unix sockets
    ] 
}

# Async utilities
tokio-util = { 
    version = "0.7", 
    features = [
        "codec",              # Framed codecs
        "compat",             # futures compatibility
        "time",               # Throttle, Timeout
    ] 
}

# Async traits for library interfaces
async-trait = "0.1"

# Futures utilities
futures = "0.3"
futures-util = "0.3"

# Runtime-agnostic async helpers (optional)
async-recursion = "1.0"

# Tokio-console support (dev/debug only)
console-subscriber = { version = "0.2", optional = true }

[dev-dependencies]
# Test utilities
tokio-test = "0.4"
```

### Runtime Configuration

**Single-threaded mode for testing**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test(flavor = "current_thread")]
    async fn test_async_function() {
        // Single-threaded for deterministic tests
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

**Multi-threaded for production**:
```rust
#[tokio::main(
    flavor = "multi_thread",
    worker_threads = 8,      // Match to CPU cores
    max_blocking_threads = 512  // For spawn_blocking
)]
async fn main() {
    // Production runtime with full parallelism
}
```

### Structured Concurrency Patterns

**Task supervision with JoinSet**:
```rust
use tokio::task::JoinSet;
use anyhow::Result;

async fn supervised_deployment(deployments: Vec<Deployment>) -> Result<()> {
    let mut set = JoinSet::new();
    
    // Spawn all deployments concurrently
    for deployment in deployments {
        set.spawn(async move {
            deploy_single(deployment).await
        });
    }
    
    // Wait for all with error aggregation
    let mut results = Vec::new();
    while let Some(result) = set.join_next().await {
        results.push(result?);
    }
    
    // Check for any failures
    let failures: Vec<_> = results.iter()
        .filter(|r| r.is_err())
        .collect();
    
    if !failures.is_empty() {
        anyhow::bail!("{} deployments failed", failures.len());
    }
    
    Ok(())
}
```

## Consequences

### Positive

- **Ecosystem compatibility**: Works with all major async libraries
- **ratatui integration**: Established patterns and examples available
- **Production readiness**: Battle-tested at scale
- **Rich feature set**: Built-in channels, timers, synchronization
- **Observability**: tracing integration, task dumps, tokio-console
- **Performance**: Optimized scheduler, io_uring on Linux
- **Documentation**: Extensive learning resources
- **Long-term support**: AWS backing ensures continued development

### Negative

- **Binary size**: +300KB overhead compared to smol
- **Compile time**: Slower builds than minimal runtimes
- **Learning curve**: Many concepts to master
- **Blocking task discipline**: Requires care for CPU-bound work

### Neutral

- **Runtime initialization**: Automatic with `#[tokio::main]`
- **Task cancellation**: Cooperative (requires manual checks)

## Alternatives Not Chosen

- **async-std**: Good API design but insufficient ecosystem and ratatui support
- **smol**: Excellent for size-constrained environments, lacks ecosystem depth needed
- **embassy**: Focused on embedded systems, not suitable for CLI applications
- **glommio**: Thread-per-core model, incompatible with many libraries

## Migration Path

If future requirements demand a smaller runtime:
1. Abstract async operations behind traits
2. Use `async-trait` for object-safe async interfaces
3. Isolate tokio-specific code to adapter modules

## References

- tokio: https://github.com/tokio-rs/tokio
- tokio docs: https://docs.rs/tokio/
- tokio tutorial: https://tokio.rs/tokio/tutorial
- ratatui with tokio examples: https://ratatui.rs/concepts/application-patterns/
- async-std: https://github.com/async-rs/async-std
- smol: https://github.com/smol-rs/smol
- heliosCLI SOTA Research: `docs/research/CLI_FRAMEWORKS_TUI_SOTA.md`

---

*This ADR will be updated as implementation progresses*
