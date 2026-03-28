# Research: Codex TUI Renderer Optimization

## Objective
Create an evidence-backed architecture baseline for the Codex TUI renderer and backend flow, then recommend phased optimizations and layered PR slicing.

## Current-State Architecture

### Renderer Flow (Current)

```text
[Terminal Input/Event Stream]
            |
            v
   TuiEvent (key/paste/draw/resize)
            |
            v
+-------------------------------+
| App::run main select loop     |
| - AppEvent channel            |
| - Codex protocol events       |
| - TuiEvent stream             |
+-------------------------------+
            |
            v
  App::handle_tui_event / handle_event
            |
            v
+-------------------------------+
| ChatWidget state mutation     |
| - history cells               |
| - bottom pane state machine   |
| - status + overlays           |
+-------------------------------+
            |
            v
 FrameRequester::schedule_frame
            |
            v
 FrameScheduler (coalesce/throttle)
            |
            v
          Tui::draw
            |
            v
   ratatui Terminal::draw
   (buffer diff flush only)
```

### Backend Flow (Current)

```text
codex CLI entry
(main.rs -> lib.rs::run_main)
            |
            v
TUI App runtime (app.rs)
            |
            v
ThreadManager (core)
- thread lifecycle
- event fan-in/out
            |
            v
Codex + ModelClient session
- provider auth/session
- stream transport
            |
            v
Responses/protocol events
(codex_protocol Event/EventMsg)
            |
            v
AppEvent + UI channels
            |
            v
ChatWidget render state
            |
            v
Terminal frame updates

Parallel control-plane exposure:
app-server JSON-RPC API emits the same thread/turn/item event model
for external clients and automation.
```

## Key Findings
1. Frame scheduling is already centralized and coalesced, which is a strong base for dirty-region and adaptive-rate improvements.
2. Render cost is dominated by state-to-view transformation and high-frequency redraw triggers, not just terminal flush.
3. Backend event throughput can outpace UI consumption under heavy streaming unless queue bounds and batching policies are explicit.
4. Current architecture supports incremental optimization because render orchestration and backend protocol flow are cleanly separated.

## External Optimization Evidence (Rust TUI)

### Reviewed article: octorus 300K-line diff rendering
- Multi-layer caches to separate session data, diff precompute, and active view caches.
- Background precompute to reduce first-open latency.
- Viewport-restricted rendering to avoid whole-document work.
- String interning and structured token reuse to reduce allocation and memory churn.
- Deferred/lazy composition for secondary decorations (for example, comments/markers).

### Additional references and implications
- ratatui uses double buffers and writes only changed cells, so avoid forcing full redraw semantics.
- crossterm polling/event APIs can be used to control event loop wake behavior.
- Tokio bounded channels provide explicit backpressure and should be preferred in high-rate producer paths.
- Rope/interner data structures (for large text/token workloads) can constrain memory and copy overhead.

## Recommended Optimization Layers

### Layer 1: Observability and Baseline (low risk)
- Add renderer timing spans by stage (event ingest, state mutation, compose, draw).
- Add queue-depth and event-lag metrics for UI-facing channels.
- Produce reproducible benchmark fixtures for large transcript and high-stream turns.

### Layer 2: Render Work Reduction (medium risk)
- Introduce viewport/dirty-range constrained state-to-view transformation.
- Cache expensive layout/text shaping outputs keyed by stable state signatures.
- Batch low-priority cosmetic updates behind frame budget checks.

### Layer 3: Event Pressure Control (medium risk)
- Enforce bounded channels on high-volume paths.
- Add coalescing policies for bursty protocol deltas.
- De-prioritize non-critical event classes during sustained load.

### Layer 4: Data-Structure Efficiency (higher risk)
- Evaluate rope/interner-backed buffers for large scrollback and diff-like payloads.
- Minimize transient allocations in render preparation paths.
- Add zero-copy borrowing contracts where lifetimes are already naturally scoped.

### Layer 5: UX-Protective Progressive Rendering (higher risk)
- Progressive display modes for large payloads (fast plain path, enriched path when ready).
- Deferred decoration composition for non-blocking first paint.

## Layered PR Strategy Draft

### PR-A: Instrumentation and Benchmarks
- Scope: metrics, traces, baseline harness, no behavior change.
- Exit: stable measurements for frame times and event lag.

### PR-B: Render Pipeline Caching + Viewport Constraint
- Scope: reduce compose work and unnecessary redraw preparation.
- Exit: measurable reduction in p95 frame time under heavy transcript scenarios.

### PR-C: Event Backpressure + Coalescing
- Scope: bounded channel policies and burst handling.
- Exit: no runaway queue growth under sustained streaming.

### PR-D: Large-Text Data Structure Enhancements
- Scope: targeted interning/rope introduction where evidence shows clear benefit.
- Exit: lower memory growth and allocation churn for large-content sessions.

### PR-E: Progressive Enrichment UX
- Scope: plain-first fast path, deferred enrichments.
- Exit: improved first-meaningful-display latency on large payloads.

## Open Questions
- Which user workflows should define canonical performance SLOs (chat streaming, diff-like browsing, or both)?
- Which event classes can be safely coalesced without degrading perceived responsiveness?
- What benchmark dataset should become the long-term regression gate?

## Risks
- Premature optimization without benchmark baselines may introduce complexity with weak impact.
- Backpressure tuning may affect perceived responsiveness if thresholds are set too aggressively.
- Data-structure migrations can expand code complexity and require careful rollback boundaries.

## Sources
- DEV article: https://dev.to/ushironoko/how-octorus-renders-300k-lines-of-diff-at-high-speed-h4p
- ratatui rendering concepts: https://ratatui.rs/concepts/rendering/
- ratatui Terminal buffering: https://docs.rs/ratatui/latest/ratatui/struct.Terminal.html
- crossterm poll API: https://docs.rs/crossterm/latest/crossterm/event/fn.poll.html
- tokio bounded channels/backpressure: https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html
- tokio channel backpressure guidance: https://tokio.rs/tokio/tutorial/channels
- ropey rope performance model: https://docs.rs/ropey/latest/ropey/struct.Rope.html
- lasso interning model: https://docs.rs/lasso/latest/lasso/
- local code: `codex-rs/tui/src/app.rs`, `codex-rs/tui/src/chatwidget.rs`, `codex-rs/tui/src/tui.rs`, `codex-rs/tui/src/tui/frame_requester.rs`, `codex-rs/core/src/thread_manager.rs`, `codex-rs/core/src/client.rs`, `codex-rs/core/src/model_provider_info.rs`, `codex-rs/app-server/README.md`
