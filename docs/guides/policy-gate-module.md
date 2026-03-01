# Policy Gate Composite Action

This repository exposes the policy gate checks as a reusable composite action:

- Path: `.github/actions/policy-gate/action.yml`
- Purpose: enforce layered `fix/*` PR targeting policy and block merge commits in PR diff range.

## Inputs

- `head-ref` (required): PR head branch name (for example, `fix/my-change`).
- `base-ref` (required): PR base branch name (for example, `main`).
- `pr-labels` (required): JSON array string of PR label names (for example, `["layered-pr-exception"]`).

## Outputs

- `passed`: `true` when all gate checks pass.

## Usage

`actions/checkout` must use `fetch-depth: 0` for this action so branch history is available for merge-commit checks.

```yaml
jobs:
  policy-gate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Enforce layered fix PR policy
        uses: ./.github/actions/policy-gate
        with:
          head-ref: ${{ github.head_ref }}
          base-ref: ${{ github.base_ref }}
          pr-labels: ${{ toJson(github.event.pull_request.labels.*.name) }}
```

## Behavior

- Blocks `fix/*` pull requests that target `main` or `master` unless label `layered-pr-exception` is present.
- Fails if merge commits are detected in `origin/<base-ref>..HEAD`.
- Prints `Policy gate passed.` and emits `passed=true` on success.
