# Security Audit: heliosCLI / codex-rs

**Date:** 2026-03-31
**Scope:** OWASP Top 10 review, input sanitization, authentication/authorization
**Agent:** Polecat-28

## Executive Summary

A comprehensive security audit was conducted across the heliosCLI codebase (primary code under `codex-rs/`). The audit covered authentication flows, credential storage, sandbox execution, network communication, input handling, and command safety checks.

The codebase demonstrates generally strong security practices including PKCE with S256, OS keyring integration, age encryption at rest, process hardening, and defense-in-depth sandboxing. Several areas for improvement were identified and addressed.

## Findings & Remediations

### HIGH Severity (Fixed)

#### 1. Narrow Dangerous Command Detection

- **File:** `codex-rs/shell-command/src/command_safety/is_dangerous_command.rs`
- **Issue:** Only detected `rm -f`/`rm -rf` and recursive `sudo` checks. Missed `dd`, `mkfs`, `chmod -R`, `chown -R`, `nc`, `nmap`, `kill -9`, and other destructive commands.
- **Fix:** Expanded `is_dangerous_to_call_with_exec()` to detect disk destruction tools, dangerous permission changes, network attack tools, and process killing.

#### 2. Bash Scripts with Redirections Bypass Safety Checks

- **File:** `codex-rs/shell-command/src/bash.rs`, `codex-rs/shell-command/src/command_safety/is_dangerous_command.rs`
- **Issue:** `parse_shell_lc_plain_commands()` returns `None` for scripts with redirections, command substitutions, or complex constructs. This caused the inner script content to skip dangerous command detection.
- **Fix:** Added `shell_lc_script_has_complex_constructs()` function that detects redirections, substitutions, and complex shell constructs. When detected, the command is flagged as potentially dangerous requiring approval.

### MEDIUM Severity (Fixed)

#### 3. Secret Environment Variables Inherited by Default

- **File:** `codex-rs/core/src/config/types.rs`
- **Issue:** `ShellEnvironmentPolicy::ignore_default_excludes` defaulted to `true`, meaning env vars containing `KEY`, `SECRET`, or `TOKEN` were inherited by child processes by default.
- **Fix:** Changed default to `false` in both `Default` impl and `From<ShellEnvironmentPolicyToml>` conversion.

#### 4. Unbounded Request Body in Responses API Proxy

- **File:** `codex-rs/responses-api-proxy/src/lib.rs`
- **Issue:** Request body was read entirely into memory with no size limit, enabling memory exhaustion.
- **Fix:** Added 64 MiB body size limit using `Read::take()`. Returns HTTP 413 if exceeded.

#### 5. Bearer Token Not Zeroized in Backend Client

- **File:** `codex-rs/backend-client/src/client.rs`
- **Issue:** Bearer token stored as plain `String` without memory-zeroing on drop.
- **Fix:** Added `zeroize` dependency and `Drop` impl for `Client` that zeroizes the bearer token.

#### 6. Full Error Body Logged During Token Refresh

- **File:** `codex-rs/core/src/auth.rs`
- **Issue:** `tracing::error!("Failed to refresh token: {status}: {body}")` logged the full response body which could contain sensitive error details.
- **Fix:** Changed to log only the parsed error message via `try_parse_error_message()` instead of the raw body.

### MEDIUM Severity (Documented, Not Fixed)

#### 7. Admin API Has No Authentication

- **File:** `codex-rs/network-proxy/src/admin.rs`
- **Status:** Mitigated by loopback-only binding by default. The `dangerously_allow_non_loopback_admin` flag must be explicitly set to bind to non-loopback.

#### 8. WebSocket Transport Has No TLS or Authentication

- **File:** `codex-rs/app-server/src/transport.rs`
- **Status:** Mitigated by loopback-only default binding and startup warning banner.

#### 9. API Key Stored in Plaintext File (Default Mode)

- **File:** `codex-rs/core/src/auth/storage.rs`
- **Status:** File permissions set to `0o600` on Unix. Keyring mode recommended for production.

#### 10. JWT Parsed Without Signature Verification

- **File:** `codex-rs/core/src/token_data.rs`
- **Status:** Acceptable for tokens received directly from trusted OAuth endpoint. Documented limitation.

### LOW Severity (Documented)

- Trace logging reveals credential access patterns and sizes
- Regex-based log sanitizer is incomplete
- Scrypt work factor not explicitly configured
- Global ephemeral store with truncated hash key
- MITM CA has unconstrained BasicConstraints
- No stdin line length limit in app-server
- No timeout on stdio-to-uds relay
- Shell detection by filename can be bypassed

## Positive Security Features

- PKCE with S256 for OAuth flows
- OS keyring integration for credential storage
- age encryption with scrypt for secrets at rest
- Atomic file writes with temp file + rename
- Memory wiping for sensitive data (zeroize, mlock)
- State parameter validation in OAuth
- Workspace restriction enforcement
- HTML escaping for XSS prevention
- File permissions (0o600) on auth files
- Comprehensive SSRF protection with DNS rebinding defense
- rustls for memory-safe TLS (no OpenSSL)
- Process hardening (prctl, rlimit, LD\_ removal)
- Defense-in-depth sandboxing (bubblewrap + seccomp + no_new_privs)
- Hop-by-hop header stripping (including Proxy-Authorization)
- Symlink attack mitigation in sandbox
- Network rule host normalization (rejects wildcards, whitespace, URL-like strings)

## Files Changed

| File                                                                | Change                                                               |
| ------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `codex-rs/shell-command/src/command_safety/is_dangerous_command.rs` | Expanded dangerous command detection + complex bash script detection |
| `codex-rs/shell-command/src/bash.rs`                                | Added `shell_lc_script_has_complex_constructs()` function            |
| `codex-rs/core/src/config/types.rs`                                 | Changed `ignore_default_excludes` default to `false`                 |
| `codex-rs/responses-api-proxy/src/lib.rs`                           | Added 64 MiB request body size limit                                 |
| `codex-rs/backend-client/src/client.rs`                             | Added `Drop` impl with zeroize for bearer token                      |
| `codex-rs/backend-client/Cargo.toml`                                | Added `zeroize` dependency                                           |
| `codex-rs/core/src/auth.rs`                                         | Sanitized error body logging during token refresh                    |

## Recommendations for Future Work

1. **Add integration tests** for the expanded dangerous command detection
2. **Consider encrypting auth.json** by default instead of plaintext storage
3. **Add certificate pinning** for production deployments
4. **Implement request body size limits** in the app-server stdio transport
5. **Add timeout handling** for stdio-to-uds relay
6. **Review and expand** the log sanitizer regex patterns
7. **Consider constrained BasicConstraints** for MITM CA certificates
