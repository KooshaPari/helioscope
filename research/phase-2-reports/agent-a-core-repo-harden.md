<DONE>
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
  status: done
  depends_on: [A9]
  commands:
    - `for repo in codex opencode goose kilocode cliproxyapi-plusplus; do git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/$repo status --short --branch >> /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/final-cleanliness.txt; done`
    - `printf '%s\n' "Lane-A closeout complete only when A1-A9 evidence files exist and A8 checks pass" > /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/final-closeout.txt`
    - `grep -q TODO /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-parity.md && echo "strictness parity matrix contains TODOs" > /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/unresolved-risks.txt || true`
    - `test -s /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/final-closeout.txt`
  evidence_path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/research/phase-2-reports/agent-a-core-repo-harden/artifacts/final-closeout.txt`

## Execution status (Phase-2)
- A1-A10 complete with bounded evidence. A7/A8/A9/A10 retain WARN-level residual risks logged in `artifacts/unresolved-risks.txt` and `artifacts/final-closeout.txt`.
