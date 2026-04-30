#!/bin/bash

set -euo pipefail

CODEX_CLI_DIR=$(realpath "$(dirname "$0")/../codex-rs/cli")
(cd "$CODEX_CLI_DIR" && cargo run --quiet --bin codex -- "$@")
