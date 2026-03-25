# PM Spec & Planning: Teammate Subagent System + Dynamic Scaling

**Version**: 1.0  
**Date**: 2026-02-23  
**Project**: heliosHarness

---

## Executive Summary

This document provides the comprehensive product and implementation plan for adding:

1. **Teammate Subagent System** - Allow delegating tasks to specialized AI agents (coder, reviewer, tester, researcher)
2. **Dynamic Thread Limit System** - Replace fixed concurrency caps with resource-aware adaptive limits

### Key Outcomes

- Enable parallel agent execution without manual concurrency tuning
- Provide Claude Code-like teammate delegation experience
- Automatic scaling based on system resources (CPU, memory, FD, load)
- Built-in resilience patterns (circuit breaker, bulkhead, backpressure)

---

## 1. Problem Statement

### Current State
- Fixed thread/concurrency limits require manual tuning
- No native teammate/subagent delegation in Codex CLI harness
- No built-in resilience for concurrent agent execution
- Manual coordination required for multi-agent work

### Target State
- Dynamic, auto-scaling concurrency based on resources
- First-class teammate delegation with status tracking
- Built-in circuit breakers and bulkhead protection
- Full observability of agent swarm

---

## 2. Scope

### In Scope
- Teammate registry and discovery
- Async delegation protocol with status tracking
- Codex CLI integration for execution
- Dynamic thread limit computation
- Resource monitoring (CPU, memory, FD, load)
- Circuit breaker and bulkhead patterns
- Priority-based queue with backpressure
- CLI commands for all operations
- Metrics and logging

### Out of Scope
- Web UI (future phase)
- Integration with non-Codex agents (future phase)
- ML-based prediction (future phase)
- Distributed deployment (future phase)

---

## 3. Success Metrics

| Metric | Baseline | Target | Measurement |
|--------|----------|--------|-------------|
| Delegation latency | N/A | < 500ms | CLI timing |
| Dynamic limit accuracy | Fixed | ±10% of optimal | Resource sampling |
| Circuit breaker false positive | N/A | < 5% | Failure analysis |
| Queue backpressure response | N/A | < 1s | Metric timestamp |
| CLI command coverage | N/A | > 90% | Test coverage |

---

## 4. Implementation Phases

### Phase 1: Foundation (Week 1-2)
**Goal**: Core infrastructure for teammate system

| Deliverable | Owner | Dependencies |
|-------------|-------|--------------|
| Teammate data model | Backend | None |
| TeammateRegistry with auto-discovery | Backend | Data model |
| DelegationRequest/Result dataclasses | Backend | None |
| XML handoff protocol | Backend | Delegation model |
| Codex CLI adapter | Backend | None |

**Milestone**: M1 - Foundation Complete
- Teammates can be listed
- Delegations can be created
- Codex CLI executes successfully

### Phase 2: Core Runtime (Week 2-3)
**Goal**: Dynamic scaling and queue management

| Deliverable | Owner | Dependencies |
|-------------|-------|--------------|
| ResourceSnapshot implementation | Backend | None |
| HysteresisController | Backend | None |
| compute_dynamic_limit | Backend | Hysteresis |
| Priority queue | Backend | None |
| Backpressure mechanism | Backend | Queue |

**Milestone**: M2 - Core Runtime Complete
- Dynamic limits adjust with resources
- Queue rejects when full
- Backpressure activates correctly

### Phase 3: Resilience (Week 3-4)
**Goal**: Fault tolerance and safety

| Deliverable | Owner | Dependencies |
|-------------|-------|--------------|
| CircuitBreaker implementation | Backend | None |
| Bulkhead per resource type | Backend | None |
| Health monitoring | Backend | None |
| Auto-restart with backoff | Backend | Health |

**Milestone**: M3 - Resilience Complete
- Circuit breakers protect against cascade
- Bulkheads isolate resources
- Health monitoring detects failures

### Phase 4: Observability (Week 4-5)
**Goal**: Monitoring and CLI

| Deliverable | Owner | Dependencies |
|-------------|-------|--------------|
| Metrics collector | Backend | None |
| Structured JSON logging | Backend | None |
| CLI commands (teammates, scaling, queue) | Backend | All previous |
| Prometheus export | Backend | Metrics |

**Milestone**: M4 - Observability Complete
- All CLI commands functional
- Metrics available
- Dashboard data provided

### Phase 5: Integration (Week 5-6)
**Goal**: Testing and polish

| Deliverable | Owner | Dependencies |
|-------------|-------|--------------|
| End-to-end tests | QA | All previous |
| Performance benchmarks | Platform | All previous |
| API documentation | Docs | All previous |
| User guides | Docs | CLI complete |
| Bug fixes | Backend | Testing |

**Milestone**: M5 - Ship Ready
- All tests passing
- Documentation complete
- Performance targets met

---

## 5. Resource Requirements

### Team Composition

| Role | FTE | Phase |
|------|-----|-------|
| Senior Backend Engineer | 1.0 | All |
| Platform/DevOps Engineer | 0.5 | Phase 3-5 |
| Technical Writer | 0.25 | Phase 4-5 |
| QA Engineer | 0.5 | Phase 5 |

### Technical Dependencies

| Dependency | Version | Purpose |
|-----------|---------|---------|
| Python | 3.12+ | Runtime |
| psutil | 5.9+ | Resource monitoring |
| typer | 0.12+ | CLI framework |
| pydantic | 2.6+ | Data validation |
| httpx | 0.27+ | HTTP client |
| structlog | 24+ | Structured logging |

---

## 6. Risks and Mitigation

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Codex CLI API changes | High | Medium | Abstract adapter, monitor changes |
| Resource sampling performance | Medium | Low | Cache, optimize sampling interval |
| Concurrency bugs | High | Medium | Thorough testing, circuit breakers |
| Team capacity | Medium | High | Prioritize ruthlessly, scope control |

---

## 7. Communication Plan

### Weekly Updates
- Sprint review with demo
- Blockers and risks
- Metrics update

### Key Stakeholders
- Engineering lead: Weekly sync
- Product: Bi-weekly review
- DevOps: Phase 3+ sync

### Documentation
- ADR updates: As decisions made
- User guides: Before M4
- API docs: Continuous

---

## 8. Acceptance Criteria Summary

### Must Have (P0)
- [ ] Teammates discoverable and listable
- [ ] Delegation creates background task
- [ ] Status tracking works
- [ ] Dynamic limits compute from resources
- [ ] Circuit breaker prevents cascade
- [ ] CLI commands functional
- [ ] Metrics available

### Should Have (P1)
- [ ] Priority queue with backpressure
- [ ] Bulkhead isolation
- [ ] Health monitoring
- [ ] Prometheus export

### Nice to Have (P2)
- [ ] Git worktree isolation
- [ ] AST-aware merge
- [ ] TUI dashboard

---

## 9. Next Steps

1. **Immediate**: Team kickoff, assign WP ownership
2. **Week 1**: Complete WP-101 (Registry) and WP-102 (Delegation)
3. **Week 2**: Complete WP-103 (Codex integration), start WP-201
4. **Bi-weekly**: Milestone review, scope adjustment as needed

---

**Document Version**: 1.0  
**Status**: Approved for Execution  
**Next Review**: Week 2 Milestone
