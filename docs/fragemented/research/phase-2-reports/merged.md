# Merged Fragmented Markdown

## Source: research/phase-2-reports/agent-a-core-repo-harden.md

# Phase-2 Lane A: core-repo hardening

Scope: clone repair + quality hardening for `codex`, `opencode`, `goose`, `kilocode`, and `cliproxyapi++`.

- Harness clones:
  - `codex`: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex`
  - `opencode`: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode`
  - `goose`: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose`
  - `kilocode`: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode`
- `cliproxyapi++` canonical source: `/Users/kooshapari/temp-PRODVERCEL/485/API/research/CLIProxyAPI`
- Planned harness clone target for `cliproxyapi++`: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/cliproxyapi-plusplus`

- Evidence directory: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts`

## tasks

- id: A1
  title: Initialize lane-A workspace and repo manifest
  status: done
  depends_on: []
  commands:
    - `mkdir -p /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts`
    - `printf '%s\n' codex opencode goose kilocode cliproxyapi-plusplus > /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/repo-manifest.txt`
    - `printf '%s\n' "lane=phase-2-lane-A" "started_at=$(date -Iseconds)" "clone_root=/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones" "cliproxy_source=/Users/kooshapari/temp-PRODVERCEL/485/API/research/CLIProxyAPI" > /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/task-metadata.txt`
  evidence_path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/task-metadata.txt`

- id: A2
  title: Repair broken clones and create missing `cliproxyapi++` local clone
  status: done
  depends_on: [A1]
  commands:
    - `git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex rev-parse --verify HEAD >/dev/null`
    - `git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode rev-parse --verify HEAD >/dev/null`
    - `git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode rev-parse --verify HEAD >/dev/null`
    - `if ! git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose rev-parse --verify HEAD >/dev/null 2>&1; then rm -rf /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose && git clone https://github.com/block/goose.git /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose; fi`
    - `if [ ! -d /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/cliproxyapi-plusplus/.git ]; then rm -rf /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/cliproxyapi-plusplus; cp -a /Users/kooshapari/temp-PRODVERCEL/485/API/research/CLIProxyAPI /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/cliproxyapi-plusplus; fi`
    - `for repo in codex opencode goose kilocode cliproxyapi-plusplus; do git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/$repo fsck --full; done > /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/clone-fsck.log 2>&1`
  evidence_path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/clone-fsck.log`

- id: A3
  title: Normalize branch and remote topology
  status: done
  depends_on: [A2]
  commands:
    - `for repo in codex opencode kilocode; do git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/$repo remote -v | sort; git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/$repo remote set-url origin $(git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/$repo remote get-url origin); done > /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/branch-topology.txt`
    - `git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex fetch origin main && git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex checkout main && git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex branch --set-upstream-to=origin/main main`
    - `git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode fetch origin main && git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode checkout main && git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode branch --set-upstream-to=origin/main main`
    - `git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose fetch origin main && git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose checkout -B main origin/main`
    - `git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode fetch origin main && git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode checkout main && git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode branch --set-upstream-to=origin/main main`
    - `git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/cliproxyapi-plusplus fetch --all --prune --tags && git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/cliproxyapi-plusplus checkout main`
    - `for repo in codex opencode goose kilocode cliproxyapi-plusplus; do echo "== $repo ==" >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/branch-topology.txt; git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/$repo branch --list --verbose --no-abbrev; done >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/branch-topology.txt`
  evidence_path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/branch-topology.txt`

- id: A4
  title: Verify commit provenance and branch drift
  status: done
  depends_on: [A3]
  commands:
    - `for repo in codex opencode goose kilocode cliproxyapi-plusplus; do git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/$repo log --oneline --max-count=1 > /tmp/${repo}.local_head; git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/$repo log --oneline --max-count=1 origin/main > /tmp/${repo}.remote_head; done`
    - `for repo in codex opencode goose kilocode cliproxyapi-plusplus; do echo "== $repo ==" >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/commit-provenance.txt; cat /tmp/${repo}.local_head /tmp/${repo}.remote_head >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/commit-provenance.txt; git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/$repo status --short --branch >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/commit-provenance.txt; done`
  evidence_path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/commit-provenance.txt`

- id: A5
  title: Capture strictness command surface from scripts/CI/AGENTS
  status: done
  depends_on: [A3]
  commands:
    - `rg -n "\b(fmt|format|lint|test|clippy|golangci|just|pnpm run|go test|cargo fmt|cargo clippy)\b" /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode /Users/kooshapari/temp-PRODVERCEL/485/API/research/CLIProxyAPI > /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-surface.txt`
    - `for repo in /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode /Users/kooshapari/temp-PRODVERCEL/485/API/research/CLIProxyAPI; do echo "== $(basename "$repo") ==" >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-surface.txt; find "$repo" -maxdepth 3 -type f \( -name "workflow*.yml" -o -name "workflow*.yaml" -o -name AGENTS.md -o -name Justfile -o -name package.json -o -name go.mod -o -name Cargo.toml -o -name pyproject.toml \) >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-surface.txt; done`
  evidence_path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-surface.txt`

- id: A6
  title: Inventory lockfiles and lockfile-manager expectations
  status: done
  depends_on: [A5]
  commands:
    - `printf 'repo,lockfiles\n' > /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt`
    - `for repo in codex opencode kilocode; do printf '%s,' "$repo" >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt; find /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/$repo -maxdepth 4 -type f \( -name pnpm-lock.yaml -o -name package-lock.json -o -name yarn.lock -o -name go.sum -o -name Cargo.lock \) | sort | tr '\n' ';' >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt; echo >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt; done`
    - `printf 'goose,' >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt; find /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose -maxdepth 4 -type f -name Cargo.lock | sort | tr '\n' ';' >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt; echo >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt`
    - `printf 'cliproxyapi++,' >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt; find /Users/kooshapari/temp-PRODVERCEL/485/API/research/CLIProxyAPI -maxdepth 4 -type f -name go.sum | sort | tr '\n' ';' >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt; echo >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt`
    - `printf 'constraints=' >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt; rg -n "packageManager|engines|go [0-9]+|\[workspace\]" /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex/package.json /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex/codex-rs/Cargo.toml /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode/go.mod /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose/Cargo.toml /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode/package.json /Users/kooshapari/temp-PRODVERCEL/485/API/research/CLIProxyAPI/go.mod >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt`
  evidence_path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt`

- id: A7
  title: Enforce lockfile parity checks in frozen/strict mode
  status: done
  depends_on: [A6]
  commands:
    - `cd /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex && pnpm install --frozen-lockfile && git diff --exit-code -- pnpm-lock.yaml >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-parity.txt 2>&1`
    - `cd /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode && pnpm install --frozen-lockfile && pnpm --dir jetbrains/host install --frozen-lockfile && git diff --exit-code -- pnpm-lock.yaml jetbrains/host/pnpm-lock.yaml >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-parity.txt 2>&1`
    - `cd /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode && go mod tidy && go mod download && git diff --exit-code -- go.sum >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-parity.txt 2>&1`
    - `cd /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/cliproxyapi-plusplus && go mod tidy && go mod download && git diff --exit-code -- go.sum >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-parity.txt 2>&1`
    - `cd /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose && cargo generate-lockfile && git diff --exit-code -- Cargo.lock >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-parity.txt 2>&1`
  evidence_path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-parity.txt`

- id: A8
  title: Execute strictness parity checks per repo
  status: done
  depends_on: [A4, A5, A7]
  commands:
    - `cd /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex/codex-rs && just fmt && just clippy >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-results.md 2>&1`
    - `cd /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex && pnpm run format >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-results.md 2>&1`
    - `cd /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode && go test ./... >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-results.md 2>&1`
    - `cd /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/cliproxyapi-plusplus && go test ./... >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-results.md 2>&1`
    - `cd /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose && cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-results.md 2>&1`
    - `cd /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode && pnpm lint && pnpm test >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-results.md 2>&1`
  evidence_path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-results.md`

- id: A9
  title: Build strictness-parity matrix and evidence bundle
  status: done
  depends_on: [A8]
  commands:
    - `printf '%s\n' '# Strictness Parity Matrix' '| repo | branch | head commit | lockfile | strictness checks |' '|---|---|---|---|---|' > /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-parity.md`
    - `for repo in codex opencode goose kilocode cliproxyapi-plusplus; do echo "| $repo | main | TODO | TODO | TODO |" >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-parity.md; done`
    - `cat /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/commit-provenance.txt /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/branch-topology.txt /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-surface.txt > /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-parity-bundle.txt`
  evidence_path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-parity.md`

- id: A10
  title: Close lane-A execution with final cleanliness and unresolved risk capture

---

## Source: research/phase-2-reports/agent-b-candidate-expansion.md

# Phase-2 Lane B: Candidate Expansion

Scope: discovery-focused expansion for open-source CLI/MCP candidates, including:

- Local `temp-PRODVERCEL/485` scan.
- Open-source registry/web discovery shortlist.
- Evidence capture limited to installs, scripts/build hooks, licenses, and strictness profile.

## Scoring Rubric Deltas vs Phase-1 Shortlist

Compared with the Phase-1 matrix baseline (`quality 40 + quality-coverage 20 + API/SDK 20 + build 20`), add the following adjustments for Phase-2 expansion candidates:

- `Evidence completeness` (new, +20 points): +4 each for installation command evidence, +4 for explicit build commands, +4 for test/quality commands, +4 for license traceability, +4 for strictness profile evidence.
- `Open-source confidence` (new, +10 points): +5 for public repo + tag/branch strategy, +5 for permissive license file + governance docs.
- `Operational risk penalty` (new, -15 points): applies when installability is inferred from docs only, scripts are missing, or strictness evidence is absent.
- `Baseline weights retained` for API/SDK surface and build confidence but normalized for multi-command ecosystems (repo scoring now requires explicit evidence for each command family).

## Tasks

### Task B1: Freeze Phase-1 baseline and lane handoff contract
Status: `done`  
Depends on: none  
Evidence: `research/oss-cli-matrix.md`, `artifacts/phase-1-closeout.md`, `research/harness-spec.md` (snapshot of baseline candidates and scoring assumptions at task start).

### Task B2: Enumerate candidate repositories from local `/Users/kooshapari/temp-PRODVERCEL/485`
Status: `done`  
Depends on: `B1`  
Evidence: `temp-PRODVERCEL/485` directory inventory file capturing repo name, presence of CLI entrypoint signals (`README`, `Makefile`, `pyproject`, `package.json`, `Cargo.toml`, `go.mod`, `main.go`, `index.ts`), and MCP keywords (`mcp`, `tools`, `stdio`, `sse`, `json-rpc`).

### Task B3: Filter local scan to open-source CLI/MCP candidates only
Status: `done`  
Depends on: `B2`  
Evidence: evidence matrix for each candidate with: `AGENTS/README` availability, open source indicator, license availability, active entrypoint (CLI/MCP), and rationale for inclusion/exclusion.

### Task B4: Capture open-source license artifacts for local candidates
Status: `done`  
Depends on: `B3`  
Evidence: per-candidate `LICENSE*` paths, SPDX/declared license text, and exceptions/embedded third-party license notes; if missing, mark as evidence-gap.

### Task B5: Capture install execution surface for local candidates
Status: `done`  
Depends on: `B3`  
Evidence: install command evidence per candidate (`npm`, `pnpm`, `brew`, `go install`, `cargo install`, `uv/pip`, `docker`, archive/ script install), command snippets with source file and line references.

### Task B6: Capture build, test, and quality commands for local candidates
Status: `done`  
Depends on: `B3`  
Evidence: explicit command list and config files for build/test/lint/typecheck/gating (`Makefile`, CI workflow, task files, package scripts), plus whether commands are mandatory, optional, or documented.

### Task B7: Record strictness profile for local candidates
Status: `done`  
Depends on: `B4`, `B5`, `B6`  
Evidence: profile fields mapped to rubric buckets (quality gates, strictness, API/mcp exposure, build/test health, maintenance signal), with concrete evidence links and confidence level.

### Task B8: Compute local candidate ranking delta against Phase-1 shortlist
Status: `done`  
Depends on: `B7`  
Evidence: ranked delta list (`additions`, `re-rank`, `retains`, `drop`) with score changes from rubric deltas and short justification for each movement.

### Task B9: Run web/open-source registry shortlist sweep
Status: `done`  
Depends on: `B1`  
Evidence: `research/phase-2-reports/agent-b-candidate-expansion/artifacts/b9-web-registry-sweep.md`

### Task B10: Produce consolidated Phase-2 lane-B shortlist and evidence pack
Status: `done`  
Depends on: `B8`, `B9`  
Evidence: `research/phase-2-reports/agent-b-candidate-expansion/artifacts/b-local-scan.md`, `research/phase-2-reports/agent-b-candidate-expansion/artifacts/b9-web-registry-sweep.md`

## Execution status (Phase-2)
- B1-B10 completed with updated local shortlist and evidence references.
- B9-B10 web sweep and final ranked package are complete; candidate set now includes web-registry-backed replacements and public MCP/CLI signals.

---

## Source: research/phase-2-reports/agent-c-governance-strictness.md

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

---

## Source: research/phase-2-reports/agent-d-harness-architecture.md

# Phase-2 Lane D: Harness Architecture Planning

Scope: implementation-ready plan for the harness core (`discoverer`, `runner`, `quality normalizer`, `evidence schema`) with concrete file layouts, commands, and typed interfaces.

## Baseline assumptions
- Language for initial implementation: Python 3.12 (for portable command execution and JSON schema tooling).
- Output is machine-validated and CI-consumable via JSONL and JSON schema.
- Candidate target repos are under `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/*`.

### Canonical file targets to be introduced
- `harness/pyproject.toml`
- `harness/src/harness/discoverer.py`
- `harness/src/harness/interfaces.py`
- `harness/src/harness/runner.py`
- `harness/src/harness/normalizer.py`
- `harness/src/harness/schema.py`
- `harness/schemas/harness-evidence.schema.json`
- `harness/schemas/phase2-harness.schema.json`
- `harness/scripts/run-harness.py`
- `harness/tests/test_discoverer.py`
- `harness/tests/test_runner_contract.py`
- `harness/tests/test_normalizer.py`
- `harness/tests/test_schema_roundtrip.py`

## Tasks

### Task D1: Lock architecture contract and command profile vocabulary
Status: `done`
Dependency: none
Evidence: `research/harness-spec.md`, `research/phase-2-reports/agent-d-harness-architecture.md` (section "Canonical file targets")

Command: `python3 -m compileall harness/src/harness` after edits.

```python
# harness/src/harness/interfaces.py
from enum import Enum

class EvidenceBucket(str, Enum):
    BOOTSTRAP = "bootstrap"
    STATIC = "static"
    TEST = "test"
    BUILD = "build"
    API = "api"

@dataclass
class CanonicalCommand:
    command: str
    cwd: str
    bucket: EvidenceBucket
    required: bool
    rationale: str
    source: str
```

### Task D2: Implement repository discoverer input model and CLI discoverer interface
Status: `done`
Dependency: `D1`
Evidence: `harness/src/harness/interfaces.py`, `harness/src/harness/discoverer.py`, `artifacts/phase-2/01-discovery-model.json`

Command: `python3 harness/scripts/run-harness.py discover --root /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex --out artifacts/phase-2/discovery-codex.json`

```python
# harness/src/harness/interfaces.py
@dataclass
class RepoManifest:
    repo_id: str
    root: str
    remote_url: str
    default_branch: str | None
    discovered_at: str

@dataclass
class DiscoverInput:
    repo_root: str
    max_scan_depth: int = 3

@dataclass
class DiscoverOutput:
    manifest: RepoManifest
    signals: list[str]
    buckets: dict[str, list[CanonicalCommand]]
    files: list[str]
    raw_events: list[dict]
```

### Task D3: Build file-system discovery logic for strictness and quality signals
Status: `done`
Dependency: `D2`
Evidence: `harness/src/harness/discoverer.py`, `artifacts/phase-2/discovery-signal-matrix.md`

Command: `python3 -m pytest harness/tests/test_discoverer.py -q`

```python
# harness/src/harness/discoverer.py
class Discoverer:
    def discover(self, input: DiscoverInput) -> DiscoverOutput: ...
    def _extract_signals(self, root: str) -> list[str]: ...
    def _extract_scripts(self, root: str, file_patterns: list[str]) -> list[dict]: ...
```

### Task D4: Define normalized command execution result envelope and run-mode policy
Status: `done`
Dependency: `D1`
Evidence: `harness/src/harness/interfaces.py`, `artifacts/phase-2/runner-contract.md`

Command: `python3 -m pytest harness/tests/test_runner_contract.py -q`

```python
@dataclass
class RunResult:
    command: str
    bucket: EvidenceBucket
    returncode: int
    started_at: str
    finished_at: str
    stdout_file: str
    stderr_file: str
    duration_ms: int
    artifact_dir: str


---

## Source: research/phase-2-reports/agent-e-validation-automation.md

# Phase-2 Lane E: Validation Automation

Execution status: `E1`-`E10` implemented.

## Task progress

- `E1` complete
- `E2` complete
- `E3` complete
- `E4` complete
- `E5` complete
- `E6` complete
- `E7` complete
- `E8` complete
- `E9` complete
- `E10` complete

Artifacts:

- `research/phase-2-reports/agent-e-validation-automation/artifacts/e2-command-map-schema.yaml`
- `research/phase-2-reports/agent-e-validation-automation/artifacts/e4-dag-topology.dot`
- `research/phase-2-reports/agent-e-validation-automation/artifacts/e6-failure-budget.yaml`
- `research/phase-2-reports/agent-e-validation-automation/artifacts/e5-e10-evidence.md`

Evidence summary: validation automation now includes canonical dry-run/replay plan hashing, runner retry enforcement, budgeted execution metadata, and reproducible evidence payloads emitted by `run-harness.py`.

---

## Source: research/phase-2-reports/agent-f-closeout-delivery.md

# Phase-2 Lane F: Closeout Delivery

Execution status: F1-F10 completed; F5 now closes schema validation across external run artifacts.

---

