#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DISPATCH="${ROOT_DIR}/scripts/tooling-dispatch.sh"
STAGER="${ROOT_DIR}/scripts/stage_npm_packages.py"

if [[ ! -x "$DISPATCH" ]]; then
  echo "missing tooling dispatcher: $DISPATCH" >&2
  exit 1
fi

if [[ ! -f "$STAGER" ]]; then
  echo "missing npm staging script: $STAGER" >&2
  exit 1
fi

# Expose preferred package manager for downstream scripts.
# Honor explicit CODEX_PREFERRED_PM first; otherwise derive from feature flag.
if [[ -z "${CODEX_PREFERRED_PM:-}" ]]; then
  if [[ "${HELIOS_USE_BUN:-0}" == "1" ]]; then
    export CODEX_PREFERRED_PM="bun"
  else
    export CODEX_PREFERRED_PM="pnpm"
  fi
fi

exec "$DISPATCH" py "$STAGER" "$@"
