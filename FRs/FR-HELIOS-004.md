# FR-HELIOS-004: Patch Application

## ID
- **FR-ID**: FR-HELIOS-004
- **Repository**: heliosCLI
- **Domain**: CODE

## Description

The system SHALL support applying code patches via CLI with conflict detection, backup creation, and rollback capability.

## Acceptance Criteria

- [ ] Applies patches to files
- [ ] Detects merge conflicts
- [ ] Creates backups before patching
- [ ] Supports rollback on failure

## Test References

| Test File | Function | FR Reference |
|-----------|----------|--------------|
| `codex-rs/core/tests/suite/apply_patch_cli.rs` | `test_patch_apply` | `// @trace FR-HELIOS-004` |

## Status

- **Current**: implemented
- **Since**: 2026-02-01
