# OpenAI Codex - Unmerged Features Analysis Report
**Analysis Date:** 2026-02-28
**Repository:** openai/codex
**Analysis Period:** Open PRs + Last 60 days (closed PRs from ~2026-01-01 onward)

---

## Executive Summary

This analysis identifies valuable unmerged features and improvements from openai/codex that could provide competitive advantages for helios-cli. The upstream repository contains **200+ open PRs** and **200+ recent closed PRs**, with a notable shift toward OpenAI-internal-only contributions starting 2026-02-28 (when the repository closed to external contributors).

**Key Findings:**
1. Most valuable features are **already open** (not merged yet)
2. Several **strategic UX improvements** closed without merging due to external contributor policy change
3. Core infrastructure improvements span: **sandbox enhancements, TUI rendering, collaboration features, plugin systems, and tool recovery mechanisms**

---

## HIGH VALUE: Implement These Features

### 1. **Markdown Table Rendering** (PR #12523)
**Status:** Open | **Size:** 5,832 additions | **Author:** Felipe Coury

**What It Does:**
- Renders Markdown tables with Unicode box-drawing borders in TUI transcripts
- Supports table width adaptation to terminal dimensions
- Implements a two-region streaming model (stable scrollback + mutable tail region)
- Fixes terminal resize handling for all markdown content

**Why It's Valuable:**
- LLMs frequently output structured data as tables (very common in agent responses)
- Current Codex/helios-cli rendering shows raw pipes (`| A | B |`) which wastes space and hurts readability
- Includes terminal resize reflow — improves UX across all rendered content
- Comprehensive architecture with proper testing

**Implementation Difficulty:** **MEDIUM**
- Complex streaming/rendering logic with holdback semantics
- Requires integration with TUI rendering pipeline
- ~5,800 lines of new code but well-structured with clear separation of concerns

**Recommendation:** **IMPLEMENT FIRST**
This single feature dramatically improves perceived output quality for agent responses.

---

### 2. **Plugin System** (PR #12864)
**Status:** Open | **Size:** 1,380 additions | **Author:** xl-openai

**What It Does:**
- Enables first-class plugin architecture via `[plugins.<name>]` in config.toml
- Plugins contribute custom skills and MCP server configurations
- Integrated through existing PluginsManager, not a separate parallel system

**Why It's Valuable:**
- Fork extensibility without modifying core
- Users can create and share domain-specific skill packs
- Competitive advantage over stock Codex (users build capabilities vs. waiting for upstream)

**Implementation Difficulty:** **LOW**
- ~1,380 lines of straightforward plugin loading logic
- Follows existing patterns for skills and MCP integration
- Minimal API surface

**Recommendation:** **IMPLEMENT AFTER** markdown tables

---

### 3. **Model Toggle (Ctrl+O)** — Two Versions Available
**Versions:**
- **PR #12780:** Session-local ring (no persistence), simpler
- **PR #12703:** Persisted config pairs, smarter reconciliation (RECOMMENDED)

**Status:** Both Closed (not merged due to external contributor policy)
**Size:** 314 additions (#12780) / 950 additions (#12703)
**Author:** Felipe Coury

**What It Does:**
- Ctrl+O hotkey toggles between two most-recently-used models
- #12703 also persists toggle pair to config; session always starts on configured default
- Tracks both model and reasoning effort with the toggle

**Why It's Valuable:**
- High-friction UX issue solved: users constantly switching between (fast + capable) models
- v2 (#12703) prevents accidental session starts on "wrong" model
- Extremely common workflow (especially with Sonnet/Opus/Spark trio)

**Implementation Difficulty:** **LOW**
- Simple hotkey handler + ring buffer management
- ~300-950 lines depending on persistence scope
- No infrastructure changes needed

**Recommendation:** **IMPLEMENT #12703**
Solves real user pain point. Code is clean and minimal.

---

### 4. **Live Skill Refresh Notifications** (PR #12803)
**Status:** Open | **Size:** 351 additions | **Author:** Yaroslav Volovich

**What It Does:**
- Notifies users when skills are refreshed (new/updated/removed)
- Live updates in TUI without requiring restart
- Covers skill list updates and dependency changes

**Why It's Valuable:**
- Improved UX for multi-skill workflows
- Users aware of skill changes without manual refresh
- Prevents silent failures from outdated skill metadata

**Implementation Difficulty:** **LOW**
- ~351 lines integrating notification events into existing skill system
- No new infrastructure

**Recommendation:** **IMPLEMENT** (after core features)

---

### 5. **Realtime Audio Device Picker & Config** (PR #12850, #12849)
**Status:** Open | **Sizes:** 582 + 299 additions | **Author:** Ahmed Ibrahim

**What It Does:**
- Allows users to select audio input/output devices for realtime mode
- Persists audio device preferences to config
- Surface device selection in TUI settings

**Why It's Valuable:**
- Essential UX for multi-device users (USB mics, headsets, etc.)
- Realtime is increasingly important for voice workflows
- Currently forces use of system default device

**Implementation Difficulty:** **MEDIUM**
- Requires audio device enumeration (platform-specific)
- Config persistence + TUI controls
- ~880 lines total

**Recommendation:** **IMPLEMENT IF** supporting realtime features

---

### 6. **MCP Transport Recovery on Broken Pipe** (PR #13100)
**Status:** Closed/Unmerged (external contributor policy)
**Size:** 482 additions | **Author:** Skylar Graika | **Related:** #6649

**What It Does:**
- Robustly recovers from browser MCP transport failures (Chrome DevTools, Playwright)
- Instead of wedging, restarts MCP connection and retries tool call
- Matches broader error patterns: `broken pipe`, `connection reset`, not just `transport closed`

**Why It's Valuable:**
- Solves long-standing MCP stability issue (#6649)
- Browser automation is increasingly important for agents
- Users get in-session recovery instead of needing app restart

**Implementation Difficulty:** **LOW**
- ~482 lines; mostly error pattern matching + recovery orchestration
- Well-tested with e2e validation on macOS

**Recommendation:** **IMPLEMENT**
Reliability improvement with minimal risk.

---

### 7. **Vim Mode** (PR #11865)
**Status:** Open | **Size:** 795 additions | **Author:** Dylan Hurd

**What It Does:**
- `/vim` toggle for Vim keybindings in prompt editor
- Normal mode navigation + Insert mode support
- Esc switches modes; Esc+Esc edits previous message

**Why It's Valuable:**
- Growing demand from power users
- Vim users are highly motivated to adopt tools with native bindings
- Well-scoped implementation (doesn't attempt full Visual mode)

**Implementation Difficulty:** **MEDIUM**
- Requires TUI input handler modifications
- ~795 lines; focused on composition, not architecture

**Recommendation:** **IMPLEMENT** (if targeting power users)

---

### 8. **Permissions Customization UI** (PR #11549)
**Status:** Open | **Size:** 203 additions | **Author:** Dylan Hurd

**What It Does:**
- `/permissions custom` option to fine-tune sandbox access
- Integrated with TUI config flow
- Custom permission profiles for advanced users

**Why It's Valuable:**
- Sandbox security is critical; gives users transparency + control
- Advanced use cases (e.g., restricted deployments) need this
- Improves trust in agent tool execution

**Implementation Difficulty:** **LOW-MEDIUM**
- ~203 lines; integrates with existing permissions system
- UX flow in TUI config

**Recommendation:** **IMPLEMENT** (security-conscious users will appreciate)

---

### 9. **OSC-52 Copy Support** (PR #12951)
**Status:** Open | **Size:** 210 additions | **Author:** Won Park

**What It Does:**
- `/copy` command in TUI uses OSC-52 escape sequence for system clipboard
- Works in tmux, SSH sessions, and terminal multiplexers
- Fallback behavior for terminals without OSC-52

**Why It's Valuable:**
- Solves long-standing pain point: copying from tmux/SSH sessions
- Power users rely on this
- Minimal risk (fallback to existing behavior)

**Implementation Difficulty:** **LOW**
- ~210 lines
- Standard OSC-52 implementation

**Recommendation:** **IMPLEMENT**

---

### 10. **Realtime Collaboration Mode** (PR #12687)
**Status:** Closed/Unmerged | **Size:** 800 additions | **Author:** Ahmed Ibrahim

**What It Does:**
- TUI mode for realtime (audio) conversations
- Handles audio streaming, transcription display, and live responses
- Integrates with realtime model features (e.g., GPT-4o Realtime API)

**Why It's Valuable:**
- Voice-first interactions increasingly important
- Differentiation from stock Codex
- Opens new use cases (voice debugging, pair programming)

**Implementation Difficulty:** **HIGH**
- Complex TUI state management for concurrent audio/text streams
- Requires realtime API client
- ~800 lines, but intricate

**Recommendation:** **IMPLEMENT IF** voice is strategic priority
Lower priority than other features but high payoff if executed.

---

## MEDIUM VALUE: Consider These Features

### 1. **Quick Model Picker Highlight** (PR #13073)
**Status:** Open | **Size:** 161 additions

Strengthens visual feedback in model selection UI. Low-hanging fruit.

### 2. **Skilled Disable Respects Config Layer** (PR #13027)
**Status:** Open | **Size:** 155 additions

Allows config-level control of skill availability. Important for multi-profile setups.

### 3. **App-Server v2 Realtime API** (PR #12715)
**Status:** Open | **Size:** 1,706 additions

Enables realtime conversation in app-server (protocol level). Prerequisite for voice features.

### 4. **Thread-Based Fork with Turn Picker** (PR #12610)
**Status:** Open | **Size:** 779 additions

Improves fork UX with visual turn selection. Better than baseline fork.

### 5. **Host_executable() Support** (PR #13045, #13065, #13064)
**Status:** Open | **Sizes:** 900 + 310 + 175 additions | **Author:** Michael Bolin

Enhances exec policy matching for absolute-path commands. Security/flexibility win.

### 6. **User Input Requests Outside Plan Mode** (PR #12726)
**Status:** Closed/Unmerged | **Size:** 276 additions

Allows `request_user_input` in Default mode (not just Plan). Expands agent autonomy options.

### 7. **Clear Terminal on /new** (PR #13111)
**Status:** Closed/Unmerged | **Size:** 2 additions

Trivial UX improvement: clears screen on new chat. Easy win.

### 8. **Theme-Aware Diff Backgrounds** (PR #13037)
**Status:** Open | **Size:** 586 additions

Better diff rendering with terminal theme support. Improves readability.

### 9. **Consolidation Prompt Refinements** (PR #12653)
**Status:** Open | **Size:** 88 additions

Tightens memory consolidation prompts. Improves memory quality.

### 10. **Exec Quiet Mode** (PR #12739)
**Status:** Closed/Unmerged | **Size:** 183 additions | **Author:** Adam Koszek

Suppresses non-essential output for scripting. Useful for automation.

---

## LOW VALUE: Skip These

### Items with Limited Applicability
- **PR #12342:** Map config clearing (niche)
- **PR #12752:** Network allowlist merging (internal)
- **PR #12389:** Deny-read blocklist (specific permission feature)
- **PR #12690:** Custom sandbox policies (WIP, incomplete)
- **PR #11209:** Feature flag removal (maintenance)

### Obsolete or Upstream-Specific
- **PR #12769:** "Codex to Helios" rebrand (closed in error; specific to fork)
- **PR #12748:** Skill sandbox settings (partially superseded by later PRs)
- **PR #12726:** Request user input (gated feature flag)

### Broken Pipe Specialties (Already in High Value)
- PRs dealing with internal MCP plumbing that don't justify standalone implementation

---

## Strategic Recommendations

### Phase 1: Foundation (Weeks 1-3)
1. **Markdown tables** (PR #12523) — Immediate UX win
2. **Plugin system** (PR #12864) — Extensibility
3. **Vim mode** (PR #11865) — Power user appeal

### Phase 2: Polish (Weeks 4-6)
4. **Model toggle** (PR #12703) — QoL feature
5. **Realtime audio device picker** (PR #12850 + #12849)
6. **OSC-52 copy** (PR #12951)
7. **MCP transport recovery** (PR #13100)

### Phase 3: Advanced (Weeks 7+)
8. **Realtime conversation mode** (PR #12687) — if voice is strategic
9. **Permissions customization** (PR #11549)
10. **Host_executable improvements** (PR #13065)

### Ongoing/Maintenance
- Consolidation prompt refinements (memory quality)
- Skill refresh notifications (UX polish)
- Live updates handling (architecture)

---

## Important Context

**Contribution Policy Change:**
OpenAI changed their contribution policy on 2026-02-28, moving to "OpenAI-internal-only contributions." This means:
- Recent closed external PRs (#13098, #13100, etc.) were closed per policy, not rejection
- Several valuable features are now only available via this fork
- Excellent opportunity for helios-cli to differentiate

**Key External Contributors:**
- **Felipe Coury** (fcoury) — Model toggle, Markdown tables, quick model picker
- **Dylan Hurd** (drhurd) — Vim mode, permissions UI, Python tool support
- **Michael Bolin** (bolinfest) — Exec policy, sandbox improvements
- **Adam Koszek** (wkoszek) — Quiet mode, prompt flag
- **Skylar Graika** (swordfish444) — MCP recovery

These contributors' PRs represent excellent features that deserve implementation in the fork.

---

## Integration Notes

### Build System
- Most features integrate cleanly into existing Bazel build (codex-rs crate organization)
- No external dependencies added in most PRs
- Test coverage generally adequate

### Testing
- Featured PRs include unit/integration tests
- Snapshot tests for TUI changes
- Some features benefit from e2e validation (audio, realtime)

### Documentation
- Config.toml schema updates needed for plugin system, audio device selection
- README updates for new commands (/vim, /copy, /permissions custom)
- Consider adding feature discovery UI for new toggles

---

## Files to Review in Detail

Key files to audit in upstream PRs:

1. **Markdown Tables:** `codex-rs/codex-tui/src/streaming/` (main implementation)
2. **Plugin System:** `codex-rs/core/src/plugin/` (loading logic)
3. **Vim Mode:** `codex-rs/codex-tui/src/input/` (keybinding handler)
4. **Model Toggle:** `codex-rs/codex-tui/src/chat_widget/` (ring buffer + config persistence)
5. **Realtime Audio:** `codex-rs/codex-tui/src/realtime/` (device enumeration + TUI controls)

---

## Conclusion

The unmerged Codex PRs contain **high-quality, production-ready features** that represent significant UX improvements and architectural enhancements. The shift to internal-only contributions creates a unique opportunity for helios-cli to implement these features and differentiate from upstream.

**Top 3 Priorities:**
1. Markdown table rendering (immediate visual impact)
2. Plugin system (enables extensibility + differentiation)
3. Model toggle (solves real user workflow friction)

These three features alone would provide substantial competitive advantage.

