# Draft PR Description: Layered TUI Optimization Program

## Title
Layered research-backed plan for Codex TUI renderer and backend optimization

## Summary
This PR introduces research artifacts and a dependency-aware plan for layered performance work in the Codex TUI stack. It does not change runtime behavior. It establishes architecture baselines, optimization candidates, and auditable evidence for follow-on implementation PRs.

## Why
- Current performance work lacks a single architecture baseline.
- Follow-on optimization work needs explicit slicing to reduce risk and improve reviewability.
- Reviewers need source-backed rationale for each optimization layer.

## Scope
- Adds `kitty-specs/001-codex-tui-renderer-optimization/spec.md`.
- Adds checklist validation results.
- Adds research outputs: architecture walkthrough, ASCII diagrams, evidence log, source register, and data model.
- Adds proposed layered PR sequence (PR-A through PR-E).

## Requested Follow-Up (Agent)
Please open a layered implementation PR series based on this research package:
1. PR-A instrumentation and benchmark baselines.
2. PR-B render workload reduction.
3. PR-C event backpressure/coalescing.
4. PR-D data-structure efficiency changes.
5. PR-E progressive rendering UX.

Each PR should include clear success signals and should not skip prerequisite layers.

## Risk Management
- Research-only PR keeps behavior unchanged.
- Layering allows rollback and isolated validation per concern.

## Reviewer Checklist
- Confirm architecture diagrams match current code paths.
- Confirm optimization rationale is evidence-backed.
- Confirm layer sequence is dependency-safe.
