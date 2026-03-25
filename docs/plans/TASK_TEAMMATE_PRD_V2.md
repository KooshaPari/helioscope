# Product Requirements Document: Teammate Subagent System + Dynamic Scaling

**Version**: 2.0  
**Date**: 2026-02-23  
**Project**: heliosHarness

---

## Executive Summary

This PRD defines the requirements for implementing a **Teammate Subagent System** with **Dynamic Thread Scaling** in heliosHarness. The system enables:

1. **Teammate Delegation**: Delegate tasks to specialized AI agents (coder, reviewer, tester, researcher)
2. **Dynamic Scaling**: Auto-adjust concurrency based on system resources
3. **Resilience**: Circuit breakers, bulkheads, graceful degradation
4. **Optimization**: Multi-level caching, pre-warming, zero-copy operations

---

## 1. Problem Statement

### Current Pain Points

| Pain Point | Impact | Frequency |
|------------|--------|------------|
| Manual concurrency tuning | Over-provisioning or crashes | Weekly |
| No subagent delegation | Developer inefficiency | Daily |
| Cache misses on重复 requests | 40-80% latency increase | Hourly |
| No fault tolerance | Cascade failures | Monthly |
| Resource contention | System instability | Weekly |

### Target Outcomes

- **Latency**: <100ms for cached responses
- **Cost**: 46% reduction via plan caching
- **Reliability**: 99.9% uptime
- **Developer velocity**: 3x delegation throughput

---

## 2. User Personas

### Persona 1: Developer
**Name**: Alex  
**Role**: Software Engineer  
**Goals**: 
- Delegate code reviews to specialized teammate
- Get fast responses from cached results
- Not worry about system tuning

**Stories**:
- US-101: List available teammates
- US-102: Delegate task to teammate
- US-103: Monitor delegation status
- US-401: Batch delegation
- US-402: Delegation templates
- US-601: Multi-level cache

### Persona 2: Operator
**Name**: Jordan  
**Role**: DevOps Engineer  
**Goals**:
- System stays healthy under load
- Automatic scaling without intervention
- Clear visibility into resource usage

**Stories**:
- US-201: View dynamic scaling status
- US-202: Monitor queue health
- US-203: View circuit breaker status
- US-501: View resource status
- US-502: FD leak detection
- US-503: Memory pressure response
- US-701: NATS event bus

### Persona 3: Architect
**Name**: Sam  
**Role**: Technical Lead  
**Goals**:
- System design is scalable
- Clear patterns for extension
- Industry-standard approaches

**Stories**:
- US-701: NATS event bus integration
- US-702: Durable task execution
- US-703: Connection pooling

---

## 3. Product Overview

### Core Features

| Feature | Priority | Complexity |
|---------|-----------|-------------|
| Teammate registry & discovery | P0 | Medium |
| Async delegation protocol | P0 | High |
| Codex CLI integration | P0 | High |
| Dynamic thread limits | P0 | High |
| Circuit breaker | P0 | Medium |
| Priority queue + backpressure | P0 | Medium |
| Multi-level cache | P1 | High |
| Predictive pre-warming | P1 | High |
| Request coalescing | P1 | Medium |
| NATS integration | P2 | High |
| Connection pooling | P2 | Medium |

### Out of Scope (v1)
- Web UI (future phase)
- Multi-region deployment (future)
- ML-based prediction (v2)

---

## 4. Functional Requirements

### 4.1 Teammate System

| FR-ID | Requirement | Acceptance Criteria |
|--------|-------------|-------------------|
| FR-101 | Auto-discover teammates | Teammates in `agents/` found on startup |
| FR-102 | List teammates | `teammates list` shows all available |
| FR-103 | Delegate task | `delegate <teammate> <task>` creates async task |
| FR-104 | Track status | Status queryable at any time |
| FR-105 | Priority support | CRITICAL/HIGH/NORMAL/LOW priorities |
| FR-106 | Timeout handling | Configurable timeout with graceful failure |

### 4.2 Dynamic Scaling

| FR-ID | Requirement | Acceptance Criteria |
|--------|-------------|-------------------|
| FR-201 | Resource monitoring | CPU, memory, FD, load tracked |
| FR-202 | Dynamic limit calc | Limits adjust with resources |
| FR-203 | Hysteresis | No thrashing (30s dwell) |
| FR-204 | Safety buffers | 5% min, 15% discretionary |
| FR-205 | Manual override | CLI flag for fixed limit |

### 4.3 Resilience

| FR-ID | Requirement | Acceptance Criteria |
|--------|-------------|-------------------|
| FR-301 | Circuit breaker | Opens after 5 failures |
| FR-302 | Bulkhead | Separate pools per type |
| FR-303 | Health monitoring | Check every 10s |
| FR-304 | Auto-restart | Exponential backoff |
| FR-305 | Graceful degradation | Reduce quality not availability |

### 4.4 Caching & Optimization

| FR-ID | Requirement | Acceptance Criteria |
|--------|-------------|-------------------|
| FR-401 | L1 cache | TTLCache with 60s TTL |
| FR-402 | L2 cache | diskcache with 1hr TTL |
| FR-403 | Pre-warming | Background warming daemon |
| FR-404 | Request coalescing | Duplicate requests merged |
| FR-405 | Zero-copy | mmap for large files |

---

## 5. Non-Functional Requirements

### Performance

| NFR-ID | Metric | Target | Max |
|--------|--------|--------|-----|
| NFR-1 | Delegate start | <100ms | <500ms |
| NFR-2 | Cache hit latency | <10ms | <50ms |
| NFR-3 | Status query | <10ms | <50ms |
| NFR-4 | Memory/agent idle | <2MB | <10MB |

### Reliability

| NFR-ID | Metric | Target |
|--------|--------|--------|
| NFR-5 | Uptime | 99.9% |
| NFR-6 | Cache consistency | Eventual |
| NFR-7 | Failure isolation | <1s detection |

### Security

| NFR-ID | Requirement |
|--------|-------------|
| NFR-8 | Teammate isolation |
| NFR-9 | No credential leakage |
| NFR-10 | Input sanitization |

---

## 6. Technical Architecture

### Component Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        heliosHarness                              │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │   Teammate   │  │  Delegation  │  │   Codex CLI      │   │
│  │   Registry   │──│   Protocol   │──│   Executor       │   │
│  └──────────────┘  └──────────────┘  └──────────────────┘   │
│                              │                    │             │
│                              ▼                    ▼             │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │            Coordination Layer                           │   │
│  │  ┌──────────┐  ┌──────────┐  ┌────────────────┐  │   │
│  │  │ Circuit  │  │ Bulkhead │  │ Priority Queue │  │   │
│  │  │ Breaker  │  │          │  │ +Backpressure │  │   │
│  │  └──────────┘  └──────────┘  └────────────────┘  │   │
│  └──────────────────────────────────────────────────────────┘   │
│                              │                                 │
│                              ▼                                 │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │           Dynamic Limit Controller                      │   │
│  │  ┌────────────┐  ┌────────────┐  ┌──────────────┐  │   │
│  │  │ Resource  │  │ Hysteresis │  │ Limit Gate   │  │   │
│  │  │ Sampler   │  │ Controller │  │ (semaphore) │  │   │
│  │  └────────────┘  └────────────┘  └──────────────┘  │   │
│  └──────────────────────────────────────────────────────────┘   │
│                              │                                 │
│                              ▼                                 │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │           Caching & Optimization Layer                 │   │
│  │  ┌──────┐  ┌──────┐  ┌──────────┐  ┌──────────┐  │   │
│  │  │ L1   │  │ L2   │  │ Pre-warm │  │ Coalesce │  │   │
│  │  │Cache │  │Cache │  │           │  │          │  │   │
│  │  └──────┘  └──────┘  └──────────┘  └──────────┘  │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow

```
User Request
    │
    ▼
Teammate Registry (resolve teammate)
    │
    ▼
Delegation Protocol (create task)
    │
    ▼
Priority Queue (enqueue with priority)
    │
    ▼
Dynamic Limit Controller (check capacity)
    │
    ├─▶ Queue waiting (if at limit)
    │
    ▼
Circuit Breaker (check health)
    │
    ▼
Cache Layer (check L1/L2)
    │
    ├─▶ Cache hit → return cached result
    │
    ▼
Codex CLI Executor (execute task)
    │
    ▼
Result caching (store in L1/L2)
    │
    ▼
Return result
```

---

## 7. API Design

### CLI Commands

```bash
# Teammate management
harness teammates list                           # List all teammates
harness teammates info <id>                   # Show teammate details

# Delegation
harness teammates delegate <teammate> <task>   # Delegate task
harness teammates delegate <teammate> <task> --context ./src --priority high --timeout 300
harness teammates cancel <delegation-id>       # Cancel delegation

# Status
harness teammates status                      # Show all delegations
harness teammates status <id>               # Show specific delegation

# Scaling
harness scaling info                        # Show current limits
harness scaling set-limit 15                 # Override limit
harness scaling enable                      # Enable dynamic scaling
harness scaling disable                    # Disable dynamic scaling

# Queue
harness queue status                       # Show queue metrics
harness queue clear                        # Clear queue

# Resources
harness resources                         # Show resource usage
harness resources --watch                  # Watch mode
```

### Python API

```python
from heliosHarness import (
    TeammateRegistry,
    DelegationProtocol,
    DynamicLimitController,
    MultiLevelCache,
    CircuitBreaker,
)

# Create registry
registry = TeammateRegistry()
teammates = await registry.discover()

# Delegate
protocol = DelegationProtocol()
result = await protocol.delegate(
    teammate="code-reviewer",
    task="Review auth module",
    priority=Priority.HIGH,
    timeout=300,
)

# Dynamic scaling
controller = DynamicLimitController()
limit = controller.current_limit
```

---

## 8. Milestones

| Milestone | Target | Deliverables |
|-----------|--------|--------------|
| M1: Foundation | Week 2 | Registry, Delegation Protocol |
| M2: Scaling | Week 3 | Dynamic limits, Queue |
| M3: Resilience | Week 4 | Circuit breaker, Bulkhead |
| M4: Caching | Week 5 | Multi-level cache, Pre-warming |
| M5: Polish | Week 6 | CLI polish, Docs |

---

## 9. Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|-------------|
| Codex API changes | High | Abstract adapter layer |
| Cache invalidation | Medium | TTL + manual invalidation |
| Memory pressure | High | Aggressive cleanup |
| Concurrency bugs | High | Thorough testing |

---

## 10. Success Metrics

| Metric | Baseline | Target |
|--------|----------|---------|
| Delegate latency | N/A | <100ms |
| Cache hit rate | 0% | >60% |
| Cost reduction | 0% | >40% |
| Uptime | N/A | 99.9% |

---

## 11. Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| Python | 3.12+ | Runtime |
| psutil | 5.9+ | Resource monitoring |
| typer | 0.12+ | CLI framework |
| pydantic | 2.6+ | Validation |
| cachetools | 2.0+ | L1 cache |
| diskcache | 0.4+ | L2 cache |
| httpx | 0.27+ | HTTP client |

---

**Document Version**: 2.0  
**Status**: Approved for Implementation  
**Owner**: heliosHarness Team
