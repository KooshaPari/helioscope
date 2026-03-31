# Dependency Audit Report — heliosCLI

**Bead**: 75041b74
**Date**: 2026-03-31
**Auditor**: Polecat-26

---

## 1. Project Type & Package Manager

| Property            | Value                                                      |
| ------------------- | ---------------------------------------------------------- |
| **Project**         | heliosCLI (Rust-based CLI for Helioscope apps)             |
| **Package Manager** | Cargo (Rust)                                               |
| **Root Workspace**  | `/Cargo.toml` (workspace root, 18 harness crates)          |
| **CLI Workspace**   | `/helios-rs/Cargo.toml` (64 crates including `helios-cli`) |
| **Edition**         | 2021 (root), 2024 (helios-rs workspace)                    |
| **Toolchain**       | Rust 1.93.0 (pinned in `.mise.toml`)                       |

**Note**: Two distinct Cargo workspaces exist:

- **Root** (`/Cargo.toml`): 18 `harness_*` crates + `arch_test`
- **helios-rs** (`/helios-rs/Cargo.toml`): The actual CLI with 64 member crates

---

## 2. Lock File Status

| Check                        | Result                             |
| ---------------------------- | ---------------------------------- |
| **Cargo.lock exists**        | ✅ Yes (2619 lines, 63 KB)         |
| **Lock file version**        | v4 (current format)                |
| **Packages locked**          | 273 packages                       |
| **Workspace crates in lock** | All 18 root harness crates present |

**⚠️ Concern**: There is NO lock file inside `helios-rs/`. The root `Cargo.lock` only covers the root workspace (harness crates). The `helios-rs` workspace has its own `Cargo.toml` with 64 crates and no corresponding `Cargo.lock`. This means `helios-rs` dependencies are NOT deterministically locked.

---

## 3. Dependency Sources

### Registry Dependencies

All 273 packages in `Cargo.lock` resolve from `crates.io-index`.

### Git Dependencies (helios-rs workspace)

The `helios-rs/Cargo.toml` declares 8 git-pinned dependencies via `patch.crates-io`:

| Crate               | Repo                                          | Pinned Rev |
| ------------------- | --------------------------------------------- | ---------- |
| `crossterm`         | github.com/nornagon/crossterm                 | `87db8bfa` |
| `ratatui`           | github.com/nornagon/ratatui                   | `9b2ad129` |
| `tokio-tungstenite` | github.com/openai-oss-forks/tokio-tungstenite | `132f5b39` |
| `tungstenite`       | github.com/openai-oss-forks/tungstenite-rs    | `9200079d` |

Direct git dependencies:

| Crate      | Repo                           | Pinned Rev |
| ---------- | ------------------------------ | ---------- |
| `nucleo`   | github.com/helix-editor/nucleo | `4253de9f` |
| `runfiles` | github.com/dzbarsky/rules_rust | `b56cbaa8` |

**Assessment**: All git dependencies are pinned to specific revisions. No floating git refs.

---

## 4. Key Dependency Versions

| Crate         | Version        | Notes                                   |
| ------------- | -------------- | --------------------------------------- |
| `tokio`       | 1.50.0         | ✅ Current                              |
| `serde`       | 1.0.228        | ✅ Current                              |
| `serde_json`  | 1.0.149        | ✅ Current                              |
| `clap`        | 4.6.0          | ✅ Current                              |
| `reqwest`     | 0.12 (pinned)  | ✅ Current                              |
| `rustls`      | 0.23 (pinned)  | ✅ Current                              |
| `sqlx`        | 0.8.6 (pinned) | ✅ Current                              |
| `openssl-sys` | 0.9.111        | ⚠️ Check for CVEs                       |
| `ahash`       | 0.7.8          | ⚠️ Known hash collision issues in 0.7.x |

---

## 5. Vulnerability Assessment

**Tooling unavailable**: `cargo-audit` could not be installed (no Rust toolchain in container, no `cc` linker). Manual assessment follows:

### Known Concerns

1. **`ahash` 0.7.8**: This version has known hash collision DoS vulnerabilities (RUSTSEC-2023-0020). The 0.8.x line fixes this. However, this may be a transitive dependency — check if it can be upgraded.

2. **`openssl-sys` 0.9.111**: Should be checked against current OpenSSL CVEs. The project also uses `rustls` as the primary TLS backend, which mitigates this risk.

3. **Git-pinned forks**: The `crossterm` and `ratatui` forks from `nornagon` are pinned to specific commits. These are maintained forks for specific fixes but represent a supply chain risk if the repos are abandoned or compromised.

4. **`tungstenite` / `tokio-tungstenite` forks**: Pinned to `openai-oss-forks` org. These are official OpenAI forks, which is reasonable but adds external dependency risk.

---

## 6. Recommendations

| Priority   | Action                                                                               |
| ---------- | ------------------------------------------------------------------------------------ |
| **HIGH**   | Generate a `Cargo.lock` for the `helios-rs` workspace to ensure deterministic builds |
| **HIGH**   | Upgrade `ahash` from 0.7.8 to 0.8.x if possible (hash collision vulnerability)       |
| **MEDIUM** | Install `cargo-audit` in CI pipeline for automated vulnerability scanning            |
| **MEDIUM** | Review git-pinned fork dependencies for freshness and security                       |
| **LOW**    | Consider consolidating the two workspaces (root + helios-rs) into one                |

---

## 7. Summary

| Metric                | Status                                                |
| --------------------- | ----------------------------------------------------- |
| Package manager       | ✅ Cargo (Rust)                                       |
| Lock file (root)      | ✅ Present, 273 packages                              |
| Lock file (helios-rs) | ❌ Missing                                            |
| Git deps pinned       | ✅ All pinned to specific revs                        |
| Known vulnerabilities | ⚠️ `ahash` 0.7.8 has known issues                     |
| Overall health        | ⚠️ Fair — needs helios-rs lock file and ahash upgrade |
