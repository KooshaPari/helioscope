# Validation

## Expected checks

- Workflow file parses as YAML.
- The workflow contains `distribution: temurin`.
- The scanner install step writes to `GITHUB_PATH`.

## Follow-up

- Merge the PR and verify the alert-sync issue auto-closes or close it manually if the repository automation lags.
