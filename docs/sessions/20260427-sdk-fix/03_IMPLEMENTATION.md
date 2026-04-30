# Implementation

## Changes
- Kept the existing `reqwest` 0.13.2 alignment in `codex-rs/rmcp-client/Cargo.toml`.
- Replaced the non-exhaustive `CallToolRequestParams` struct literal with
  `CallToolRequestParams::new(name)` and assigned the optional arguments after construction.

## Rationale
- The transport bound failure is a dependency-version mismatch, not a design issue.
- The request-parameter constructor fix is the smallest way to satisfy `rmcp`'s non-exhaustive
  model without broad refactoring.
