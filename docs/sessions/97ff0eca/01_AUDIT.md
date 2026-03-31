# Dependency Audit — heliosCLI

## Scope

- Root workspace (`Cargo.toml` + `Cargo.lock`)
- `codex-rs/` workspace (the actual heliosCLI project, `helios-rs/` is an incomplete fork)
- `helios-rs/` workspace (incomplete — only `cli/` directory exists, references missing members)

## Findings

### 1. Root Workspace — Broken `harness_pyo3` Dependency

`crates/harness_pyo3/Cargo.toml` references `ffi_utils = { path = "../../../phenotype-shared/crates/ffi_utils" }` which does not exist. This broke all cargo operations on the root workspace.

**Fix**: Excluded `crates/harness_pyo3` from the workspace `members` list and added it to `exclude`. This is consistent with the AGENTS.md note about `harness_pyo3` having a missing `ffi_utils` dependency.

### 2. Root Workspace — Updated Dependencies

After fixing the broken dependency, ran `cargo update`. Updated 9 packages:

| Package                    | Old     | New     |
| -------------------------- | ------- | ------- |
| js-sys                     | 0.3.92  | 0.3.93  |
| wasm-bindgen               | 0.2.115 | 0.2.116 |
| wasm-bindgen-macro         | 0.2.115 | 0.2.116 |
| wasm-bindgen-macro-support | 0.2.115 | 0.2.116 |
| wasm-bindgen-shared        | 0.2.115 | 0.2.116 |
| web-sys                    | 0.3.92  | 0.3.93  |
| winnow                     | 1.0.0   | 1.0.1   |
| zerocopy                   | 0.8.47  | 0.8.48  |
| zerocopy-derive            | 0.8.47  | 0.8.48  |

Removed 7 packages (pyo3 ecosystem, no longer needed after excluding harness_pyo3):

- pyo3 0.28.2, pyo3-build-config 0.28.2, pyo3-ffi 0.28.2, pyo3-macros 0.28.2, pyo3-macros-backend 0.28.2, target-lexicon 0.13.5, ffi_utils (virtual)

### 3. codex-rs Workspace — Updated 196 Packages

The codex-rs workspace (the actual heliosCLI project) had 196 packages updated. Key updates include:

#### Security-Relevant Updates

| Package           | Old     | New      | Notes                  |
| ----------------- | ------- | -------- | ---------------------- |
| aws-lc-rs         | 1.15.4  | 1.16.2   | Cryptographic backend  |
| aws-lc-sys        | 0.37.0  | 0.39.1   | Cryptographic backend  |
| rustls            | 0.23.36 | 0.23.37  | TLS implementation     |
| rustls-webpki     | 0.103.9 | 0.103.10 | Certificate validation |
| openssl           | 0.10.75 | 0.10.76  | OpenSSL bindings       |
| openssl-sys       | 0.9.111 | 0.9.112  | OpenSSL sys bindings   |
| native-tls        | 0.2.14  | 0.2.18   | TLS backend            |
| webpki-roots      | 1.0.5   | 1.0.6    | Root certificates      |
| webpki-root-certs | 1.0.5   | 1.0.6    | Root certificates      |
| ureq              | 3.1.4   | 3.3.0    | HTTP client            |
| sentry            | 0.46.1  | 0.46.2   | Error tracking         |
| tokio             | 1.49.0  | 1.50.0   | Async runtime          |
| libc              | 0.2.182 | 0.2.183  | C library bindings     |
| zerocopy          | 0.8.37  | 0.8.48   | Zero-copy parsing      |
| tempfile          | 3.24.0  | 3.27.0   | Temp file handling     |
| uuid              | 1.20.0  | 1.23.0   | UUID generation        |

#### Major Version Updates

| Package       | Old     | New    | Notes                 |
| ------------- | ------- | ------ | --------------------- |
| anstream      | 0.6.21  | 1.0.0  | Colored output        |
| anstyle-parse | 0.2.7   | 1.0.0  | ANSI parsing          |
| console       | 0.15.11 | 0.16.3 | Terminal output       |
| webbrowser    | 1.0.6   | 1.2.0  | Browser launching     |
| tiff          | 0.10.3  | 0.11.3 | Image format          |
| moxcms        | 0.7.11  | 0.8.1  | Image codec           |
| proptest      | 1.9.0   | 1.11.0 | Property testing      |
| serde_with    | 3.16.1  | 3.18.0 | Serialization helpers |
| clap          | 4.5.58  | 4.6.0  | CLI argument parsing  |
| insta         | 1.46.3  | 1.47.2 | Snapshot testing      |

### 4. Known Vulnerability Context

- **CVE-2026-33056** (tar crate): Noted in Rust blog (Mar 21, 2026). The `tar` crate used by Cargo for package extraction has a vulnerability. This affects cargo operations, not runtime. No direct `tar` dependency in heliosCLI lock files.
- **RUSTSEC-2025-0151** (sha-rst): Malicious crate removed from crates.io. Not a dependency of heliosCLI.

### 5. helios-rs Workspace Status

The `helios-rs/` workspace is incomplete:

- Only contains `Cargo.toml` and `cli/` directory
- References 65 workspace members that don't exist
- Cannot build or update independently
- The actual working codebase is in `codex-rs/` (helios-rs appears to be a renamed fork in progress)

### 6. Warnings

- Root workspace: `profiles for the non root package will be ignored` — `harness_runner/Cargo.toml` defines a `[profile.release]` section that should be moved to the workspace root.
- 25 packages in codex-rs remain behind latest compatible versions (pinned by workspace version constraints).

## Verification

- `cargo update --dry-run` completed successfully on both workspaces
- `cargo update` completed successfully, updating Cargo.lock files
- `cargo build` could not complete due to missing C linker (`cc`) in the container environment — this is an environment limitation, not a code issue
- All dependency resolution succeeded without conflicts

## Changes Made

1. `Cargo.toml` — Excluded `crates/harness_pyo3` from workspace (broken dependency)
2. `Cargo.lock` — Updated 9 packages, removed 7 (pyo3 ecosystem)
3. `codex-rs/Cargo.lock` — Updated 196 packages
