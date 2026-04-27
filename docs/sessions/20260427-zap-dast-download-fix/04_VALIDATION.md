# Validation

## Expected checks

- Workflow YAML parses.
- The `wget` command points at the latest release download URL.
- Checkout credentials are not persisted.

## Follow-up

- Re-run or observe the GitHub Actions alert after merge to confirm the failure mode moved past the dead download.
