# Strictness Parity Matrix
| repo | branch | head commit | lockfile parity | strictness checks | decision |
|---|---|---|---|---|---|
| codex | main | 55fc075 | PASS ["clones/codex/codex-rs/Cargo.lock", "clones/codex/pnpm-lock.yaml"] | PASS | PASS |
| opencode | main | 73ee493 | PASS ["clones/opencode/go.sum"] | WARN | WARN |
| goose | main | 66d075050ed | FAIL [] | WARN | WARN |
| kilocode | main | 1440d1986d | FAIL ["clones/kilocode/jetbrains/host/pnpm-lock.yaml", "clones/kilocode/pnpm-lock.yaml"] | WARN | WARN |
| cliproxyapi-plusplus | main | f80eed6 | PASS ["clones/cliproxyapi-plusplus/go.sum"] | WARN | WARN |

## Evidence bundle
- strictness results: `research/phase-2-reports/agent-a-core-repo-harden/artifacts/strictness-results.md`
- lockfile parity: `research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-parity.txt`
- commit evidence: `research/phase-2-reports/agent-a-core-repo-harden/artifacts/commit-provenance.txt`
- lockfile baseline: `research/phase-2-reports/agent-a-core-repo-harden/artifacts/lockfile-inventory.txt`
