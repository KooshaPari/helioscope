#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

usage() {
  cat <<'USAGE' >&2
Usage: scripts/task_surface.sh <lint|test|fmt|quality>

Commands:
  lint     Run lint checks (cargo clippy --tests)
  test     Run test suite (cargo nextest run --no-fail-fast)
  fmt      Format Rust code (cargo fmt -- --config imports_granularity=Item)
  quality  Run strict quality surface (lint then test)
USAGE
}

run_lint() {
  (
    cd "${REPO_ROOT}/codex-rs"
    cargo clippy --tests
  )
}

run_test() {
  (
    cd "${REPO_ROOT}/codex-rs"
    cargo nextest run --no-fail-fast
  )
}

run_fmt() {
  (
    cd "${REPO_ROOT}/codex-rs"
    cargo fmt -- --config imports_granularity=Item
  )
}

run_quality() {
  run_lint
  run_test
}

if [[ $# -ne 1 ]]; then
  usage
  exit 2
fi

case "$1" in
  lint)
    run_lint
    ;;
  test)
    run_test
    ;;
  fmt)
    run_fmt
    ;;
  quality)
    run_quality
    ;;
  *)
    echo "error: unsupported task-surface command: $1" >&2
    usage
    exit 2
    ;;
esac
