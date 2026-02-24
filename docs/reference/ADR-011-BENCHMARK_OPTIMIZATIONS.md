# ADR-011: Benchmark-Specific Optimizations

**Date**: 2026-02-24  
**Status**: Proposed  
**Author**: heliosHarness Team

## Context

Current benchmarks show gaps that need targeted optimization:
- Terminal-Bench: 65% → Target 80%
- SWE-bench: <15% → Target 40%
- SWE-efficiency: 0.15x → Target 0.5x expert

This ADR outlines Phase 6 optimizations for benchmark-specific improvements.

## Decision

### Terminal-Bench Optimizations (Priority 1)

| Optimization | Target | Implementation |
|--------------|--------|----------------|
| Shell timeout handling | <5s | Configurable, task-specific |
| TUI render latency | <10ms | Delta rendering |
| Subprocess spawn | <50ms | PTY reuse |
| Keyboard interrupt | <5ms | Priority queue |

### SWE-bench Optimizations (Priority 2)

| Optimization | Target | Implementation |
|--------------|--------|----------------|
| Multi-file context | 10 files | Graph-based traversal |
| Patch generation | Minimal | Test-aware diffs |
| Build integration | Auto-detect | Ninja/CMake/Make |

### SWE-efficiency Optimizations (Priority 3)

| Optimization | Target | Implementation |
|--------------|--------|----------------|
| Static analysis | Bottleneck detection | Perf integration |
| Optimization patterns | Expert-level | Pattern library |
| Correctness verification | 100% | Auto-test generation |

## Implementation Phases

### 6.1: Terminal-Bench Focus
- Shell timeout config
- TUI delta rendering
- Subprocess pooling

### 6.2: SWE-bench Focus
- Context graph
- Patch generation
- Build integration

### 6.3: SWE-efficiency Focus
- Performance analysis
- Optimization patterns
- Verification framework

## Consequences

### Positive
- Benchmark scores improved
- Production reliability
- Multi-platform support

### Negative
- Complexity increase
- Testing overhead
- Maintenance burden

### Neutral
- May require benchmark-specific tuning

---

*ADR-011 created 2026-02-24*
