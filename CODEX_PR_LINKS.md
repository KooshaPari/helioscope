# OpenAI Codex - Unmerged PRs: Direct Links & Status

**Last Updated:** 2026-02-28
**Total Open PRs:** 200+
**Total Recent Closed PRs (last 60 days):** 200+

## HIGH PRIORITY FEATURES

| # | Title | Link | Status | Size | Difficulty | Notes |
|---|-------|------|--------|------|------------|-------|
| 12523 | Markdown Table Rendering | [PR](https://github.com/openai/codex/pull/12523) | OPEN | 5,832+ | MEDIUM | Tables with Unicode borders + terminal resize reflow |
| 12864 | Plugin System | [PR](https://github.com/openai/codex/pull/12864) | OPEN | 1,380+ | LOW | First-class plugins via config.toml |
| 12703 | Model Toggle (Ctrl+O) v2 | [PR](https://github.com/openai/codex/pull/12703) | CLOSED | 950+ | LOW | Persisted config pairs; recommended version ★ |
| 11865 | Vim Mode | [PR](https://github.com/openai/codex/pull/11865) | OPEN | 795+ | MEDIUM | /vim toggle for modal editing |
| 13100 | MCP Transport Recovery | [PR](https://github.com/openai/codex/pull/13100) | CLOSED | 482+ | LOW | Fixes browser MCP wedges (broken pipe) |
| 12951 | OSC-52 Copy Support | [PR](https://github.com/openai/codex/pull/12951) | OPEN | 210+ | LOW | /copy works in tmux/SSH |
| 12850 | Realtime Audio Device Picker | [PR](https://github.com/openai/codex/pull/12850) | OPEN | 582+ | MEDIUM | Select input/output devices |
| 12849 | Realtime Audio Device Config | [PR](https://github.com/openai/codex/pull/12849) | OPEN | 299+ | MEDIUM | Persist audio device prefs |

## MEDIUM PRIORITY FEATURES

| # | Title | Link | Status | Size | Notes |
|---|-------|------|--------|------|-------|
| 12803 | Live Skill Refresh Notifications | [PR](https://github.com/openai/codex/pull/12803) | OPEN | 351+ | Notify on skill updates without restart |
| 13073 | Quick Model Picker Highlight | [PR](https://github.com/openai/codex/pull/13073) | OPEN | 161+ | Stronger visual feedback in picker |
| 13027 | Skill Disable Respects Config | [PR](https://github.com/openai/codex/pull/13027) | OPEN | 155+ | Config-level skill control |
| 12715 | App-Server v2 Realtime API | [PR](https://github.com/openai/codex/pull/12715) | OPEN | 1,706+ | Realtime conversation protocol |
| 12610 | Fork with Turn Picker UI | [PR](https://github.com/openai/codex/pull/12610) | OPEN | 779+ | Improve fork UX |
| 13065 | host_executable() Preflight | [PR](https://github.com/openai/codex/pull/13065) | OPEN | 175+ | Exec policy improvements |
| 13045 | host_executable() Support | [PR](https://github.com/openai/codex/pull/13045) | OPEN | 900+ | Path mapping for basename rules |
| 13064 | host_executable() Adoption | [PR](https://github.com/openai/codex/pull/13064) | OPEN | 310+ | Zsh-fork integration |
| 12687 | Realtime Collaboration Mode (TUI) | [PR](https://github.com/openai/codex/pull/12687) | CLOSED | 800+ | Audio streaming + transcription UI |
| 12726 | request_user_input Outside Plan | [PR](https://github.com/openai/codex/pull/12726) | CLOSED | 276+ | Expand autonomy options |
| 12739 | Exec Quiet Mode | [PR](https://github.com/openai/codex/pull/12739) | CLOSED | 183+ | Suppress scaffolding for scripting |
| 13037 | Theme-Aware Diff Backgrounds | [PR](https://github.com/openai/codex/pull/13037) | OPEN | 586+ | Better diff rendering |
| 11549 | Permissions Customization UI | [PR](https://github.com/openai/codex/pull/11549) | OPEN | 203+ | /permissions custom option |

## LOWER PRIORITY / NICE-TO-HAVE

| # | Title | Link | Status | Notes |
|---|-------|------|--------|-------|
| 13111 | Clear Terminal on /new | [PR](https://github.com/openai/codex/pull/13111) | CLOSED | 2 lines; trivial UX win |
| 12780 | Model Toggle (Ctrl+O) v1 | [PR](https://github.com/openai/codex/pull/12780) | CLOSED | Session-local, no persistence |
| 12653 | Consolidation Prompt Refinements | [PR](https://github.com/openai/codex/pull/12653) | OPEN | Tighter memory consolidation |
| 12653 | Memory Staleness Tuning | [PR](https://github.com/openai/codex/pull/13088) | OPEN | Tune memory read-path |
| 12936 | Background Job for Req Cache Refresh | [PR](https://github.com/openai/codex/pull/12936) | OPEN | Auto-refresh requirements cache |
| 12612 | Unify Rollout Reconstruction | [PR](https://github.com/openai/codex/pull/12612) | OPEN | 1,796+ lines; complex refactor |
| 12334 | /title Command (Terminal Title Config) | [PR](https://github.com/openai/codex/pull/12334) | OPEN | 1,179+ | Configure terminal window title |
| 12732 | DynamicToolCall Item | [PR](https://github.com/openai/codex/pull/12732) | OPEN | 2,436+ | App-server protocol enhancement |
| 12297 | SDK MVP: Collaboration Mode | [PR](https://github.com/openai/codex/pull/12297) | OPEN | 548+ | SDK support for request_user_input |
| 12192 | MCP Elicitations Support | [PR](https://github.com/openai/codex/pull/12192) | OPEN | 1,432+ | Experimental MCP elicitation flow |

## EXTERNAL CONTRIBUTOR CONTEXT

These PRs were authored by community members before OpenAI's 2026-02-28 policy change to "internal-only contributions."

**Top External Contributors to Review:**

- **Felipe Coury** (@fcoury)
  - PR #12523 (Markdown tables)
  - PR #12703 (Model toggle v2)
  - PR #12780 (Model toggle v1)
  - Consistently high-quality, well-tested work

- **Dylan Hurd** (@drhurd, also @dylan-hurd-oai internally)
  - PR #11865 (Vim mode)
  - PR #11549 (Permissions UI)
  - PR #12521 (Rules env context)
  - PR #12504 (Sandbox policy custom)
  - Deep TUI and security expertise

- **Michael Bolin** (@bolinfest)
  - PR #13065 (host_executable preflight)
  - PR #13045 (host_executable support)
  - PR #13064 (host_executable adoption)
  - Multiple exec policy improvements
  - Infrastructure focus; excellent test coverage

- **Ahmed Ibrahim** (@aibrahim-oai, internal contributor)
  - PR #12687 (Realtime collaboration)
  - PR #12850 (Audio device picker)
  - PR #12849 (Audio device config)
  - Voice/realtime expertise

- **Skylar Graika** (@swordfish444)
  - PR #13100 (MCP transport recovery)
  - Well-tested reliability improvement

- **Adam Koszek** (@wkoszek)
  - PR #12739 (Exec quiet mode)
  - PR #12736 (Prompt flag -P)
  - Scripting/automation focus

## STATUS LEGEND

- **OPEN**: Still accepting contributions (but external contributors now rejected)
- **CLOSED**: Closed without merging (likely due to 2026-02-28 policy change or author preference)
- **MERGED**: Successfully merged to main (reference implementations)

## USAGE NOTES

### Finding Code to Adapt

1. **Markdown Tables**: View files changed in PR #12523
   - Core: `codex-rs/codex-tui/src/streaming/` directory
   - Key modules: `table_detect`, `markdown`, `streaming/controller`

2. **Plugin System**: PR #12864
   - Core: `codex-rs/core/src/plugin/` directory
   - Pattern matches existing `SkillsManager` design

3. **Model Toggle**: PR #12703
   - File: `codex-rs/codex-tui/src/chat_widget/` 
   - Look for `ModelHistoryEntry`, `model_toggle_pair` config

4. **Vim Mode**: PR #11865
   - File: `codex-rs/codex-tui/src/input/`
   - Look for input handler composition

5. **MCP Recovery**: PR #13100
   - File: `codex-rs/core/src/mcp_connection_manager.rs`
   - Function: `is_transport_closed_error()`

### Git Cherry-Pick

Note: These PRs are against upstream `main` as of different dates. Before cherry-picking:

1. Check base commit date in PR description
2. Resolve conflicts with current helios-cli state
3. Adapt as needed for any divergences in codebase structure

Example workflow:
```bash
git remote add upstream https://github.com/openai/codex.git
git fetch upstream pull/12523/head
git cherry-pick -x <commit-hash>  # Review and resolve conflicts
```

---

**Report Generated:** 2026-02-28
**Data Source:** gh pr list via GitHub CLI
**Analysis Tool:** Claude Code

