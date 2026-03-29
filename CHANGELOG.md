The changelog can be found on the [releases page](https://github.com/openai/codex/releases).

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-29

### Added
- Release channel framework (stage-gates CI, CodeRabbit, gatekeeper)
- phenotype-config SDK integration
- Docs-site scaffold and verification harness with VitePress integration
- AgilePlus spec format migration for kitty-specs

### Fixed
- Syntax errors and dependency version issues
- Replace unwrap with expect for file search limit
- Replace deprecated criterion::black_box with std::hint::black_box (#126, #127)
- Rust CI, codespell, and cargo-deny failures on main (#125)

### Changed
- Migrate kitty-specs to docs/specs (AgilePlus format) (#128)
- Extract trait-based architecture from chat_composer (KeyEventRouter, Renderable, Submitter, HistoryNavigator traits)
- Text manipulation utilities extraction to dedicated module
- Migrate top 20 TODOs to GitHub Issues

### Chore
- Branch cleanup and removal of broken symlink member references
- Integration of @phenotype/docs into VitePress docs

## [0.1.0-codex.105.0] - 2026-02-25

HELIOS-CODEX version: our v0.1.0, forked from upstream openai/codex at rust-v0.105.0.

### Added
- Rebrand codex → helios across all crates and binaries
- Go-based transport layer for network proxy
- Performance scripts and benchmark baselines (from heliosHarness)
- Docs/context governance scaffold
- Fork documentation (HELIOSCLI_README.md, HELIOS_REBRAND_SUMMARY.md)

### Changed
- N-way merge consolidating heliosCLI with Rust core, Go transport, Zig, Python benchmarks

### Security
- Added esbuild override >=0.25.0 (CVE mitigation)

### Upstream (openai/codex rust-v0.105.0)
- Display pending child-thread approvals in TUI (#12767)
- Record whether a skill script is approved for the session (#12756)
- Support external agent config detect and import (#12660)
- Add search term to thread list (#12578)
- Add service name to app-server (#12319)
- Surface skill permission profiles in zsh-fork exec approvals (#12753)
- Network approval persistence plumbing (#12358)
- Update Docker image digest (#12372)
