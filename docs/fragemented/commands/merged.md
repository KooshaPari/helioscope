# Merged Fragmented Markdown

## Source: commands/clone-playbook.md

# Helios Harness Clone Playbook (Phase-1)

## Objective
Create reproducible clone and repair flow for CLI/API/SDK research targets.

## Base workspace
`/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness`

## Mandatory clone map
Use full clones unless already present.

```bash
mkdir -p /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones
cd /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones

git clone https://github.com/openai/codex.git

git clone https://github.com/opencode-ai/opencode.git

git clone https://github.com/block/goose.git

git clone https://github.com/Kilo-Org/kilocode.git
```

### Additional local OSS candidate clones
```bash
git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones git clone https://github.com/KooshaPari/cliproxyapi-plusplus.git

git -C /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones git clone https://github.com/robert-fuchs/pluggedin-mcp-proxy.git
```

(Replace candidate remotes as needed for your final approved shortlist.)

## Verification checks (run for each clone)
```bash
git status --short

git branch --show-current

git remote -v

git log -1 --oneline
ls -1 .github/workflows
```

## Broken/partial clone repair
If clone ends with broken refs (`HEAD` invalid):

```bash
git fetch --all

git checkout -B main origin/main
# or fallback branch from fetched tags
```

If the cloned folder cannot be repaired and you must proceed from local mirror:

```bash
git remote set-url origin <mirror-url>
git fetch origin

git reset --hard origin/<branch>
```

## Evidence collection checklist
- `clone_state.md`
  - path, branch, commit, clean-tree, remotes
- `build_matrix.md`
  - install, build, test, quality, package commands discovered
- `governance.md`
  - AGENTS/instructions/contribution policy, skip rules, invitation-only notes
- `quality_outcome.md`
  - Pass/warn/fail with reason codes

## Cleanup and handoff
Delete stale partial clones before re-cloning to avoid hidden lockups:

```bash
rm -rf /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/<repo>
```

## Notes
- Prefer upstream canonical remotes over local mirrors for evidence consistency.
- For local monorepo extras (`temp-PRODVERCEL/485/zentest`, `kush`, etc.), capture only when explicitly included in E-lane discovery.

---

