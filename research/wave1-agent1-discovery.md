<DONE>
# Wave 1 / Agent 1 Discovery

Scope: local clone/repo discovery for `codex`, `opencode`, `goose`, `kilo`.
Timestamp: 2026-02-22 (workspace local snapshot)

## Target Repo Discovery (Current)

### 1) `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones`

- `codex`
  - Path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex`
  - Git: present
  - Status: **broken (no refs)**
  - `HEAD`: `ref: refs/heads/.invalid`
  - Branch: unusable (`detached_or_broken`)
  - `git status --short` count: `0`
  - Remote: `https://github.com/openai/codex.git`
  - Last commit: unavailable until clone is repaired

- `opencode`
  - Path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode`
  - Git: healthy
  - Branch: `main`
  - `git status --short` count: `0`
  - Remote: `https://github.com/opencode-ai/opencode.git`
  - Last commit: `73ee493` (`docs(readme): update archive note`, `2025-09-17`)

- `goose`
  - Path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose`
  - Git: present
  - Status: **broken (no refs)**
  - `HEAD`: `ref: refs/heads/.invalid`
  - Branch: unusable (`detached_or_broken`)
  - `git status --short` count: `0`
  - Remote: `https://github.com/block/goose.git`
  - Last commit: unavailable until clone is repaired

- `kilocode` (alias `kilo`)
  - Path: `/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode`
  - Status: **missing**

### 2) Additional local matches (same target stack)

- `API/codex-upstream`
  - Branch: `main`
  - Remote: `https://github.com/openai/codex.git`
  - Last commit: `2d6757430` (`plan mode prompt (#10308)`, `2026-01-31`)
  - `git status --short` count: `0`

- `API/research/opencode`
  - Branch: `dev`
  - Remote: `https://github.com/sst/opencode.git`
  - Last commit: `070ced0b3` (`fix: revert hook try/catch that surpressed errors`, `2025-12-10`)
  - `git status --short` count: `0`

- `API/research/goose`
  - Branch: `main`
  - Remote: `https://github.com/block/goose.git`
  - Last commit: `66d075050ed` (`blog: typo fixes (#5896)`, `2025-11-26`)
  - `git status --short`: 6 changes (local edits/untracked)

- `API/research/kilocode`
  - Branch: `main`
  - Remote: `https://github.com/Kilo-Org/kilocode.git`
  - Last commit: `1440d1986d` (`Merge pull request #4300 ...`, `2025-12-09`)
  - `git status --short` count: `0`

## Toolchain discovery

- `codex` (openai)
  - Root `package.json` says `node >=22`, `pnpm >=10.28.0` and `packageManager: pnpm@10.28.2+sha512...`
  - Rust workspace detected: many `codex-rs/*/Cargo.toml` manifests
  - `pnpm-lock.yaml` present

- `opencode` (`API/research/opencode` and `heliosHarness/clones/opencode`)
  - `go.mod` exists with `go 1.24.0` (Go toolchain)
  - No project-level `package.json` at `heliosHarness/clones/opencode`

- `goose` (`API/research/goose`)
  - Root `Cargo.toml`: Rust workspace with `resolver = "2"`, `edition = "2021"`
  - UI/docs tooling present with `package.json` and Node workflows (no single repo-level version pin seen)

- `kilo` (`API/research/kilocode`)
  - Root `package.json`: `packageManager: pnpm@10.8.1`, `engines.node: 20.19.2`
  - `.tool-versions`: `nodejs 20.19.2`
  - `.nvmrc`: `v20.19.2`

## Clone checklist

1. Repair broken/empty existing clones under `heliosHarness/clones`:
   - `rm -rf /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex`
   - `rm -rf /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose`
2. Re-clone targets for Wave 1 prep (into `heliosHarness/clones`):
   - `git clone https://github.com/openai/codex.git /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/codex`
   - `git clone https://github.com/opencode-ai/opencode.git /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode`
   - `git clone https://github.com/block/goose.git /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/goose`
   - `git clone https://github.com/Kilo-Org/kilocode.git /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode`
3. Verification commands:
   - `git -C <repo> remote -v`
   - `git -C <repo> status --short`
   - `git -C <repo> log -1 --oneline`
