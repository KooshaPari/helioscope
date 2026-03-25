# Tooling Migration Feature Flags

This repo supports feature-flagged tooling migration so lanes can opt in without
breaking default workflows.

## Flags

Set these in shell env (or `.env`) before running commands:

- `HELIOS_USE_BUN=0|1`
- `HELIOS_USE_JAQ=0|1`
- `HELIOS_USE_PY314=0|1`
- `HELIOS_USE_TS_NATIVE=0|1`

Defaults are `0` (disabled).

## Dispatcher

Use `scripts/tooling-dispatch.sh` to resolve tools by flag:

- `pm`: `bun` (flagged) or `pnpm` fallback
- `json`: `jaq` (flagged) or `jq` fallback
- `py`: `python3.14` (flagged) or `python3/python` fallback
- `ts`: `tsgo` (flagged) or `bunx/pnpm exec` fallback

Examples:

```bash
HELIOS_USE_BUN=1 scripts/tooling-dispatch.sh pm --version
HELIOS_USE_JAQ=1 scripts/tooling-dispatch.sh json --version
HELIOS_USE_PY314=1 scripts/tooling-dispatch.sh py --version
HELIOS_USE_TS_NATIVE=1 scripts/tooling-dispatch.sh ts --version
```

## Just targets

- `just tooling-flags`
- `just tooling-smoke-migration`
- `just tooling-smoke-bun`
- `just tooling-smoke-py314`
- `just tooling-smoke-ts-native`
- `just format-js`
- `just format-js-fix`
- `just stage-npm-packages -- --release-version <v> --package <name>`

## Script integration

`scripts/babysit-open-prs.sh` now resolves JSON parser via dispatcher (`json` mode),
so it can run with `jq` or `jaq` behind the same command path.

`scripts/stage-npm-packages.sh` routes npm staging through dispatcher-selected
Python and exports `CODEX_PREFERRED_PM` (`pnpm` or `bun`) based on feature flags.
`scripts/stage_npm_packages.py` and `codex-cli/scripts/build_npm_package.py`
validate `CODEX_PREFERRED_PM` and enforce it for package install/build steps.
