# Research

## Findings
- `rmcp 1.3.0` exposes `StreamableHttpClient` support for `reqwest 0.13.2`, not `0.12.x`.
- The failing CI run (`#129`) compiles `codex-rmcp-client` with `reqwest 0.12.28`, which breaks the
  streamable HTTP transport trait bound.
- `CallToolRequestParams` is `#[non_exhaustive]` in `rmcp 1.3.0`, so the struct literal in
  `rmcp_client.rs` must use the provided constructor.

## Sources
- Local crate sources in `~/.cargo/registry/src/.../rmcp-1.3.0/`
- GitHub Actions run `24934131337`
