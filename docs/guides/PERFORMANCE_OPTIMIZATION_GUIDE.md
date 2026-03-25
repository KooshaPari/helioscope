# Performance Optimization Guide

## Overview

This guide documents the performance optimization modules added in ADR-010 implementation.

## Modules

### 1. Resource Management (`resources.py`)

Safe subprocess handling with automatic cleanup.

```python
from harness.resources import safe_popen, ResourceMonitor

# Safe subprocess with automatic cleanup
with safe_popen(["ls", "-la"], stdout=PIPE) as proc:
    output = proc.stdout.read()

# Track resource usage
with ResourceMonitor() as monitor:
    # ... do work ...
    metrics = monitor.get_metrics()
    print(f"FD leak: {metrics.get('fd_leak', False)}")
```

### 2. HTTP Connection Pool (`http_pool.py`)

Reusable HTTP connections with pooling.

```python
from harness.http_pool import HTTPConnectionPool, get_http_client

# Get pooled client
client = get_http_client()
response = client.get("https://api.example.com")
```

### 3. Bounded Cache (`bounded_cache.py`)

Cache with size limits and eviction policies.

```python
from harness.bounded_cache import BoundedCache, EvictionPolicy

cache = BoundedCache(max_size=1000, ttl=300, policy=EvictionPolicy.LRU)
cache.set("key", "value")
value = cache.get("key")
```

### 4. Memory Profiling (`memory_profiler.py`)

Track memory usage and detect leaks.

```python
from harness.memory_profiler import MemoryProfiler

profiler = MemoryProfiler()
profiler.start()
# ... code ...
report = profiler.stop()
print(f"Memory growth: {report.rss_growth_mb}MB")
```

### 5. CPU Profiling (`cpu_profiler.py`)

CPU sampling and hotspot detection.

```python
from harness.cpu_profiler import CPUSampler, get_cpu_info

# Get CPU info
info = get_cpu_info()
print(f"Cores: {info['physical_cores']}")

# Sample CPU usage
sampler = CPUSampler()
sampler.start()
# ... code ...
stats = sampler.stop()
print(f"CPU peak: {stats['cpu_peak']}%")
```

### 6. Async Runtime (`async_runtime.py`)

Reusable event loop to avoid asyncio overhead.

```python
from harness.async_runtime import AsyncRuntime, gather_with_limit

runtime = AsyncRuntime.get_instance()

# Run async with concurrency limit
results = runtime.run(gather_with_limit(
    task1(), task2(), task3(),
    max_concurrent=5
))
```

### 7. Graceful Shutdown (`shutdown.py`)

Clean process termination.

```python
from harness.shutdown import GracefulShutdown, register_cleanup

shutdown = GracefulShutdown()
shutdown.register()

@shutdown.on_shutdown
def cleanup():
    # Cleanup code
    pass
```

### 8. Circuit Breaker (`circuit_breaker.py`)

Fault tolerance for external services.

```python
from harness.circuit_breaker import CircuitBreaker

breaker = CircuitBreaker("api", failure_threshold=5, timeout=30)
try:
    result = breaker.call(risky_function)
except CircuitOpenError:
    # Service unavailable
    pass
```

### 9. Request Batching (`request_batcher.py`)

Batch requests for efficiency.

```python
from harness.request_batcher import RequestBatcher

async def process_batch(items):
    return [process(i) for i in items]

batcher = RequestBatcher(process_batch, batch_size=10)
result = await batcher.submit("req1", item)
```

### 10. Latency Tracking (`latency_tracker.py`)

Track API latency metrics.

```python
from harness.latency_tracker import LatencyTracker

tracker = LatencyTracker()
tracker.record("api/users", 45.2, True)  # latency_ms, success
stats = tracker.get_stats("api/users")
print(f"P95: {stats.p95_ms}ms")
```

### 11. Teammate Registry (`teammates.py`)

Multi-agent coordination.

```python
from harness.teammates import TeammateRegistry, TeammateType

registry = TeammateRegistry()
registry.register(type=TeammateType.CODER, name="coder-1")
teammate = registry.get_available(TeammateType.CODER)
```

### 12. Delegation Protocol (`teammates.py`)

Agent-to-agent task delegation.

```python
from harness.teammates import DelegationProtocol

protocol = DelegationProtocol()
request = await protocol.delegate(
    from_agent="orchestrator",
    to_agent="coder-1",
    task="Write tests"
)
```

### 13. Task Queue (`task_queue.py`)

Priority queue with backpressure.

```python
from harness.task_queue import TaskQueue, TaskPriority

queue = TaskQueue(max_size=100, rate_limit=10)
task_id = await queue.submit("coder-1", payload, TaskPriority.HIGH)
```

### 14. Bulkhead (`bulkhead.py`)

Resource isolation between pools.

```python
from harness.bulkhead import Bulkhead, BulkheadConfig

bulkhead = Bulkhead(BulkheadConfig("coders", max_concurrent=5))
async with bulkhead.acquire():
    # Execute with limit
    pass
```

### 15. Scratchpad (`scratchpad.py`)

File buffering for context overflow.

```python
from harness.scratchpad import ScratchpadFileSystem

scratchpad = ScratchpadFileSystem(session_id)
file_id = await scratchpad.write("data.json", large_data, "Query results")
pointer = scratchpad.get_pointer(file_id)
```

### 16. Context Compaction (`context_compactor.py`)

Manage context window efficiently.

```python
from harness.context_compactor import ContextCompactor, ContextMessage, CompactionStrategy

compactor = ContextCompactor(
    CompactionConfig(max_tokens=80000, strategy=CompactionStrategy.SUMMARIZE)
)
compactor.add_message(ContextMessage(role="user", content="Hello", priority=5))
context = compactor.get_compacted_context()
```

### 17. Planning (`planning.py`)

Plan → Execute separation.

```python
from harness.planning import PlanExecutor

executor = PlanExecutor()
plan = executor.create_plan("Build app", [
    {"id": "setup", "description": "Setup project"},
    {"id": "code", "description": "Write code"},
])
```

### 18. Session Store (`session_store.py`)

Persistent session state.

```python
from harness.session_store import get_session_store

store = get_session_store()
session = store.create("Build todo app")
store.update(session.session_id, progress=0.5)
```

## Benchmark Commands

```bash
# Run LLM SLA benchmark
python -m harness.benchmarks.llm_sla_benchmark --agents 6

# Run extended benchmark
python -m harness.benchmarks.extended_benchmark --swarm

# Run harness comparison
python -m harness.benchmarks.harness_benchmark --compare
```

## Performance Targets

| Metric | Target |
|--------|---------|
| Memory leak detection | 0 leaks |
| CPU profiling | 90% coverage |
| Network latency | -30% |
| Agent orchestration | < 100ms overhead |
| Context efficiency | 80% |

## References

- ADR-010: Performance Optimization Master Plan
- WBS: ADR-010-WBS-PERFORMANCE-OPTIMIZATION.md
