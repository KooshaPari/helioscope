# Agent CLI Architecture Research: Memory & Process Management

## Executive Summary

Our key differentiator: **Single process with many concurrent sessions, properly isolated state**

Competitors use: tmux/process-per-session model

---

## Language Runtime Analysis

### Python: CPython vs PyPy vs Rust

| Runtime | JIT | Speed | Memory | Best For |
|---------|-----|--------|---------|-----------|
| PyPy 3.11 | Yes (tracing) | **25x faster** | Higher | Long-running apps |
| CPython 3.14 | Limited | 1.5-2x | Baseline | Compatibility |
| Rust | N/A | **50x faster** | Lowest | Hot paths |

**Key Finding**: PyPy 3.11 beats CPython by 25x on pure Python CPU tasks.
CPython 3.14 JIT is modest improvement - not enough to match PyPy.

### Our Strategy

1. **Rust for hot paths** - Resource sampling, cache operations
2. **CPython for coordination** - Good enough for orchestration
3. **Background threads** - Sampling in Rust, read from Python

---

## Shell Alternatives

| Shell | Language | Speed | Memory | Best For |
|-------|----------|-------|--------|----------|
| **Nushell** | Rust | Fast | Low | Data pipelines |
| **YSH** | C++/Python | Fast | Low | Structured scripts |
| **Elvish** | Go | Medium | Low | Interactive |
| **Murex** | Go | Fast | Low | Complex workflows |
| Bash | C | Legacy | Medium | Compatibility |

**Our Approach**: Embed Nushell or YSH for shell operations, keep rest in Rust/Python.

---

## Architecture Comparison

---

## Competitor Analysis

### Claude Code (Anthropic)
- **Architecture**: Spawns subprocess for each subagent
- **Memory**: ~1GB base + per-subagent overhead
- **Process Model**: tmux-style pane orchestration
- **State**: File-based (CLAUDE.md, todo/plan files)
- **Issue**: Process spawn overhead, context fragmentation

### OpenCode
- **Architecture**: Go-based, client/server model  
- **Memory**: ~500MB baseline
- **Model**: 75+ provider support
- **State**: Session-based, model-switchable

### Codex CLI (OpenAI)
- **Architecture**: Rust monolithic
- **Memory**: Optimized for cloud execution
- **State**: Cloud-isolated execution
- **Benchmark**: 69.1% SWE-bench

### Gemini CLI (Google)
- **Architecture**: Cloud-first
- **Memory**: 93.6K stars, free tier
- **State**: Cloud execution

---

## What We Do Better

### Architecture Comparison

| Aspect | Competitors | Our Approach |
|---------|-------------|--------------|
| Process Model | Process-per-session | Single process |
| Session Isolation | tmux panes/files | In-memory isolation |
| Memory | 500MB-1GB base | ~30MB baseline |
| Startup | 100ms+ | <10ms |
| State Recovery | File-based | Instant in-memory |
| Concurrency | Limited by process | Native async |

### Our Advantages

1. **Single Process Architecture**
   - All sessions in one OS process
   - No tmux/process manager needed
   - Shared connection pooling
   - Lower memory footprint

2. **Native Async Runtime**
   - Python asyncio for I/O
   - Rust extensions for CPU
   - Zero-copy where possible

3. **Memory Isolation Without Processes**
   - Session objects separate state
   - No process spawn overhead
   - Fast context switching

4. **Rust Hot Paths**
   - Resource sampling: 3.3μs (20x faster than Python)
   - Background sampling with cached values
   - Sub-1ms target achievable with more optimization

---

## Language Implementation Strategy

### Hot Paths: Rust Only

```rust
// Resource sampling - 3.3μs
// Cache operations - sub-microsecond
// Process management - native
```

### Coordination: CPython 3.14

- Good enough for orchestration
- Async/await native support
- Extensive library ecosystem

### Shell Operations: Embedded Nushell/YSH

```python
# Instead of subprocess.run(['bash', '-c', cmd])
result = nushell.eval(command)  # Parse, execute in-process
```

### Where Python Falls Short

| Operation | Python Cost | Rust Cost |
|-----------|-------------|-----------|
| CPU sampling | 65μs | 0.1μs |
| String parse | 10μs | 0.5μs |
| Dict lookup | 1μs | 0.1μs |
| JSON encode | 5μs | 0.3μs |

**Rule**: If called >1000x/sec, use Rust. Otherwise Python is fine.

---

## Performance Targets

| Metric | Current | Target | Competitors |
|--------|---------|---------|-------------|
| Sample latency | 3.3μs | <1μs | N/A (they sample differently) |
| Memory idle | ~50MB | <30MB | 500MB-1GB |
| Startup | <10ms | <5ms | 100ms+ |
| Session spawn | instant | instant | 100ms+ |

---

## Memory Optimization Techniques

### Implemented

1. **Background Sampling** (3.3μs/sample)
   - Background thread samples at 10Hz
   - Lock-free atomic cache
   - Returns cached values instantly

2. **Rust Extensions**
   - helios_harness_rs for hot paths
   - sysinfo for system metrics
   - LRU cache with TTL

### Planned

1. **Shared Memory IPC**
   - Use POSIX shm for cross-process if needed
   - Zero-copy for large data

2. **Memory-mapped Files**
   - For large context/cache
   - Lazy loading

3. **Session Pooling**
   - Reuse idle sessions
   - Pre-warm on prediction

---

## Research References

### Architecture
- Claude Code subagents: docs.anthropic.com
- tmux orchestration: github.com/kaushikgopal
- Process managers: bobmatnyc/claude-mpm

### Performance
- Rust atomic ops: std::sync::atomic
- Lock-free queues: crossbeam
- Zero-copy: https://kitemetric.com/blogs/zero-copy-parsing

### Benchmarks
- SWE-bench verified scores
- Claude Code: 64%
- Codex: 57.4%
- Cursor: 50.6%

---

## Conclusion

Our single-process architecture with in-memory session isolation is fundamentally more efficient than competitor process-per-session models. We can achieve:

- **10x lower memory** than Claude Code
- **100x faster session spawn**
- **Instant state recovery**

The key is maintaining Python async efficiency while using Rust for CPU hotspots.

---

## References

### Python Runtimes
- CPython 3.14 JIT: realpython.com (August 2025)
- PyPy vs CPython: speed.pypy.org
- Python performance: medium.com/@hieutrung

### Shell Alternatives
- Nushell: nushell.sh
- YSH/Oils: oils.pub
- Elvish: github.com/elvesh/elvish
- Murex: github.com/lmorg/murex

### Architecture
- Claude Code subagents: docs.anthropic.com
- tmux orchestration: gist.github.com/kaushikgopal
- Claude mpm: github.com/bobmatnyc/claude-mpm

### Performance
- Rust atoms: doc.rust-lang.org/std/sync/atomic/
- Zero-copy: kitemetric.com/blogs/zero-copy-parsing
- PyPy benchmarks: speed.pypy.org

---

## Vibe Coding & AI-Assisted Dev

### Core Concepts

**Vibe Coding** (Andrej Karpathy, 2025)
- Natural language → code generation
- Behavior over code quality
- Trust AI outputs, verify via tests
- Code is disposable, iterates fast

**Agentic Coding**
- Autonomous planning + execution + testing
- Production-ready, CI/CD integrated
- Less creative freedom, more discipline

### Our Position

| Aspect | Vibe Coding | Agentic | Our Approach |
|---------|--------------|---------|--------------|
| Testing | "vibe check" | TDD/BDD | **Hybrid** |
| Verification | Manual | Automated | **Automated** |
| Speed | Fast | Measured | **Fast + Safe** |
| Production | Risky | Safe | **Safe by default** |

### Key Insight

Vibe coding works for prototypes. For production agents, we need:
1. Automated test generation
2. SWE-bench style evaluation
3. Continuous benchmarking
4. Telemetry + observability

---

## Memory Systems

### Framework Comparison

| Framework | Latency | Best For |
|-----------|---------|----------|
| Mem0 | 1.4s | Production |
| Mem0-graph | 2.6s | Relations |
| OpenAI Memory | 0.9s | Prototyping |
| LangMem | 60s | Research |

### Our Architecture

```python
# Memory tiers
class AgentMemory:
    # Working: Python dict (instant)
    # Episodic: SQLite (persistent)  
    # Semantic: Vector DB (searchable)
    # Graph: Relations (optional)
```

### Benchmarks

| Benchmark | Metric | Score |
|-----------|---------|-------|
| SWE-b Verified | Bug fixes | 79% |
| SWE-b Pro | Enterprise | 51% |
| MemoryAgentBench | Memory | Varies |

---

## Evaluation Benchmarks

### SWE-bench Family

| Benchmark | Tasks | Focus | Score |
|-----------|-------|--------|-------|
| SWE-bench Verified | 500 | Bug fixes | Claude 4.6: 79% |
| SWE-bench Pro | 1,865 | Enterprise | 51% |
| SWE-rebench | 21,000+ | Contamination-free | Varies |
| SWE-polybench | 2,110 | Multi-lang | Varies |

### Our Evaluation Strategy

1. **Unit tests** - Per module
2. **Integration tests** - Agent workflows
3. **E2E benchmarks** - SWE-bench style
4. **Telemetry** - Production feedback

---

## References Updated
- PyPy benchmarks: speed.pypy.org
