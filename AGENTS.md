# Documentation Organization Governance

**CRITICAL**: This document defines the strict governance structure for organizing markdown files in the repository.

## Root-Level Files (Keep in Root)

Only these files should remain in the project root:
- `README.md` - Main project documentation
- `CHANGELOG.md` - Project changelog
- `AGENTS.md` - This governance file (AI agent instructions)
- `CLAUDE.md` / `claude.md` - Claude-specific instructions
- `00_START_HERE.md` - Getting started guide (if applicable)

## Documentation Structure

All other `.md` files must be organized in `docs/` subdirectories:

```
docs/
├── guides/              # Implementation guides and how-tos
│   └── quick-start/     # Quick start guides
├── reports/             # Completion reports, summaries, status reports
├── research/            # Research summaries, indexes, analysis
├── reference/           # Quick references, API references
└── checklists/          # Implementation checklists, verification lists
```

## File Organization Rules

**When creating or moving documentation:**

### 1. Quick Starts → `docs/guides/quick-start/`
- Files matching `*QUICK_START*.md` or `*QUICKSTART*.md`
- Examples: `GRAPH_OPTIMIZATION_QUICK_START.md`, `AUTH_ROUTES_QUICK_START.md`

### 2. Quick References → `docs/reference/`
- Files matching `*QUICK_REFERENCE*.md` or `*QUICK_REF*.md`
- Examples: `NATS_QUICK_REFERENCE.md`, `CLI_QUICK_REFERENCE.md`

### 3. Implementation Guides → `docs/guides/`
- Files matching `*IMPLEMENTATION_GUIDE*.md` or `*GUIDE*.md`
- General implementation documentation
- Examples: `API_IMPLEMENTATION_GUIDE.md`, `DEPLOYMENT_GUIDE.md`

### 4. Completion Reports → `docs/reports/`
- Files matching `*COMPLETE*.md`, `*COMPLETION*.md`, `*SUMMARY*.md`, `*REPORT*.md`
- Phase completion files (`PHASE_*.md`)
- Test-related reports (`*TEST*.md`)
- Examples: `IMPLEMENTATION_COMPLETE.md`, `PHASE_1_COMPLETION_SUMMARY.md`

### 5. Research Files → `docs/research/`
- Files matching `*RESEARCH*.md` or `*INDEX*.md`
- Examples: `RESEARCH_SUMMARY.md`, `API_TESTS_INDEX.md`

### 6. Checklists → `docs/checklists/`
- Files matching `*CHECKLIST*.md`
- Examples: `IMPLEMENTATION_CHECKLIST.md`, `MIGRATION_CHECKLIST.md`

## Optionality and failure behavior

**Project stance (required):** **Require** dependencies where they belong; **require** clear, loud failures—no silent or “graceful” degradation.

- **Force requirement where it belongs.** Do not make dependencies “optional” just to avoid startup or runtime failure. If a service or config is required for correctness (e.g. go-backend, temporal-host, database), treat it as required and fail when it is missing or down.
- **Fail clearly, not silently.** You **must** use explicit failures (preflight failed, runtime error)—not continuing with reduced functionality, logging-only warnings, or hiding errors. Users and operators **must** see *what* failed (e.g. named items: `go-backend; temporal-host`) and that the process did not silently degrade.
- **Graceful in other ways.** Be “graceful” only via: retries with visible feedback (e.g. “Waiting for X… (2/6)”); error messages that list each failing item (semicolon- or newline-separated, not one vague blob); actionable messages and non-obscure stack traces. Do *not* use optionality or silent fallbacks as a substitute for fixing or starting the real dependency.

**When working as an AI agent:** Follow this stance when adding preflight checks, startup logic, or error handling; do not suggest making required dependencies optional to “fix” failures.

---

## AI Agent Instructions

**Planner agents (no code in docs/plans):** Planner agents (PM, Analyst, Architect, SM, TEA, UX Designer, Tech Writer, BMad Master) must **never write code** in documentation or plans. Equip engineer agents or others to create code or act on docs; write specs, criteria, and handoffs—not implementation. Avoid stuffing docs or plans with code where not relevant; prefer references, file paths, or brief pseudocode when necessary.

**Subagent swarm (async orchestration):** If you have subagent/swarm capabilities, call task agents **async** so you are reawoken as each completes; spawn more agents or do more work then. Run a swarm: break down and isolate tasks well; **max 50 concurrent task agents at a time** (not 5). Work in between (plan, monitor); when idle you are reawoken on each completion to re-eval, spawn more, or consolidate.

**Subagent deployment: native CLI + thegent:** Use **both** native subagents (when present in CLI tools) and thegent subagents. **Native subagents:** Cursor Agent, Gemini CLI, Codex subagent, Copilot CLI, Claude Code—invoke these directly when the tool exposes subagent/agent capabilities (e.g. `cursor agent`, `codex-subagent`). **Thegent subagents:** `thegent run gemini`, `thegent run droid`, `thegent run cursor`, `thegent run codex`, `thegent run copilot`, `thegent run claude`—use thegent for unified orchestration, model passthrough, and droid dispatch. Prefer native when you need tool-specific behavior; use thegent for cross-provider orchestration, droids, or when model/env control is needed. See `docs/reference/THGENT_QUICK_REFERENCE.md`.

**Codex CLI idle/poll behavior:** When wrapping up a task, end with a long wait + poll loop to simulate idle. Poll at a predictable 30s interval, and structure it so user interruption stops only the sleep/wait (not child agents), allowing the user to steer without interrupting subagents.

**Phased WBS and plans with DAGs:** When generating plans, roadmaps, or implementation breakdowns, use a **phased Work Breakdown Structure (WBS)** and model **dependencies as a DAG** (directed acyclic graph). Structure work into ordered phases (e.g. Phase 1: Discovery/Scope, Phase 2: Design, Phase 3: Build, Phase 4: Test/Validate, Phase 5: Deploy/Handoff). Tasks must have explicit predecessors with no cycles; list dependencies so execution order is unambiguous. Prefer a phased WBS plus a dependency list or table: **Phase | Task ID | Description | Depends On**. Planner agents must use phased WBS and DAG-style dependencies in plans and PRDs so implementers and PMs can schedule and parallelize correctly.

**Agent-led environment; no user-handoff tasks:** Assume **agent-driven, agent-led** execution. User and external humans do not perform plan steps—only prompts and basic elicitation. **Never** put in plans: "Schedule external security audit", "Stakeholder Presentation", "Team Kickoff: Assign owners", "Human checkpoint", "Get approval from X", or any task that requires a human to do work. Agents produce deliverables (e.g. presentation doc, owner manifest); agents own decomposition, execution, and handoffs. **Timescales:** Use **aggressive** estimates in agent terms only (tool calls, parallel subagents, wall clock in minutes). Forbidden: "2 days", "schedule X", "assign owners". Use: "N tool calls", "~M min wall clock". See CLAUDE.md for the full timescale mapping.

**When working as an AI agent:**

- **NEVER** create `.md` files in the project root (except the allowed files above)
- **ALWAYS** place new documentation in the appropriate `docs/` subdirectory
- **VERIFY** file location before creating documentation
- **MOVE** misplaced files to correct subdirectories if found
- **REFERENCE** this structure when users ask about documentation organization
- **DO NOT** ask for permission or priority order; decide on your own, run needed commands, and proceed with the most critical path.

**Native services over Docker; local OSS/free only:** Prefer **native** service runs (e.g. `make install-native`, process-compose) over Docker for dev. Use Docker only when native is not feasible. **Strictly prefer local, OSS, and free**—do not recommend paid online services when local or free alternatives exist; prefer self-hosted, open-source, or free-tier options.

**See also:** For context management, delegation table, dev environment, and package manager instructions see `CLAUDE.md` (or `claude.md`). For IDE-specific agent activation see `.bmad/docs/` (e.g. `claude-code-instructions.md`, `gemini-instructions.md`, `cursor-instructions.md`).

## Maintenance

- Use the `organize_docs.sh` script to reorganize misplaced files
- Keep root directory clean and organized
- Review and reorganize quarterly to maintain structure
- Update this governance document as the project evolves


## Multi-Actor Coordination
- **Command Debouncing**: High-impact commands (`make lint`, `make test`, `make quality`, `make validate`) MUST use `smart-command.sh` (via Makefile/Taskfile) to prevent conflicts between multiple root/subagents.
- **Shared Service Awareness**: `process-compose` is the primary orchestrator. Use its CLI/API (e.g., `make dev-status`, `make dev-restart`) instead of raw scripts to ensure global visibility.
- **Graceful Service Interaction**: Infrastructure and app services use "if-not-running" wrappers to allow multiple actors to share a single set of healthy processes. DO NOT force-kill shared resources.
- **Lock Files**: Active command locks are stored in `.process-compose/locks/`. Always check for existing locks before running heavy tasks.
- **Unified Logging**: Read aggregated logs from `.process-compose/process-compose.log`.

## Opinionated Quality Enforcement
- We want opinionated rules that enforce opinionated styling to a strict degree.
- This is an exclusively agent/vibecoded project; programmatic enforcement must guard against bad quality and antipatterns.
- Rather than disables or ignores, fix code properly.

## Lint Violation Governance

**Before filing bugs or writing code for lint violations:**

1. **Verify true violations**: Check if flagged issues are truly unused, unimplemented, or can be refactored without losing functionality
2. **Never use ignorers**: Never add `//nolint:lintname`, `//lint-ignore`, or skip linter configurations to silence violations
3. **Fix properly**: Address root causes—extract functions, constants, reduce complexity, not suppression
4. **Test coverage preserved**: Ensure all fixes maintain existing test coverage; do not disable tests to pass linters

**Violation handling priorities:**

| Category | Action |
|----------|--------|
| `revive` (unused params in mocks) | Rename to `_paramName` if intentionally unused |
| `goconst` (repeated strings) | Extract to named constants |
| `mnd` (magic numbers) | Extract to named constants with units |
| `gocognit` (complexity) | Extract helper functions, reduce nesting |
| `funlen` (long functions) | Split into focused helper functions |
| `gochecknoglobals` | Validate necessity; convert to singletons if needed |
| `gosec` (security) | Fix immediately; no exceptions |

**Subagent delegation for lint fixes:**
- Group 1-3 related files per subagent by violation type
- Production code takes priority over test code for complexity/security
- Test code refactoring (funlen) can be delegated more aggressively

<!-- PHENOTYPE_GOVERNANCE_OVERLAY_V1 -->
## Phenotype Governance Overlay v1

- Enforce `TDD + BDD + SDD` for all feature and workflow changes.
- Enforce `Hexagonal + Clean + SOLID` boundaries by default.
- Favor explicit failures over silent degradation; required dependencies must fail clearly when unavailable.
- Keep local hot paths deterministic and low-latency; place distributed workflow logic behind durable orchestration boundaries.
- Require policy gating, auditability, and traceable correlation IDs for agent and workflow actions.
- Document architectural and protocol decisions before broad rollout changes.


## Bot Review Retrigger and Rate-Limit Governance

- Retrigger commands:
  - CodeRabbit: `@coderabbitai full review`
  - Gemini Code Assist: `@gemini-code-assist review` (fallback: `/gemini review`)
- Rate-limit contract:
  - Maximum one retrigger per bot per PR every 15 minutes.
  - Before triggering, check latest PR comments for existing trigger markers and bot quota/rate-limit responses.
  - If rate-limited, queue the retry for the later of 15 minutes or bot-provided retry time.
  - After two consecutive rate-limit responses for the same bot/PR, stop auto-retries and post queued status with next attempt time.
- Tracking marker required in PR comments for each trigger:
  - `bot-review-trigger: <bot> <iso8601-time> <reason>`


## Review Bot Governance

- Keep CodeRabbit PR blocking at the lowest level in `.coderabbit.yaml`: `pr_validation.block_on.severity: info`.
- Keep Gemini Code Assist severity at the lowest level in `.gemini/config.yaml`: `code_review.comment_severity_threshold: LOW`.
- Retrigger commands:
  - CodeRabbit: comment `@coderabbitai full review` on the PR.
  - Gemini Code Assist (when enabled in the repo): comment `@gemini-code-assist review` on the PR.
  - If comment-trigger is unavailable, retrigger both bots by pushing a no-op commit to the PR branch.
- Rate-limit discipline:
  - Use a FIFO queue for retriggers (oldest pending PR first).
  - Minimum spacing: one retrigger comment every 120 seconds per repo.
  - On rate-limit response, stop sending new triggers in that repo, wait 15 minutes, then resume queue processing.
  - Do not post duplicate trigger comments while a prior trigger is pending.

- Update docs/examples when API behavior changes (at minimum `app-server/README.md`).
- Regenerate schema fixtures when API shapes change:
  `just write-app-server-schema`
  (and `just write-app-server-schema --experimental` when experimental API fixtures are affected).
- Validate with `cargo test -p codex-app-server-protocol`.
- Avoid boilerplate tests that only assert experimental field markers for individual
  request fields in `common.rs`; rely on schema generation/tests and behavioral coverage instead.
