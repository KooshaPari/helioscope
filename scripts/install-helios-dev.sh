#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CODEX_RS_DIR="${ROOT_DIR}/codex-rs"
TARGET_BIN="${CODEX_RS_DIR}/target/release/codex"
LOCAL_BIN="${HOME}/.local/bin"

echo "[install-dev] building release binary..."
cargo build -p codex-cli --release --manifest-path "${CODEX_RS_DIR}/Cargo.toml"

if [[ ! -x "${TARGET_BIN}" ]]; then
  echo "[install-dev] build completed but binary not found: ${TARGET_BIN}" >&2
  exit 1
fi

mkdir -p "${LOCAL_BIN}"

ln -sfn "${TARGET_BIN}" "${LOCAL_BIN}/helios"
ln -sfn "${TARGET_BIN}" "${LOCAL_BIN}/helios-dev"

echo "[install-dev] linked:"
echo "  ${LOCAL_BIN}/helios -> ${TARGET_BIN}"
echo "  ${LOCAL_BIN}/helios-dev -> ${TARGET_BIN}"
echo "[install-dev] done."
