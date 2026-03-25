# Changelog Process

This project uses [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## Scope

- Update `CHANGELOG.md` for every user-visible change.
- Keep entries in `## [Unreleased]` until a release is cut.
- Move unreleased entries into a versioned section at release time.

## Entry Types

Use only these headings under each release section:

- `### Added`
- `### Changed`
- `### Deprecated`
- `### Removed`
- `### Fixed`
- `### Security`

## Authoring Rules

- Write concise, behavior-focused entries.
- Prefer one bullet per meaningful change.
- Include code/document path context when useful.
- Do not include internal-only noise (format-only churn, refactors with no behavior change).

## Release Cut Checklist

1. Confirm `## [Unreleased]` is accurate and complete.
2. Create a new section in `CHANGELOG.md`: `## [X.Y.Z] - YYYY-MM-DD`.
3. Move unreleased entries under that new section.
4. Add an empty `## [Unreleased]` section back at the top.
5. Verify README links still point to:
   - `CHANGELOG.md`
   - `docs/guides/CHANGELOG_PROCESS.md`
   - `docs/reference/CHANGELOG_ENTRY_TEMPLATE.md`

## Template

Use `docs/reference/CHANGELOG_ENTRY_TEMPLATE.md` for new entries.
