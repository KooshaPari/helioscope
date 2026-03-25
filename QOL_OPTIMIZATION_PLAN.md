# Heavy Optimization & QOL Plan - heliosHarness

## 1. Build Speed Optimizations

### Cargo Config

```toml
# .cargo/config.toml
[build]
incremental = true
jobs = 4

[target.x86_64-apple-darwin]
linker = "clang"
rustflags = ["-C", "link-args=-fuse-ld=lld"]

[profile.dev]
opt-level = 0
debug = true

[profile.release]
lto = "thin"
codegen-units = 1
opt-level = 3
```

### Key Optimizations

| Tool          | Speedup   | Status       |
| ------------- | --------- | ------------ |
| mold linker   | 30-70%    | RECOMMEND    |
| sccache       | 50%+      | RECOMMEND    |
| cargo-check   | 3x faster | ALREADY USE  |
| cargo-nextest | 2-3x      | CONFIG READY |

---

## 2. Rust Analyzer Config

```json
// .vscode/settings.json
{
  "rust-analyzer.cargo.loadOutDirsFromCheck": true,
  "rust-analyzer.checkOnSave.command": "check",
  "rust-analyzer.procMacro.enable": true,
  "rust-analyzer.imports.mergeBehavior": "prefix",
  "rust-analyzer.completion.autoImport": true,
  "rust-analyzer.hoverActions.enable": true
}
```

### VSCode Extensions

- rust-analyzer
- rustfmt
- rust syntax
- error lens
- crates

---

## 3. CI/CD Optimizations

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install mold linker
        run: sudo apt-get install mold

      - name: Cache cargo
        uses: Swatinem/rust-cache@v2

      - name: Cache sccache
        uses: mozilla-actions/sccache-action@v0.0.6

      - name: Check
        run: cargo check --all-targets

      - name: Clippy
        run: cargo clippy --all-targets -- -D warnings

      - name: Test
        run: cargo test --all

      - name: Build
        run: cargo build --release
```

---

## 4. QOL Improvements

### Git Hooks (pre-commit)

```bash
#!/bin/bash
set -e
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo fmt --check
```

### Aliases

```bash
# ~/.cargo/config.toml
[alias]
c = "check"
ci = "clippy --all-targets -- -D warnings"
t = "test"
b = "build"
r = "run"
w = "test --workspace"
f = "fmt"
fa = "fmt --all"
```

### Makefile

```makefile
check:
	cargo check --all-targets

clippy:
	cargo clippy --all-targets -- -D warnings

test:
	cargo test --all

fmt:
	cargo fmt --all

lint: fmt check clippy

build:
	cargo build --all

.PHONY: check clippy test fmt lint build
```

---

## 5. Code Quality

### Clippy Lints

```toml
# clippy.toml
cognitive-complexity-threshold = 30
too-many-arguments-threshold = 8
type-complexity-threshold = 500
```

### Miri (undefined behavior)

```bash
cargo +nightly miri test
```

---

## 6. Performance Profiling

```bash
# Timing report
cargo +nightly build -Z timings

# Flamegraph
cargo flamegraph --bin <name>

# Benchmark
cargo bench
```

---

## Implementation Priority

| Priority | Item                | Impact |
| -------- | ------------------- | ------ |
| 1        | Add cargo config    | High   |
| 2        | Add aliases         | High   |
| 3        | Add CI with sccache | High   |
| 4        | Add Makefile        | Medium |
| 5        | VSCode settings     | Medium |
| 6        | Clippy config       | Low    |

---

## Scripts to Create

1. `.cargo/config.toml` - Build optimizations
2. `.cargo/aliases` - Shell shortcuts
3. `Makefile` - Common tasks
4. `.github/workflows/ci.yml` - CI pipeline
5. `.vscode/settings.json` - Editor config
6. `.pre-commit-config.yaml` - Git hooks
