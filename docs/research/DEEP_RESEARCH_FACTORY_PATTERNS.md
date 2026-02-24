# Deep Research: Factory Droid Patterns for Multi-Project Ecosystem

**Date:** 2026-02-24

---

## 1. Factory Droid Architecture Deep Dive

### 1.1 Core Components

Factory's droids operate with these key components:

| Component | Description | Implementation Pattern |
|-----------|-------------|----------------------|
| **Memory System** | Persistent context across sessions | SQLite + Redis |
| **Specification Engine** | Parse and validate specs | YAML/JSON Schema |
| **Verification Pipeline** | Pre-execution validation | Test runners + scanners |
| **Checkpoint Manager** | State preservation | Git + snapshots |
| **Rollback Engine** | Atomic revert | Git revert + state restore |
| **Orchestrator** | Root + sub-agent coordination | Message queues |

### 1.2 Autonomy Levels (Detailed)

```
L0: MANUAL
├── Human writes all code
├── Human approves all changes
└── Human deploys

L1: ASSISTED
├── AI suggests code
├── Human approves
└── Human deploys

L2: CONDITIONAL
├── AI executes
├── Auto-rollback on failure
└── Human alerted (not asked)

L3: AUTONOMOUS
├── AI interprets intent
├── AI executes
├── AI rolls back if needed
└── Human notified (not asked)

L4: SELF-HEALING
├── Full automation
├── Automatic recovery
├── Self-optimization
└── Zero human intervention
```

---

## 2. Applying Factory Patterns to Each Project

### 2.1 heliosHarness → Root Manager

| Factory Pattern | heliosHarness Implementation | Status |
|----------------|------------------------------|--------|
| Memory | Checkpoint + audit system | In Progress |
| Spec Engine | YAML/JSON parser | In Progress |
| Verification | Test runner pipeline | Not Started |
| Checkpoint | Git state capture | In Progress |
| Rollback | Atomic revert | Not Started |
| Orchestration | Root + sub-agents | Design |

### 2.2 thegent → Reference Implementation

| Pattern | thegent Implementation | Maturity |
|---------|----------------------|----------|
| CLI Interface | Rich CLI with clap | L3 |
| Agent Memory | Session-based | L2 |
| Tool Execution | MCP tools | L3 |
| Code Generation | LLM-powered | L3 |
| Governance | TDD/BDD docs | L2 |

**Learnings from thegent:**
- Session-based memory works for CLI
- MCP tools provide excellent abstraction
- Git integration essential for rollback

### 2.3 4sgm → Python Tooling

| Pattern | 4sgm Implementation | Notes |
|---------|---------------------|-------|
| Task Automation | Taskfile.yml | Mature |
| Python Package | pyproject.toml | Production |
| Observability | Langfuse | Integrated |
| Testing | pytest | Comprehensive |

### 2.4 tokenledger → Cost Governance

| Pattern | tokenledger Implementation | Notes |
|---------|---------------------------|-------|
| Provider Abstraction | Trait-based | Rust |
| Rate Limiting | governor crate | Production |
| Cost Tracking | Multi-provider | Complete |
| Optimization | Recommendations | L1 |

### 2.5 trace → Requirements Traceability

| Pattern | trace Implementation | Notes |
|---------|----------------------|-------|
| User Stories | Template-driven | Complete |
| Epic Management | Phased approach | Complete |
| FR Tracking | Structured docs | Complete |
| ADR System | Template-based | Complete |

---

## 3. SDD Implementation Across Projects

### 3.1 Specification Format (Unified)

```yaml
# Universal Spec Format
spec:
  name: "feature-name"
  version: "1.0.0"
  owner: "team-name"
  
  # SDD Components
  verification:
    - test: "test_name"
    - security: "scan_type"
    - performance: "benchmark_name"
    
  rollback:
    strategy: git_revert  # | snapshot | hybrid
    checkpoint: required
    
  success_criteria:
    - metric: "value > threshold"
    
  # BDD Components
  behavior:
    given: "initial_state"
    when: "action"
    then: "expected_outcome"
```

### 3.2 Verification Pipeline

```
┌─────────────┐
│   SPEC     │
└──────┬──────┘
       │
       ▼
┌─────────────┐     ┌─────────────┐
│   PARSE     │────▶│  VALIDATE   │
└─────────────┘     └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │  CHECKPOINT │
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │   EXECUTE   │
                    └──────┬──────┘
                           │
              ┌────────────┴────────────┐
              │                         │
              ▼                         ▼
       ┌─────────────┐          ┌─────────────┐
       │   VERIFY    │          │   ROLLBACK  │
       └──────┬──────┘          └─────────────┘
              │                         │
              └────────────┬────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │    AUDIT    │
                    └─────────────┘
```

---

## 4. BDD Coordination Patterns

### 4.1 Agent-to-Agent Communication

```gherkin
Feature: Sub-Agent Coordination

  Scenario: Root manager assigns task
    Given root manager has specification
    When spec is validated
    Then decompose into subtasks
    And assign to available agents
    
  Scenario: Agent completes task
    Given agent executes subtask
    When verification passes
    Then report success to root
    And signal ready for next task
    
  Scenario: Agent fails task
    Given agent encounters error
    When error exceeds threshold
    Then trigger rollback
    And notify root manager
    And request new direction
```

### 4.2 Human-Agent Interaction

```gherkin
Feature: Elicitation-Only UX

  Scenario: User provides idea
    Given user provides natural language idea
    When elicitation handler processes
    Then classify intent
    And generate specification
    And queue for autonomous execution
    
  Scenario: Execution completes
    Given agent execution finishes
    When verification passes
    Then notify user of success
    And provide execution summary
    
  Scenario: Execution fails
    Given agent execution fails
    When rollback completes
    Then notify user of failure
    And explain what went wrong
```

---

## 5. Integration Strategy

### 5.1 Project Dependencies

```
heliosHarness (Root Manager)
    │
    ├──▶ thegent (Agent Executor)
    │        └──▶ agentapi++ (MCP Tools)
    │
    ├──▶ 4sgm (Python Tasks)
    │        └──▶ trace (Requirements)
    │
    └──▶ tokenledger (Cost Governance)
             └──▶ cliproxy++ (LLM Proxy)
```

### 5.2 Data Flow

1. **Elicitation** → User provides idea
2. **Spec Generation** → Convert to YAML spec
3. **Verification** → Checkpoint + validate
4. **Execution** → Route to appropriate agent
5. **Verification** → Run tests/scans
6. **Audit** → Log all actions
7. **Notify** → Alert user of result

---

## 6. Key Technical Decisions

### 6.1 Specification Format

| Option | Pros | Cons | Decision |
|--------|------|------|----------|
| YAML | Human-readable, git-friendly | Parsing edge cases | **Selected** |
| JSON | Strict, widely used | Less readable | Alternative |
| TOML | Simple | Limited nesting | Not selected |
| Custom DSL | Fully controllable | Learning curve | Deferred |

### 6.2 Checkpoint Strategy

| Layer | Storage | Retention | Trigger |
|-------|---------|-----------|---------|
| Git | Git | Forever | Pre-action |
| Config | SQLite | 30 days | Pre-deploy |
| DB | PostgreSQL | 7 days | Migration |
| Metrics | Time-series | 90 days | Hourly |

### 6.3 Rollback Strategy

| Trigger | Action | Verification |
|---------|--------|--------------|
| Test failure | Git revert | State comparison |
| Security finding | Quarantine | Scan clean |
| Performance regression | Traffic rollback | Metrics normal |
| Manual stop | Graceful halt | Clean shutdown |

---

## 7. References

- Factory Droids: https://factory.ai/news/ga
- SDD Paper: arXiv:2602.00180
- AWS AutoPilot: Self-driving DevOps
- ArgoCD: GitOps + Rollback
- Temporal: Workflow state machine

---

*Last Updated: 2026-02-24*
