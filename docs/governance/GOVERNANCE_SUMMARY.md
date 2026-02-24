# heliosHarness Governance Summary

> **Status**: Research project. Production at `clones/helios-cli/`.

## Project Type

- **Classification**: Research/POC
- **Production Code**: `clones/helios-cli/`
- **This Directory**: Experiments, prototypes, benchmarks

## Architecture

```
heliosHarness/
├── harness/src/harness/  # Python research
├── crates/               # Rust experiments  
├── clones/               # Reference implementations
│   └── helios-cli/       # PRODUCTION
└── docs/                # Research docs
```

## Governance

Since this is a research project:
- Lighter governance than production
- Focus on experimentation and learning
- Production rules apply in `clones/helios-cli/`

## Commands

```bash
# Python research
task py:test

# Rust experiments
task rust:test

# Check
task check
```

## References

- Production: `clones/helios-cli/`
- Taskfile: Standard (see Taskfile.yml)
