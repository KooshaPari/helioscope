# heliosHarness - Research Project

> **Status**: Research/POC project. Production harness moved to `helios-cli`.

## Project Type

This is a **research and experimentation** project for the helios harness system. It contains:
- Python harness implementations (research/原型)
- Rust crate experiments
- Cloned references (clones/)
- Benchmarks and benchmarks

## Production Code

**Production harness is in `clones/helios-cli/`**

The main helios-cli production code is located at:
```
clones/helios-cli/
```

## Key Directories

| Directory | Purpose |
|-----------|---------|
| `harness/src/harness/` | Python harness (research) |
| `crates/` | Rust crates (research) |
| `clones/helios-cli/` | **Production CLI** |
| `clones/` | Reference implementations |

## Commands

```bash
# Python tests
cd harness && uv run pytest tests

# Rust tests
cargo test --workspace

# Check project
task check
```

## Governance

This is a research sandbox. Production governance applies to `clones/helios-cli/`.
