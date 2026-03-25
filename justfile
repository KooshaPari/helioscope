set working-directory := "codex-rs"
set positional-arguments

# Display help
help:
    just -l

# `codex`
alias c := codex
codex *args:
    cargo run --bin codex -- "$@"

# `codex exec`
exec *args:
    cargo run --bin codex -- exec "$@"

# Run the CLI version of the file-search crate.
file-search *args:
    cargo run --bin codex-file-search -- "$@"

# Build the CLI and run the app-server test client
app-server-test-client *args:
    cargo build -p codex-cli
    cargo run -p codex-app-server-test-client -- --codex-bin ./target/debug/codex "$@"

# format code
fmt:
    cargo fmt -- --config imports_granularity=Item 2>/dev/null

fix *args:
    cargo clippy --fix --tests --allow-dirty "$@"

clippy:
    cargo clippy --tests "$@"

install:
    rustup show active-toolchain
    cargo fetch

# Run `cargo nextest` since it's faster than `cargo test`, though including
# --no-fail-fast is important to ensure all tests are run.
#
# Run `cargo install cargo-nextest` if you don't have it installed.
# Prefer this for routine local runs; use explicit `cargo test --all-features`
# only when you specifically need full feature coverage.
test:
    cargo nextest run --no-fail-fast

# Build and run Codex from source using Bazel.
# Note we have to use the combination of `[no-cd]` and `--run_under="cd $PWD &&"`
# to ensure that Bazel runs the command in the current working directory.
[no-cd]
bazel-codex *args:
    bazel run //codex-rs/cli:codex --run_under="cd $PWD &&" -- "$@"

[no-cd]
bazel-lock-update:
    bazel mod deps --lockfile_mode=update

[no-cd]
bazel-lock-check:
    ./scripts/check-module-bazel-lock.sh

bazel-test:
    bazel test //... --keep_going

bazel-remote-test:
    bazel test //... --config=remote --platforms=//:rbe --keep_going

build-for-release:
    bazel build //codex-rs/cli:release_binaries --config=remote

# Run the MCP server
mcp-server-run *args:
    cargo run -p codex-mcp-server -- "$@"

# Regenerate the json schema for config.toml from the current config types.
write-config-schema:
    cargo run -p codex-core --bin codex-write-config-schema

# Regenerate vendored app-server protocol schema artifacts.
write-app-server-schema *args:
    cargo run -p codex-app-server-protocol --bin write_schema_fixtures -- "$@"

# Feature-layer smoke checks for compile-time and optional adapters.
# Keep this intentionally small and fast so it runs both in PRs and local CI loops.
feature-matrix:
    cargo check -p codex-tui --no-default-features
    cargo check -p codex-tui --no-default-features --features debug-logs
    cargo check -p codex-cloud-tasks-client --no-default-features
    cargo check -p codex-cloud-tasks-client --no-default-features --features mock
    cargo check -p codex-otel --features disable-default-metrics-exporter

# Tail logs from the state SQLite database
log *args:
    if [ "${1:-}" = "--" ]; then shift; fi; cargo run -p codex-state --bin logs_client -- "$@"

# Standardized task-surface wrapper recipes.
surface-lint:
    bash ../scripts/task_surface.sh lint

surface-test:
    bash ../scripts/task_surface.sh test

surface-fmt:
    bash ../scripts/task_surface.sh fmt

surface-quality:
    bash ../scripts/task_surface.sh quality

# DevOps / Delivery helpers
devops-status:
    git -C .. status --short --branch
    git -C .. remote -v
    git -C .. log --oneline -n 5

devops-check *args:
    bash ../scripts/devops-checker.sh "$@"

devops-check-ci:
    bash ../scripts/devops-checker.sh --check-ci

devops-check-ci-summary:
    bash ../scripts/devops-checker.sh --check-ci --emit-summary

devops-push *args:
    bash ../scripts/push-helioscli-with-fallback.sh "$@"

devops-push-origin *args:
    bash ../scripts/push-helioscli-with-fallback.sh --skip-primary "$@"
