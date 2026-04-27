# Research

## Live evidence

- Issue `#186` points to run `24918681199`.
- The failed log shows `wget .../releases/download/2.15.0/zap-full-linux-amd64.tar.gz` returning `404 Not Found`.
- The same checkout cleanup warning appears after the failed step because the repo has archived
  submodule/worktree baggage.

## Decision

Use the latest release asset path from GitHub Releases instead of a pinned versioned URL, and
disable persisted checkout credentials to reduce cleanup friction.
