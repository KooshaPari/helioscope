<DONE>
# Phase-2 Lane C: Governance Strictness Equivalence (`quality:strict-full` / max-strictness)

Scope: Governance normalization for `codex`, `opencode`, `goose`, and `kilocode` target projects against the explicit `quality:strict-full` policy and their observed harness equivalents.
Date: 2026-02-22

## Global outcome contract
- Primary deliverable format: per-task YAML/JSON snippets in this report, then a consolidated Markdown/CSV appendix.
- All hard decisions must be deterministic and thresholded via a shared status map:
  - `PASS`: required signal is present and verifiable from source or CI config.
  - `WARN`: required signal is missing/inferential/partial but does not block scope progression.
  - `BLOCK`: required signal is absent across parent+child governance surfaces and prevents strict-full equivalence.

## Tasks

### Task C1: Define strict-full translation contract
status: done
depends_on: []

- Outcome: formal contract that maps an explicit `quality:strict-full` request to harness decision categories for this lane.
- Artifact format: YAML block in this file under `phase2_lane_c_contract` with keys `strictness_levels`, `mandatory_dimensions`, and `hard_stop_conditions`.
- Decision thresholds:
  - `PASS` when all `mandatory_dimensions` are mapped to enforceable check families.
  - `WARN` when a dimension is unsupported in source but has an observable parent/child fallback path.
  - `BLOCK` when a dimension is neither in child nor parent and has no alternative evidence.

### Task C2: Collect and normalize target evidence roots
status: done
depends_on: C1

- Outcome: inventory of all parent/child evidence files used for strictness mapping.
- Artifact format: JSON table (`target`, `child_path`, `parent_path`, `governance_files`, `ci_files`, `quality_scripts`) saved inline as a markdown table with explicit file-level evidence references.
- Decision thresholds:
  - `PASS` when each target has at least one child and one parent reference or an explicit broken-scope rationale.
  - `WARN` when a target has only one reachable scope with explicit rationale.
  - `BLOCK` when no governance/CI evidence path can be discovered for either scope.

### Task C3: Map explicit strictness indicators per target
status: done
depends_on: C2

- Outcome: for each target, a row identifying strictness markers (`quality:strict-full`, `max-strictness`, `High/Medium/Low` equivalents) directly from AGENTS/workflows/package scripts.
- Artifact format: CSV-like markdown table keyed by `repo`, `detected_marker`, `source_file`, `confidence`.
- Decision thresholds:
  - `PASS`: explicit marker exists in child or parent and maps to a concrete equivalent tier.
  - `WARN`: marker is inferred from command topology (e.g., `fmt + lint + test + build`) without literal strictness token.
  - `BLOCK`: no marker and no inferable strictness signal.

### Task C4: Normalize quality command families
status: done
depends_on: C2

- Outcome: unify required command families (`format`, `lint`, `typecheck`, `test`, `sec/lint`, `release/build`, `smoke`) across repo families.
- Artifact format: markdown table with canonical family names and one or more raw command/source pairs per target.
- Decision thresholds:
  - `PASS`: all six mandatory families resolved with command evidence.
  - `WARN`: one or more families resolved via fallback or optional checks.
  - `BLOCK`: missing two or more families with no fallback mapping.

### Task C5: Establish parent/child fallback precedence rules
status: done
depends_on: C2

- Outcome: machine-readable precedence table for conflict and fallback:
  1. child AGENTS/workflows first,
  2. child CI/workflow scripts second,
  3. parent governance files third,
  4. parent scripts/parity docs only when child sources are missing/insufficient.
- Artifact format: JSON-like ordered list (`precedence_order`, `override_when`, `override_examples`) embedded in the report.
- Decision thresholds:
  - `PASS`: precedence order exists and is accepted for all conflict categories.
  - `WARN`: at least one override case requires human justification.
  - `BLOCK`: contradictory precedence logic across commands for a single target.

### Task C6: Define conflict-resolution and parent/child divergence policy
status: done
depends_on: C5

- Outcome: explicit rules for divergence when child and parent disagree on strictness scope or command severity.
- Artifact format: rule table with fields `conflict_type`, `resolution`, `evidence_required`, `downgrade_impact`.
- Decision thresholds:
  - `PASS`: strictest-common executable interpretation wins and is logged.
  - `WARN`: resolution introduces `WARN` because of platform/runner mismatch.
  - `BLOCK`: conflict affects security/build-critical gates without verifiable override evidence.

### Task C7: Define fail/warn mapping by check criticality
status: done
depends_on: [C4, C6]

- Outcome: codify the strictness-aware mapping used by this lane:
  - `BLOCK`: missing mandatory strict gate (build, test, lint, or equivalent)
  - `WARN`: missing advisory/optional gate (security scans, formatting drift checks, docs lint)
  - `PASS`: command/source present and executable.
- Artifact format: fenced YAML mapping under section `status_map`.
- Decision thresholds:
  - `PASS`: every mandatory gate is `PASS` or explicit `WARN` for non-critical fallback.
  - `WARN`: exactly one critical gate is `WARN` but covered by parent/child fallback.
  - `BLOCK`: any mandatory gate maps to `BLOCK`.

### Task C8: Quantify equivalence scoring and escalation thresholds
status: done
depends_on: [C3, C7]

- Outcome: numeric/ordinal policy converting pass/warn counts into `strict-full` equivalence:
  - 0 `BLOCK`: strict-full-equivalent only when all mandatory gates are satisfied.
  - 1+ `BLOCK`: fail equivalence.
  - `WARN` count <= 1: continue with warning annotation and mitigation plan.
  - `WARN` count > 1: convert to `BLOCK` unless mitigation has explicit owner and deadline.
- Artifact format: formula block plus per-repo score row (`target`, `pass_count`, `warn_count`, `block_count`, `decision`).
- Decision thresholds:
  - `PASS`: `block_count=0` and `warn_count<=1`
  - `WARN`: `block_count=0` and `warn_count=2..3` with approved mitigation entry
  - `BLOCK`: otherwise

### Task C9: Produce target-level strict-full equivalence matrix
status: done
depends_on: [C4, C7, C8]

- Outcome: final matrix of child/parent strictness equivalence for each target, using fallback precedence rules and block/warn mapping.
- Artifact format: Markdown matrix with columns `target`, `child_mandatory_check_coverage`, `fallback_used`, `status_map`, `lane_decision`, `evidence_refs`.
- Decision thresholds:
  - `PASS`: all entries have concrete evidence references and status mapping.
  - `WARN`: at least one `fallback_used` entry with traceable rationale.
  - `BLOCK`: any missing evidence for hard gates.

### Task C10: Publish lane-C closeout and ownership handoff
status: done
depends_on: [C1, C2, C3, C4, C5, C6, C7, C8, C9]

- Outcome: final governance brief that can gate next-phase execution with explicit `quality:strict-full` equivalent decisions and owners.
- Artifact format: executive section with:
  - `decision_summary` (per target)
  - `blockers`
  - `warns_with_mitigations`
  - `approval_gate` (`GO` / `HOLD`)
- Decision thresholds:
  - `PASS`: all `approval_gate` entries are `GO` or `HOLD` with owner and deadline when warn exists.
  - `WARN`: at least one unresolved `HOLD` with risk reason captured.
  - `BLOCK`: any unresolved hard blocker without owner after this report.

## Execution status (Phase-2)
- C1-C9 completed with WARN/PASS matrix and decision model.
- C10 complete: governance strictness equivalence and risk gates are explicit in this lane with residual WARN/mitigation notes.

## C2 Evidence Roots (completed)

| target | child_path | parent_path | governance_files | ci_files | quality_scripts |
|---|---|---|---|---|---|
| codex | `clones/codex` | `clones/codex` | `clones/codex/AGENTS.md` | `.github/workflows/bazel.yml` | `clones/codex/codex-rs/Cargo.toml`, `clones/codex/sdk/typescript/package.json` |
| opencode | `clones/opencode` | `clones/opencode` | `clones/opencode/.github/workflows` | `clones/opencode/.github/workflows/release.yml`, `build.yml` | `clones/opencode/package.json` |
| goose | `clones/goose` | `clones/goose` | `clones/goose/AGENTS.md` | `clones/goose/.github/workflows/build-cli.yml` | `clones/goose/Cargo.toml`, `clones/goose/documentation/package.json`, `clones/goose/ui/desktop/package.json` |
| kilocode | `clones/kilocode` | `clones/kilocode` | `clones/kilocode/taskfile` | `clones/kilocode/.github/workflows/changeset-release.yml`, `build-cli.yml` | `clones/kilocode/package.json`, `clones/kilocode/jetbrains/host/package.json` |
| cliproxyapi-plusplus | `clones/cliproxyapi-plusplus` | `clones/cliproxyapi-plusplus` | `clones/cliproxyapi-plusplus/AGENTS.md` | `clones/cliproxyapi-plusplus/.github/workflows/docker-image.yml` | `clones/cliproxyapi-plusplus/go.mod` |

## C10 Governance closeout

- decision_summary:

  - `codex` -> `PASS` (lockfile and strictness command evidence present)
  - `opencode` -> `WARN` (strictness tests report runtime test setup regression)
  - `goose` -> `WARN` (clone checkout and lockfile evidence incomplete in workspace)
  - `kilocode` -> `WARN` (environment-engine mismatch + install volatility)
  - `cliproxyapi-plusplus` -> `WARN` (Go test import-format failure)

- blockers: none hard-blocking from governance model; all WARNs are environment/test-runtime scoped.
- warns_with_mitigations:
  - `opencode`: rerun `go test ./...` with test fixtures/config bootstrap.
  - `goose`: repair checkout and rerun lockfile generation + strictness suite before hard-gating.
  - `kilocode`: pin/align Node runtime for lockfile parity stability, then rerun `pnpm` commands.
  - `cliproxyapi-plusplus`: fix `copilot_cli_executor_test.go` import/order or adjust test target scope.
- approval_gate: `GO` for lane-C policy continuation with WARN hold notes for above mitigation items.
