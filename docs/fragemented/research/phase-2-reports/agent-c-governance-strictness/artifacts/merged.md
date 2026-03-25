# Merged Fragmented Markdown

## Source: research/phase-2-reports/agent-c-governance-strictness/artifacts/strictness-equivalence-matrix.md

# Lane C: strictness/parent-child strict-full equivalence

## Task decisions
| target | child_evidence | parent_evidence | inferred_map | decision |
|---|---|---|---|---|
| codex | AGENTS.md + .github/workflows/rust-ci.yml (fmt/clippy/test) + package scripts | workflow policy | static=test/build inferred from command families | WARN |
| opencode | incomplete script matrix | release-only workflows + archived status | limited mandatory gate evidence | WARN |
| goose | Justfile + workflows (lint/test/build/security checks + check-everything) | explicit parent policy in docs | strict command family coverage present | PASS |
| kilocode | `.github/workflows/code-qa.yml`, monorepo pnpm script matrix | repository governance docs | lint/test/build/typecheck evidenced | PASS |

## Decision model (`quality:strict-full` parity)
- PASS requires static + test + build + quality evidence or clear fallback equivalents.
- WARN allows one bucket infer from parent/explicit fallback with rationale.
- BLOCK if mandatory buckets cannot be mapped and no fallback exists.

## Parent/child precedence
1. Child AGENTS and CI files
2. Child script/Taskfile commands
3. Parent workflow docs
4. Manual fallback (requires WARN reason + owner)

## ECV decision summary
- Codex/Opencode treated WARN due token-grammar mismatch (`strict-full` not explicit) and partial parent/child mapping.
- Goose and Kilo treated PASS for phase-2 gate with explicit evidence.

---

