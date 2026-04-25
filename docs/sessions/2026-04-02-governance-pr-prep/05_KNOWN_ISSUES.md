# Known Issues

- The `chore/governance-pr-ready` worktree was created at an awkward nested path under
  `heliosCLI/heliosCLI/.worktrees/`. The branch is usable, but the path should be normalized after
  the governance lane is safely captured.
- The root checkout is still polluted by untracked nested surfaces and should not be used as the
  PR branch.
- This branch documents the intended ruleset posture, but remote GitHub protection settings still
  need a separate sync pass.
