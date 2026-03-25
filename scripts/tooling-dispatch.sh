#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  tooling-dispatch.sh pm [args...]
  tooling-dispatch.sh json [args...]
  tooling-dispatch.sh py [args...]
  tooling-dispatch.sh ts [args...]

Feature flags (env):
  HELIOS_USE_BUN=0|1
  HELIOS_USE_JAQ=0|1
  HELIOS_USE_PY314=0|1
  HELIOS_USE_TS_NATIVE=0|1
EOF
}

command_exists() {
  command -v "$1" >/dev/null 2>&1
}

select_pm() {
  if [[ "${HELIOS_USE_BUN:-0}" == "1" ]] && command_exists bun; then
    echo "bun"
    return
  fi
  if command_exists pnpm; then
    echo "pnpm"
    return
  fi
  if command_exists bun; then
    echo "bun"
    return
  fi
  echo "error: neither pnpm nor bun found in PATH" >&2
  exit 127
}

select_json() {
  if [[ "${HELIOS_USE_JAQ:-0}" == "1" ]] && command_exists jaq; then
    echo "jaq"
    return
  fi
  if command_exists jq; then
    echo "jq"
    return
  fi
  if command_exists jaq; then
    echo "jaq"
    return
  fi
  echo "error: neither jq nor jaq found in PATH" >&2
  exit 127
}

select_py() {
  if [[ "${HELIOS_USE_PY314:-0}" == "1" ]] && command_exists python3.14; then
    echo "python3.14"
    return
  fi
  if command_exists python3; then
    echo "python3"
    return
  fi
  if command_exists python; then
    echo "python"
    return
  fi
  echo "error: no python runtime found (python3/python)" >&2
  exit 127
}

select_ts_runner() {
  if [[ "${HELIOS_USE_TS_NATIVE:-0}" == "1" ]] && command_exists tsgo; then
    echo "tsgo"
    return
  fi
  if [[ "${HELIOS_USE_BUN:-0}" == "1" ]] && command_exists bunx; then
    echo "bunx"
    return
  fi
  if command_exists pnpm; then
    echo "pnpm exec"
    return
  fi
  if command_exists bunx; then
    echo "bunx"
    return
  fi
  echo "error: no TS runner found (pnpm exec/bunx/tsgo)" >&2
  exit 127
}

main() {
  if [[ $# -lt 1 ]]; then
    usage
    exit 1
  fi

  local mode="$1"
  shift

  case "$mode" in
    pm)
      local pm
      pm="$(select_pm)"
      exec "$pm" "$@"
      ;;
    json)
      local json_tool
      json_tool="$(select_json)"
      exec "$json_tool" "$@"
      ;;
    py)
      local py
      py="$(select_py)"
      exec "$py" "$@"
      ;;
    ts)
      local ts_runner
      ts_runner="$(select_ts_runner)"
      # shellcheck disable=SC2086
      exec $ts_runner "$@"
      ;;
    *)
      usage
      exit 1
      ;;
  esac
}

main "$@"
