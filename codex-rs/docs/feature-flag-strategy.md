# Feature-Flag Strategy Audit (codex-rs)

## Current state

- Compile-time feature surface is small and mostly localized:
  - `codex-tui`: `voice-input`, `vt100-tests`, `debug-logs`.
  - `codex-cloud-tasks-client`: `online` (default), `mock`.
  - `codex-otel`: `disable-default-metrics-exporter`.
- `codex-tui` already uses a compile-time capability seam for voice support.
  The implementation lives in `voice.rs` and a stub implementation is selected when voice is disabled.
- Runtime feature toggles for product behavior already live in `core` (`[features]` config table) and are unrelated to Cargo features.

## Low-risk architecture move

Instead of expanding compile-time flags per environment in a single place, keep the current split but make seams explicit:

1. Treat compile-time features as **adapter plumbing only**.
2. Keep one public voice contract in each feature boundary (`mod voice`) and move stub logic out of inline `cfg` blocks.
3. Add targeted feature checks to CI/local workflow to keep `--no-default-features` and side-feature combinations executable in one command.

## Implementation done in this proposal

- `codex-rs/tui/src/lib.rs`: moved inline voice fallback module into
  `codex-rs/tui/src/voice_stub.rs` behind `#[path = ...]` feature gates.
- `justfile`: added `just feature-matrix` to validate common combinations quickly:
  - `codex-tui` default-disabled and debug-logs permutations
  - `codex-cloud-tasks-client` default/mock clients
  - `codex-otel` exporter disabling flag

## Why this is safer than duplication-heavy approaches

- No behavior change; only relocation of one stub implementation.
- Avoids future drift where inline stub and real module diverge.
- Creates a place to add one additional adapter implementation (stub/real/test double)
  without touching the module graph each time.
