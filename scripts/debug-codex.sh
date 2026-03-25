#!/bin/bash

# Set "helios.cliExecutable": "/path/to/repo/scripts/debug-codex.sh" in VSCode settings to always get the
# latest local CLI binary when debugging.


set -euo pipefail

HELIOS_RS_DIR=$(realpath "$(dirname "$0")/../helios-rs")
(cd "$HELIOS_RS_DIR" && cargo run --quiet --bin helios -- "$@")
