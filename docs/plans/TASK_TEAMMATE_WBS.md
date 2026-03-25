# Work Breakdown Structure: Task/Teammate System + Dynamic Scaling

**Project**: heliosHarness - Teammate Subagent System  
**Version**: 1.0  
**Date**: 2026-02-23

---

## Phase 1: Foundation (Week 1-2)

### WP-101: Teammate Registry System
- [ ] **WP-101.1** Define Teammate data model (id, name, role, system_prompt, tools)
- [ ] **WP-101.2** Implement TeammateRegistry with auto-discovery from `agents/` directory
- [ ] **WP-101.3** Add YAML/JSON config support for teammate definitions
- [ ] **WP-101.4** Create CLI commands: `teammates list`, `teammates info <id>`
- [ ] **WP-101.5** Write unit tests for registry (coverage > 80%)

### WP-102: Delegation Protocol
- [ ] **WP-102.1** Define DelegationRequest/DelegationResult dataclasses
- [ ] **WP-102.2** Implement XML-based handoff protocol (Thought, Action, Result, Handoff)
- [ ] **WP-102.3** Create async delegation executor
- [ ] **WP-102.4** Add timeout and retry logic
- [ ] **WP-102.5** Implement delegation status tracking (pending, running, completed, failed)

### WP-103: Codex CLI Integration
- [ ] **WP-103.1** Research Codex CLI subagent execution options
- [ ] **WP-103.2** Implement Codex CLI adapter (codex exec --profile)
- [ ] **WP-103.3** Add support for isolated work directories per teammate
- [ ] **WP-103.4** Handle Codex CLI output parsing and result extraction
- [ ] **WP-103.5** Add error handling for Codex CLI failures

---

## Phase 2: Core Implementation (Week 2-3)

### WP-201: Dynamic Thread Limit System
- [ ] **WP-201.1** Implement ResourceSnapshot (CPU, memory, FD, load avg)
- [ ] **WP-201.2** Create HysteresisController with upper/lower thresholds
- [ ] **WP-201.3** Implement compute_dynamic_limit with resource-based calculation
- [ ] **WP-201.4** Add config support (YAML/env vars)
- [ ] **WP-201.5** Write unit tests for limit computation

### WP-202: Work Queue & Backpressure
- [ ] **WP-202.1** Implement priority-based task queue
- [ ] **WP-202.2** Add backpressure机制 (queue depth monitoring)
- [ ] **WP-202.3** Create graceful degradation when queue full
- [ ] **WP-202.4** Add queue metrics (size, capacity, load %)
- [ ] **WP-202.5** Implement priority escalation

### WP-203: Health Monitoring
- [ ] **WP-203.1** Implement agent heartbeat system
- [ ] **WP-203.2** Create health status classification (healthy, slow, unhealthy, crashed)
- [ ] **WP-203.3** Add automatic restart with exponential backoff
- [ ] **WP-203.4** Implement pause/resume instead of kill
- [ ] **WP-203.5** Add health check CLI commands

---

## Phase 3: Coordination & Safety (Week 3-4)

### WP-301: Git Parallelism
- [ ] **WP-301.1** Implement private GIT_INDEX_FILE per agent
- [ ] **WP-301.2** Create CAS-based commit references
- [ ] **WP-301.3** Add worktree support for full isolation
- [ ] **WP-301.4** Implement conflict detection

### WP-302: Circuit Breaker & Bulkhead
- [ ] **WP-302.1** Implement CircuitBreaker with state machine (CLOSED, OPEN, HALF_OPEN)
- [ ] **WP-302.2** Create Bulkhead for resource isolation
- [ ] **WP-302.3** Add circuit breaker listeners for state changes
- [ ] **WP-302.4** Implement bulkhead per resource type (CPU, I/O, DB)

### WP-303: Smart Merge
- [ ] **WP-303.1** Research Mergiraf integration
- [ ] **WP-303.2** Implement AST-aware conflict resolution
- [ ] **WP-303.3** Add merge result validation
- [ ] **WP-303.4** Create manual merge workflow for conflicts

---

## Phase 4: Observability & CLI (Week 4-5)

### WP-401: Metrics & Logging
- [ ] **WP-401.1** Implement metrics collector (circuit breaker, bulkhead, health)
- [ ] **WP-401.2** Add structured JSON logging
- [ ] **WP-401.3** Create Prometheus export endpoint
- [ ] **WP-401.4** Implement dashboard data provider

### WP-402: CLI Commands
- [ ] **WP-402.1** Complete `harness teammates delegate <teammate> <task>`
- [ ] **WP-402.2** Add `harness teammates status` for swarm view
- [ ] **WP-402.3** Create `harness scaling info` for dynamic limits
- [ ] **WP-402.4** Add `harness queue status` for queue monitoring
- [ ] **WP-402.5** Implement `harness health check`

### WP-403: Dashboard
- [ ] **WP-403.1** Create TUI dashboard for swarm monitoring
- [ ] **WP-403.2** Add real-time updates via polling
- [ ] **WP-403.3** Implement team visualization
- [ ] **WP-403.4** Add resource utilization graphs

---

## Phase 5: Integration & Testing (Week 5-6)

### WP-501: Integration Tests
- [ ] **WP-501.1** End-to-end delegation flow test
- [ ] **WP-501.2** Dynamic scaling stress test
- [ ] **WP-501.3** Circuit breaker failure injection test
- [ ] **WP-501.4** Concurrent teammate execution test

### WP-502: Documentation
- [ ] **WP-502.1** Write API documentation
- [ ] **WP-502.2** Create user guides
- [ ] **WP-502.3** Add troubleshooting guide
- [ ] **WP-502.4** Write architecture documentation

### WP-503: Performance Optimization
- [ ] **WP-503.1** Optimize resource sampling frequency
- [ ] **WP-503.2** Reduce delegation latency
- [ ] **WP-503.3** Profile and optimize hot paths
- [ ] **WP-503.4** Benchmark against baseline

---

## Work Item Dependencies

```
WP-101 ──┬──▶ WP-201 ──▶ WP-301 ──▶ WP-401 ──▶ WP-501
         │      │           │           │
         └──▶ WP-202 ◀───────▶ WP-302 ◀──│
         │      │                       │
         └──▶ WP-203 ◀─────────────────┘
         
WP-102 ──▶ WP-103 ──▶ WP-201 ──▶ WP-401 ──▶ WP-501
                                    │
WP-202 ──▶ WP-203 ──▶ WP-302 ──────┘
                                    │
WP-303 ─────────────────────────────┘
```

---

## Milestones

| Milestone | Target | Deliverables |
|-----------|--------|--------------|
| M1: Foundation | Week 2 | Registry, Delegation Protocol, Codex Adapter |
| M2: Core Runtime | Week 3 | Dynamic Limits, Queue, Health Monitoring |
| M3: Coordination | Week 4 | Git Parallelism, Circuit Breaker, Merge |
| M4: Observability | Week 5 | Metrics, CLI, Dashboard |
| M5: Complete | Week 6 | Integration Tests, Docs, Performance |

---

## Resource Allocation

| Role | Allocation |
|------|------------|
| Backend Engineer | 60% |
| Platform Engineer | 25% |
| Documentation | 15% |

---

**Document Version**: 1.0  
**Status**: Ready for Execution
