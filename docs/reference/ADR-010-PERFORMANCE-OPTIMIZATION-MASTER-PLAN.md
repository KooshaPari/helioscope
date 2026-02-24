# ADR-010: Performance Optimization Master Plan

**Date**: 2026-02-23  
**Status**: Implemented  
**Author**: heliosHarness Team

## Context

The Codex CLI harness needs systematic performance optimization across multiple dimensions:
- Memory leaks and resource management
- CPU utilization and profiling
- Network latency and connection handling
- Shell-level optimization
- Multi-agent orchestration

This ADR establishes a phased implementation plan with Work Breakdown Structure (WBS).

## Decision

We will implement a **5-Phase Optimization Plan** with prioritized WBS:

---

## Phase 1: Memory Safety & Leak Prevention (Weeks 1-2)

**Goal**: Eliminate memory leaks, add RAII guards, enable profiling

### WBS Tasks

| ID | Task | Effort | Priority |
|----|------|--------|----------|
| 1.1 | Add `finally` blocks to all `subprocess.Popen` calls | 2 days | P0 |
| 1.2 | Implement connection pooling for HTTP clients | 2 days | P0 |
| 1.3 | Add bounded cache with max size limits | 2 days | P0 |
| 1.4 | Implement memory profiling in benchmark suite | 2 days | P1 |
| 1.5 | Add leak detection to CI pipeline | 2 days | P1 |
| 1.6 | Enable M1/M2 native builds with SIMD | 1 day | P2 |

### ADR Reference
- ADR-010-001: Subprocess Resource Management
- ADR-010-002: HTTP Connection Pooling

---

## Phase 2: CPU Optimization & Profiling (Weeks 3-4)

**Goal**: Reduce CPU overhead, add profiling, optimize hot paths

### WBS Tasks

| ID | Task | Effort | Priority |
|----|------|--------|----------|
| 2.1 | Integrate `perf` profiling in benchmarks | 2 days | P0 |
| 2.2 | Add Instruments (macOS) profiling support | 2 days | P0 |
| 2.3 | Implement async event loop reuse | 2 days | P1 |
| 2.4 | Add CPU hotspot detection to CI | 1 day | P1 |
| 2.5 | Optimize JSON parsing with simd-json | 2 days | P2 |
| 2.6 | Implement graceful shutdown handlers | 1 day | P2 |

### ADR Reference
- ADR-010-003: Async Runtime Configuration
- ADR-010-004: Profiling Infrastructure

---

## Phase 3: Network & Latency Optimization (Weeks 5-6)

**Goal**: Reduce network latency, optimize API calls, connection reuse

### WBS Tasks

| ID | Task | Effort | Priority |
|----|------|--------|----------|
| 3.1 | Implement HTTP connection pooling | 2 days | P0 |
| 3.2 | Add request batching for LLM calls | 2 days | P0 |
| 3.3 | Implement response streaming | 2 days | P1 |
| 3.4 | Add circuit breakers for external APIs | 2 days | P1 |
| 3.5 | Implement connection keep-alive | 1 day | P2 |
| 3.6 | Add latency metrics to benchmark | 1 day | P2 |

### ADR Reference
- ADR-010-005: Network Optimization
- ADR-010-006: Circuit Breaker Pattern

---

## Phase 4: Multi-Agent Orchestration (Weeks 7-10)

**Goal**: Implement teammate system, parallel execution, swarm coordination

### WBS Tasks

| ID | Task | Effort | Priority |
|----|------|--------|----------|
| 4.1 | Implement Teammate Registry | 3 days | P0 |
| 4.2 | Build Delegation Protocol | 3 days | P0 |
| 4.3 | Create Codex CLI Executor | 3 days | P0 |
| 4.4 | Add Circuit Breaker coordination | 2 days | P1 |
| 4.5 | Implement Queue-based execution | 2 days | P1 |
| 4.6 | Add Bulkhead pattern | 2 days | P1 |
| 4.7 | Implement Fan-out to subagents | 3 days | P2 |
| 4.8 | Add Dynamic Subagent spawning | 3 days | P2 |

### ADR Reference
- ADR-001: Teammate System (existing)
- ADR-003: Multi-Agent Coordination (existing)

---

## Phase 5: Context Engineering & Harness Patterns (Weeks 11-14)

**Goal**: Implement Hightouch-style harness patterns

### WBS Tasks

| ID | Task | Effort | Priority |
|----|------|--------|----------|
| 5.1 | Implement File Buffering system | 3 days | P0 |
| 5.2 | Build Context Compaction engine | 3 days | P0 |
| 5.3 | Add Planning → Execution separation | 3 days | P1 |
| 5.4 | Implement Verification loops | 3 days | P1 |
| 5.5 | Add Dynamic Plan updates | 2 days | P2 |
| 5.6 | Implement Session persistence | 2 days | P2 |

### ADR Reference
- ADR-010-007: Context Engineering
- ADR-010-008: Verification Framework

---

## Implementation Dependencies

```
Phase 1 ─────┐
              ├──► Phase 2 ─────┐
              │                 ├──► Phase 3 ─────┐
              │                 │                 ├──► Phase 4 ─────┐
              │                 │                 │                 ├──► Phase 5
              │                 │                 │                 
              ▼                 ▼                 ▼                 ▼
Memory/Leak   CPU/Profile     Network/Latency   Agents            Context
```

---

## Success Metrics

| Phase | Metric | Target |
|-------|--------|--------|
| Phase 1 | Memory leak detection | 0 leaks in CI |
| Phase 2 | CPU profiling coverage | 90% of hot paths |
| Phase 3 | Network latency reduction | 30% faster |
| Phase 4 | Agent orchestration overhead | < 100ms |
| Phase 5 | Context window utilization | 80% efficiency |

---

## Resource Allocation

| Role | Phase 1 | Phase 2 | Phase 3 | Phase 4 | Phase 5 |
|------|---------|---------|---------|---------|---------|
| Backend | 60% | 60% | 50% | 70% | 60% |
| DevOps | 20% | 30% | 30% | 10% | 10% |
| Research | 20% | 10% | 20% | 20% | 30% |

---

## ADR Templates

### ADR-010-001: Subprocess Resource Management

**Status**: Proposed

**Context**: Subprocess.Popen calls without proper cleanup cause FD leaks

**Decision**: 
- Wrap all Popen in context managers
- Add finally blocks for all handle cleanup
- Track FD count in metrics

**Consequences**:
- Positive: Eliminates FD leaks
- Negative: More verbose code
- Neutral: Requires code review for compliance

### ADR-010-002: HTTP Connection Pooling

**Status**: Proposed

**Context**: HTTP clients create new connections per request

**Decision**:
- Implement httpx.Client with connection pool
- Set max connections per host
- Enable keep-alive

**Consequences**:
- Positive: 30-50% latency reduction
- Negative: Connection state management complexity

### ADR-010-003: Async Runtime Configuration

**Status**: Proposed

**Context**: asyncio.run() called per operation creates event loop overhead

**Decision**:
- Create single event loop per process
- Reuse across operations
- Add proper lifecycle management

### ADR-010-004: Profiling Infrastructure

**Status**: Proposed

**Context**: No systematic profiling in CI

**Decision**:
- Add perf integration for Linux
- Add Instruments support for macOS
- Enable in CI for release builds

### ADR-010-005: Network Optimization

**Status**: Proposed

**Context**: Network calls have high latency overhead

**Decision**:
- Connection pooling (see ADR-010-002)
- Request batching where possible
- Response streaming for LLM calls

### ADR-010-006: Circuit Breaker Pattern

**Status**: Proposed

**Context**: External API failures cascade

**Decision**:
- Implement circuit breaker for all external calls
- Track failure rates
- Auto-recovery after cooldown

### ADR-010-007: Context Engineering

**Status**: Proposed

**Context**: Context window overflow in long sessions

**Decision**:
- File buffering for large data
- Context compaction (summarization)
- Priority-based context injection

### ADR-010-008: Verification Framework

**Status**: Proposed

**Context**: No systematic output verification

**Decision**:
- Schema validation for tool outputs
- Test execution for code generation
- Incremental verification loops

---

## Review Schedule

| Phase | Review Date | Gate |
|-------|-------------|------|
| Phase 1 | Week 2 | Memory leak free |
| Phase 2 | Week 4 | Profile coverage 90% |
| Phase 3 | Week 6 | Latency -30% |
| Phase 4 | Week 10 | Agents operational |
| Phase 5 | Week 14 | Context efficiency 80% |

---

*ADR-010 created 2026-02-23*
