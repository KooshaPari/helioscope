# SonarCloud workflow fix

## Goal

Fix the `SonarCloud failing on main` alert by repairing the GitHub Actions workflow in
`.github/workflows/sonarcloud.yml`.

## Scope

- Add the required Java distribution input to `actions/setup-java`.
- Make the scanner path persist across workflow steps.
- Reduce checkout cleanup risk in a repo with archived worktree baggage.

## Success criteria

- Workflow YAML is valid.
- The SonarCloud job can reach the scanner step.
- The alert-sync issue can be closed after the patch is merged.
