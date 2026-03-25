<DONE>
# CLI/API/SDK Harness Spec (Phase-1 Draft)

## Purpose
Create a reusable, evidence-driven harness to evaluate OSS CLI/API/SDK repos on two axes:
- **Functionality**: install/build/test/runtime behavior.
- **Quality parity**: strictness and governance equivalence to project rules (`strict`, `strict-full`, `max-strictness`, `no-skip`).

## Target contract
The harness must support:
- CLI repos with mixed ecosystems (`go`, `rust`, `node`, `python`).
- API/SDK adjunct repos that ship CLI-compatible adapters.
- Parent/child repository dependency (e.g., child project missing but parent quality evidence exists).
- Non-blocking execution: strict failures escalate only when hard-gate semantics are clear.

## Canonical command profile model
For each repo, map observed commands into five buckets:
1. **Bootstrap**: clone/checkout/install/bootstrap.
2. **Static quality**: format/lint/typecheck/vet.
3. **Behavioral checks**: unit/integration tests.
4. **Build/packaging**: binary/build artifacts/publish staging.
5. **API artifacts**: schema/build docs/openapi/client generation.

### Strictness equivalence mapping
- `strict-full-noskip` (target): fail on any command or missing mandatory check; equivalent to repo-local `quality`, `task quality`, or enforced workflow matrix.
- `strict` (target): allow documented non-blocking external dependencies but no skipped mandatory checks.
- `strict-light`: warning-based when explicit evidence is incomplete.

## Harness architecture

### Modules
- `discoverer`
  - Detect repository root, toolchain, commands, workflow inventory, and AGENTS/contributing guidance.
- `runner`
  - Execute mapped commands in isolated profiles:
    - `bootstrap`
    - `quality`
    - `api`
    - `runtime`
- `quality-normalizer`
  - Normalize command naming into the canonical profile model.
  - Promote local project profiles:
    - `task quality`
    - `quality:strict`
    - `task test`
- `result-schema`
  - Emit evidence bundle with:
    - repo, branch, commit hash, remote
    - commands executed/failed
    - check timestamps
    - strictness level
    - skip rationale (if any)
- `comparator`
  - Score and rank for harness eligibility and shortlist.

### Data model (minimum fields)
- `repo_id`, `path`, `remote`, `branch`, `commit`, `toolchain`, `commands`, `quality_profile`, `quality_results`, `api_surface`, `mcp_surface`, `auth_profile`, `parent_repo`.
- `result_code`: PASS/WARN/FAIL, with reason code taxonomy.

## Multi-repo policy (parent + child)
- If child repo is missing/malformed, run parent repo quality profile and mark child fields as **inherited**.
- If both parent and child have quality evidence, both are recorded; child errors do not erase parent score.
- Parent fallback is allowed only with explicit justification in `result_code=WARN` unless a child hard-gate is explicitly required.

## Candidate onboarding workflow
1. `clone` and `status --short` each target.
2. Capture branch + commit + remote for traceability.
3. Detect quality commands and map into profile buckets.
4. Execute strict profile command set in canonical order:
   - bootstrap
   - quality
   - tests
   - build
   - api
5. Emit per-repo artifact under `artifacts/`.
6. Emit consolidated matrix and governance summary for phase handoff.

## Next phase design intent
Phase-2 should add:
- machine-readable schema for `harness-manifest.json`.
- CI dry-run runner with capped concurrency.
- strictness policy file (`harness/strictness.yaml`) to avoid manual interpretation.
- artifact contract tests (schema + command coverage).
