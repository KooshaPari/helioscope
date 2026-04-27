# Implementation

The workflow now uses:

- `actions/checkout@v4` with `persist-credentials: false`
- `actions/setup-java@v4` with `distribution: temurin` and Java 17
- `GITHUB_PATH` to expose the downloaded SonarScanner bin directory to later steps

These changes keep the job simple and avoid the two failure modes visible in the run log:
the missing setup-java distribution input and the non-persistent PATH export.
