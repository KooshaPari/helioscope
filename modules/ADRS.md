# Architectural Decision Records

## ADR-001: Plugin-Based Extension System

**Status:** Proposed

**Context:** heliosCLI needs to support extensions without core changes

**Decision:** Use plugin architecture with entry points

**Consequences:**
- Extensions load at runtime
- Clear contracts for extensions
- Version compatibility checking

---

## ADR-002: Async-First Handlers

**Status:** Proposed

**Context:** I/O-bound operations require async support

**Decision:** All handlers must support async execution

**Consequences:**
- Use asyncio throughout
- Provide sync wrappers for compatibility

---

## ADR-003: Configuration-Driven Modules

**Status:** Proposed

**Context:** Modules need flexible configuration

**Decision:** YAML/JSON config with validation

**Consequences:**
- Schema validation on load
- Hot reload support
- Environment variable interpolation
