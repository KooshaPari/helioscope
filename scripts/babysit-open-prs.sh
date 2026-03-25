#!/usr/bin/env bash
set -euo pipefail

REPO=""
ONCE=0
SLEEP_SECONDS=240
RETRY_WINDOW_SECONDS=1800
AUTO_PING=1
MAX_STATES=0
GH_TIMEOUT_SECONDS=25
DRY_RUN=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo)
      if [[ $# -lt 2 || -z "${2:-}" ]]; then
        echo "--repo requires a value" >&2
        exit 1
      fi
      REPO="$2"
      shift 2
      ;;
    --once)
      ONCE=1
      shift
      ;;
    --sleep)
      if [[ $# -lt 2 || ! "$2" =~ ^[0-9]+$ ]]; then
        echo "--sleep requires a numeric value" >&2
        exit 1
      fi
      SLEEP_SECONDS="$2"
      shift 2
      ;;
    --no-ping)
      AUTO_PING=0
      shift
      ;;
    --max-cycles)
      if [[ $# -lt 2 || ! "$2" =~ ^[0-9]+$ ]]; then
        echo "--max-cycles requires a numeric value" >&2
        exit 1
      fi
      MAX_STATES="$2"
      shift 2
      ;;
    --gh-timeout)
      if [[ $# -lt 2 || ! "$2" =~ ^[0-9]+$ ]]; then
        echo "--gh-timeout requires a numeric value" >&2
        exit 1
      fi
      GH_TIMEOUT_SECONDS="$2"
      shift 2
      ;;
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    --help|-h)
      cat <<'HELP'
      Usage: scripts/babysit-open-prs.sh [--repo org/repo] [--once] [--sleep SECONDS] [--no-ping] [--max-cycles N]
             [--gh-timeout SECONDS] [--dry-run]

Default behavior: run in a loop, scan open PRs, classify blockers, and optionally ping
@coderabbitai review when only CodeRabbit is failing due to rate-limit signals.

Output columns:
  PR  Branch  MergeState  Mergeable?  CI(fail/nonCoderabbit/rate)  Status
HELP
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

require_cmds() {
  local missing=0
  for cmd in gh; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
      echo "Missing required command: $cmd" >&2
      missing=1
    fi
  done

  if command -v python3 >/dev/null 2>&1; then
    python_cmd="python3"
  elif command -v python >/dev/null 2>&1; then
    python_cmd="python"
  else
    echo "Missing required command: python3 (or python)" >&2
    missing=1
  fi
  if [[ "$missing" -eq 1 ]]; then
    exit 1
  fi

  export PYTHON_CMD="$python_cmd"
}

require_cmds

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TOOLING_DISPATCH="$SCRIPT_DIR/tooling-dispatch.sh"

if [[ ! -x "$TOOLING_DISPATCH" ]]; then
  echo "Missing required script: $TOOLING_DISPATCH" >&2
  exit 1
fi

JQ_CMD=("$TOOLING_DISPATCH" json)

jq_run() {
  "${JQ_CMD[@]}" "$@"
}

run_gh() {
  if command -v timeout >/dev/null 2>&1; then
    timeout "${GH_TIMEOUT_SECONDS}s" gh "$@"
    return $?
  fi

  gh "$@"
  return $?
}

compute_cutoff_timestamp() {
  local seconds="$1"
  "$PYTHON_CMD" - "$seconds" <<'PY'
import datetime
import sys

seconds = int(sys.argv[1])
dt = datetime.datetime.now(datetime.timezone.utc) - datetime.timedelta(seconds=seconds)
print(dt.isoformat().replace("+00:00", "Z"))
PY
}

infer_repo_from_remote() {
  local remote_url
  remote_url=$(git config --get remote.origin.url || true)
  if [[ -z "$remote_url" ]]; then
    return 1
  fi
  if [[ "$remote_url" == git@github.com:* ]]; then
    REPO="${remote_url#git@github.com:}"
    REPO="${REPO%.git}"
  elif [[ "$remote_url" == https://github.com/* ]]; then
    REPO="${remote_url#https://github.com/}"
    REPO="${REPO%.git}"
  else
    return 1
  fi
  return 0
}

if [[ -z "$REPO" ]]; then
  if [[ "$DRY_RUN" -eq 1 ]]; then
    if ! infer_repo_from_remote; then
      echo "Unable to infer repo from remote URL. Use --repo org/repo." >&2
      exit 1
    fi
  else
    if ! REPO=$(run_gh repo view --json nameWithOwner -q '.nameWithOwner'); then
      if ! infer_repo_from_remote; then
        echo "Unable to infer repo. Configure GitHub CLI auth or provide --repo." >&2
        exit 1
      fi
    fi
  fi
fi

if [[ -z "$REPO" ]]; then
  echo "Could not determine repository. Use --repo org/repo." >&2
  exit 1
fi

if [[ "$DRY_RUN" -eq 1 ]]; then
  echo "dry-run enabled: no GitHub mutations or network calls."
  echo "repo=$REPO"
  echo "gh_timeout=${GH_TIMEOUT_SECONDS}s"
  echo "No live PR sweep performed."
  exit 0
fi

if ! ME=$(run_gh api user -q '.login'); then
  echo "Unable to authenticate with GitHub CLI. Set GH auth (gh auth login) or pass --dry-run." >&2
  DRY_RUN=1
fi

if [[ "$DRY_RUN" -eq 1 ]]; then
  echo "Unable to authenticate with GitHub CLI; continuing in dry-run mode."
  echo "repo=$REPO"
  echo "gh_timeout=${GH_TIMEOUT_SECONDS}s"
  echo "No live PR sweep performed."
  exit 0
fi

CYCLES=0


status_badge() {
  local merge_state="$1"
  local fail_total="$2"
  local fail_non_cb="$3"
  local cr_rate="$4"
  local mergeable="$5"

  if [[ "$fail_total" == "0" && "$merge_state" == "CLEAN" && "$mergeable" == "MERGEABLE" ]]; then
    echo "READY"
    return
  fi
  if [[ "$mergeable" != "MERGEABLE" ]]; then
    echo "BLOCKED-MERGE"
    return
  fi
  if [[ "$fail_total" -gt 0 && "$fail_non_cb" -eq 0 && "$cr_rate" -gt 0 ]]; then
    echo "BLOCKED-CODERABBIT"
    return
  fi
  if [[ "$fail_total" -gt 0 ]]; then
    echo "BLOCKED-CI"
    return
  fi
  if [[ "$merge_state" != "CLEAN" ]]; then
    echo "BLOCKED-MERGE"
    return
  fi
  echo "BLOCKED"
}

recent_my_request() {
  local pr="$1"
  local cutoff
  cutoff=$(compute_cutoff_timestamp "$RETRY_WINDOW_SECONDS")

  if [[ "$DRY_RUN" -eq 1 ]]; then
    return 1
  fi

  local recent
  recent=$(run_gh pr view "$pr" --repo "$REPO" --json comments -q "[.comments[] | select(.author.login == \"$ME\" and .body == \"@coderabbitai review\" and .createdAt >= \"$cutoff\") ] | length" || echo 0)
  [[ "$recent" != "0" ]]
}

ping_coderabbit_if_needed() {
  local pr="$1"
  local fail_total="$2"
  local fail_non_cb="$3"
  local cr_rate="$4"

  if [[ "$AUTO_PING" -eq 0 ]]; then
    return
  fi
  if [[ "$fail_total" -eq 0 || "$fail_non_cb" -ne 0 || "$cr_rate" -eq 0 ]]; then
    return
  fi
  if recent_my_request "$pr"; then
    return
  fi

  if run_gh pr comment "$pr" --body "@coderabbitai review" >/dev/null; then
    echo "    └ re-triage sent to CodeRabbit"
  fi
}

classify_cycle() {
  local pr_json="$1"
  local num branch merge_state
  num=$(echo "$pr_json" | jq_run -r '.number')
  branch=$(echo "$pr_json" | jq_run -r '.headRefName')

  local checks
  if [[ "$DRY_RUN" -eq 1 ]]; then
    checks='[]'
  else
    checks=$(run_gh pr checks "$num" --repo "$REPO" --json name,state || echo '[]')
  fi
  local fail_total fail_non_coderabbit
  fail_total=$(echo "$checks" | jq_run '[.[] | select(.state=="FAILURE" or .state=="ACTION_REQUIRED")] | length')
  fail_non_coderabbit=$(echo "$checks" | jq_run '[.[] | select((.state=="FAILURE" or .state=="ACTION_REQUIRED") and (.name | ascii_downcase) != "coderabbit")] | length')

  local comment_json
  if [[ "$DRY_RUN" -eq 1 ]]; then
    comment_json='{"comments":[]}'
  else
    comment_json=$(run_gh pr view "$num" --json comments --repo "$REPO" || echo '{"comments":[]}')
  fi
  local cr_rate
  cr_rate=$(echo "$comment_json" | jq_run '[.comments[] | select(.author.login=="coderabbitai" and (.body | test("rate limit|secondary rate limit|quota|retry-after|abuse"; "i")))] | length')

  local meta
  if [[ "$DRY_RUN" -eq 1 ]]; then
    meta='{"mergeStateStatus":"UNKNOWN","mergeable":"UNKNOWN"}'
  else
    meta=$(run_gh pr view "$num" --repo "$REPO" --json mergeStateStatus,mergeable || echo '{"mergeStateStatus":"UNKNOWN","mergeable":"UNKNOWN"}')
  fi
  merge_state=$(echo "$meta" | jq_run -r '.mergeStateStatus')
  local mergeable
  mergeable=$(echo "$meta" | jq_run -r '.mergeable')

  local state
  state=$(status_badge "$merge_state" "$fail_total" "$fail_non_coderabbit" "$cr_rate" "$mergeable")

  printf '%-4s | %-24s | %-16s | %-7s | %-5s / %-12s / %-3s | %s\n' \
    "#$num" "${branch:0:24}" "$merge_state" "$mergeable" "$fail_total" "$fail_non_coderabbit" "$cr_rate" "$state"

  if [[ "$state" == BLOCKED-CODERABBIT* ]]; then
    ping_coderabbit_if_needed "$num" "$fail_total" "$fail_non_coderabbit" "$cr_rate"
  fi
}

while true; do
  CYCLES=$((CYCLES + 1))
  ts=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
  echo "[$ts] open PR sweep #$CYCLES"
  echo "PR   | branch                    | mergeState       | mergeable | CI(fail/other/rate) | state"
  echo "--------------------------------------------------------------------------------"

  open_json=$(run_gh pr list \
    --repo "$REPO" \
    --state open \
    --limit 100 \
    --json number,title,headRefName,mergeStateStatus,mergeable || echo '[]')

  if [[ -z "$open_json" ]]; then
    echo "No open PRs in $REPO"
  else
    while IFS= read -r pr_json; do
      [[ -z "$pr_json" ]] && continue
      classify_cycle "$pr_json"
    done < <(echo "$open_json" | jq_run -c '.[]')
  fi

  if [[ "$ONCE" -eq 1 ]]; then
    break
  fi
  if [[ "$MAX_STATES" -gt 0 && "$CYCLES" -ge "$MAX_STATES" ]]; then
    break
  fi

  echo "sleep ${SLEEP_SECONDS}s"
  sleep "$SLEEP_SECONDS"
done
