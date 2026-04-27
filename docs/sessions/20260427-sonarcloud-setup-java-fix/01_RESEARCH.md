# Research

## Live evidence

- Issue `#181` points to run `24937041945` and reports `SonarCloud failing on main`.
- The failing log shows `actions/setup-java@v3` with `Input required and not supplied: distribution`.
- The workflow also uses a transient `export PATH=...` in the scanner install step, which does not survive into the next GitHub Actions step.
- The repo already uses `persist-credentials: false` in `.github/workflows/scorecard.yml`.

## Decision

Use `distribution: temurin` with `actions/setup-java@v4`, and write the SonarScanner bin
directory to `GITHUB_PATH` so the next step can invoke `sonar-scanner`.
