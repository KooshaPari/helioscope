# ADR-AUTO-001: Specification Format - YAML with JSON Schema

**Status:** Accepted
**Date:** 2026-02-24
**Decisions:** heliosHarness Core Team
**Supersedes:** N/A

---

## Context

We need a machine-readable specification format for agent instructions that is:
- Human writable and readable
- Validatable via schema
- Version controllable
- Extensible

The options considered were:
1. YAML with JSON Schema
2. JSON with JSON Schema
3. TOML
4. Custom DSL (CUE, Dhall)

## Decision

We will use YAML 1.2 with JSON Schema validation for specifications.

```yaml
spec:
  name: "feature-auth"
  version: "1.0"
  verification:
    - test: auth_flow_test
  rollback:
    strategy: git_revert
  success_criteria:
    - metric: login_latency_ms < 100
```

## Rationale

- YAML is widely understood and human-readable
- JSON Schema provides strong validation
- Git diff shows meaningful changes
- Existing tooling (parsers, linters) available

## Alternatives Rejected

### JSON
- Less readable for humans
- Verbose for large specs

### TOML
- Limited nesting support
- Less ecosystem support

### Custom DSL (CUE)
- Learning curve
- Less familiar to team

## Consequences

### Positive
- Schema validation prevents bad specs
- Human-readable format
- Git-friendly diffs

### Negative
- YAML parsing edge cases
- Schema maintenance overhead

---

## Implementation

### Affected Components
- src/spec/parser.rs
- src/spec/schema.rs
- docs/spec-schema.json

### Migration Strategy
- Define schema first
- Implement parser
- Add validation layer

### Rollout Plan
- Phase 1: Schema definition
- Phase 2: Parser implementation
- Phase 3: Validation enforcement
