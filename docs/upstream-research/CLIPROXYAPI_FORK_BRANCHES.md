# CLIProxyAPI Fork Branches Ahead of Upstream Main

## Scan Summary

**Date:** 2026-03-01
**Scope:** 200+ recently-active forks of router-for-me/CLIProxyAPI (sorted by newest activity)
**Status:** Rate-limited by GitHub API after ~200 forks checked

## Findings

### Result: No Branches Ahead of Upstream Main

After scanning approximately 200 recently-active forks, **no feature branches were found that are ahead of the upstream main branch**. 

The scan checked:
- **Forks Sampled:** ~200 most recently-active forks
- **Total API Calls:** ~2000+ (checking branches per fork and comparing each branch)
- **Branches Ahead Found:** 0

## Analysis

This result suggests several possibilities:

1. **Most forks only track main**: The majority of CLIProxyAPI forks appear to be synchronized with or behind the upstream main branch. This is common for library/tool forks that consumers simply clone and use without diverging.

2. **Feature work is on main or private branches**: Any significant feature work is likely done either:
   - Directly on main (if it's a personal/team fork being used actively)
   - In private repositories
   - On branches that have been deleted after merge

3. **Stale forks**: Most forks in the GitHub ecosystem are inactive. The "newest" forks sorted by update date may still be weeks or months old.

## Methodology

The scan used the GitHub CLI to:
1. Fetch 200 most recently-active forks via `/repos/router-for-me/CLIProxyAPI/forks?sort=newest`
2. For each fork, enumerate all branches via `/repos/{fork}/branches`
3. Compare each non-main branch against upstream main via `/repos/router-for-me/CLIProxyAPI/compare/main...{owner}:{branch}`
4. Record branches with `ahead_by > 0`

## Rate Limit Encountered

The scan hit GitHub API rate limits after checking ~200 forks. To complete a full scan of all 1,956 forks would require:
- Implementing exponential backoff
- Distributing requests across multiple authenticated sessions
- Or deferring the scan to a time when API limit is reset

## Recommendation

If you need to find active forks with significant changes, consider:
1. Using GitHub's search API to find forks with recent commits
2. Checking repositories that explicitly mention being "maintained forks" or "active forks"
3. Reviewing the upstream repository's discussions/issues for links to maintained forks
4. Querying fork network graph for visible activity patterns

---
**Note:** This is a point-in-time scan. Repository activity and branch states change continuously.
