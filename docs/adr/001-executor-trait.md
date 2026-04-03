# ADR-001: Multi-Backend Executor Abstraction

**Status:** Accepted  
**Date:** 2026-04-02  
**Authors:** heliosCLI Team  
**Reviewers:** Sage Research Agent  

---

## Context

heliosCLI needs to support multiple deployment backends: Docker, Kubernetes, local process, and sandboxed environments. The question is how to abstract these different execution models without creating a mess of conditional logic.

### Research Reviewed

1. **"Design Patterns" (GoF, 1994)** — Adapter pattern for interface unification
2. **"Clean Architecture" (Martin, 2017)** — Dependency inversion, boundary abstractions
3. **OSS CLI Analysis** — docker/kubectl/tilt all use separate tools; no unified CLI exists
4. **Hexagonal Architecture Research** — Ports and adapters pattern for testability

### Alternatives Considered

| Approach | Pros | Cons | Research |
|----------|------|------|----------|
| Subcommand-per-backend (`helios docker`, `helios k8s`) | Clear separation | Duplicated flags, UX fragmentation | kubectl/docker pattern |
| Flag-per-backend (`--backend=docker`) | Unified UX | More complex implementation | Selected approach |
| Separate binaries | Clean boundaries | Maintenance overhead | Vagrant approach |
| Shell script wrappers | Quick to implement | No type safety, hard to test | Anti-pattern |

---

## Decision

**Adopt the Executor Trait Pattern with `--backend` flag.**

```rust
// src/execution/mod.rs
trait Executor {
    async fn deploy(&self, app: &App) -> Result<Deployment>;
    async fn status(&self, id: &str) -> Result<Status>;
    async fn logs(&self, id: &str) -> Result<LogStream>;
}
```

Backends implement this trait:
- `DockerExecutor` — Docker CLI integration
- `KubernetesExecutor` — Kubectl API integration
- `LocalExecutor` — Direct process execution
- `SandboxExecutor` — Seccomp/gvisor isolation

---

## Consequences

### Positive

1. **Single CLI Interface:** Users learn one command structure, not N
2. **Testability:** Mock executors for unit tests (hexagonal architecture)
3. **Extensibility:** New backends = new trait implementations
4. **Type Safety:** Rust compiler enforces backend completeness

### Negative

1. **Implementation Complexity:** Trait requires async, error unification
2. **Backend Parity:** Not all features available on all backends (documented)
3. **Learning Curve:** Team must understand trait system

### Neutral

1. **Performance:** Slight overhead vs. direct calls (measurable, acceptable)
2. **Binary Size:** All backends compiled in (could use features flags)

---

## Research Links

- GoF Adapter Pattern: https://en.wikipedia.org/wiki/Adapter_pattern
- Hexagonal Architecture: https://alistair.cockburn.us/hexagonal-architecture/
- Clean Architecture: https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html

---

## Related Decisions

- ADR-002: Error Handling with `thiserror`
- ADR-003: Async Runtime Selection (tokio)

---

**Supersedes:** N/A  
**Superseded by:** N/A
