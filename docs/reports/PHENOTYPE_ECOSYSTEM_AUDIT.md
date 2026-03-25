# Phenotype Ecosystem Shared Packages Audit

**Date:** 2026-03-02  
**Scope:** Audit of cross-org shared governance and packages

---

## 1. phenotype-go-kit

**Location:** `/Users/kooshapari/CodeProjects/Phenotype/repos/template-commons/phenotype-go-kit`

### Structure
- **Module:** `github.com/KooshaPari/phenotype-go-kit`
- **Go Version:** 1.22
- **Files:** 4 source files (in `pkg/auth/`)
- **LOC:** ~63 lines of production code

### Contents
| File | Purpose |
|------|---------|
| `go.mod` | Go module definition |
| `go.sum` | Dependency checksums |
| `pkg/auth/base_token_storage.go` | Token storage abstraction |
| `README.md` | Documentation |

### Versioning & Status
- **Versioned:** Yes (part of template-commons suite)
- **Tagged Releases:** Included in template-commons git tags (e.g., `phenotype-go-auth/v0.1.0`)
- **Status:** Minimal implementation; auth abstraction only

### Observations
- Go package is very small and focused on token storage
- Part of decomposed modular suite (see template-commons recent refactor: `refactor: decompose monolithic kits into standalone packages (#7)`)
- No independent npm/go.dev publishing visible; consumed via git imports

---

## 2. template-commons

**Location:** `/Users/kooshapari/CodeProjects/Phenotype/repos/template-commons`

### Structure
Multi-package repo with 15 language/framework-specific packages:

#### Python Packages (PyPI-versioned)
| Package | Version | LOC | Purpose |
|---------|---------|-----|---------|
| `phenotype-py-kit` | 0.1.0 | 717 | Unified Python toolkit (api, config, logging, testing) |
| `phenotype-logging` | 0.1.0 | 172 | Logging utilities |
| `phenotype-api` | 0.1.0 | 209 | API client/server base |
| `phenotype-config` | 0.1.0 | 133 | Config management |
| `phenotype-testing` | 0.1.0 | 112 | Testing helpers |

#### Go Packages (go.mod-versioned)
| Package | Files | Purpose |
|---------|-------|---------|
| `phenotype-go-kit` | 4 | Token storage & auth |
| `phenotype-go-auth` | 4 | OAuth token handling |
| `phenotype-go-config` | 3 | Viper config wrapper |
| `phenotype-go-cli` | 3 | Cobra CLI helpers |
| `phenotype-go-middleware` | 3 | HTTP middleware stack |

#### TypeScript
| Package | Version | Purpose |
|---------|---------|---------|
| `@phenotype/toolkit` | 1.0.0 | Logger, config, index exports |

#### Multi-Language
| Package | Targets | Purpose |
|---------|---------|---------|
| `phenotype-id` | go, python, typescript | ID generation/validation |

### Repository Metrics
- **Total Source Files:** 38 source code files
- **Total LOC:** ~3,033 lines (Python + Go + TypeScript)
- **Root Governance:** `CLAUDE.md` (50+ lines, delegation & governance) + `AGENTS.md`

### Versioning Strategy
- **Python:** Semantic versioning via `pyproject.toml` (all v0.1.0)
- **Go:** go.mod module versioning (tag-based via git; tags in format `<pkg>/v<version>`)
- **TypeScript:** npm package.json (toolkit at 1.0.0)
- **Git Tags:** `phenotype-go-auth/v0.1.0`, `phenotype-go-config/v0.1.0`, `phenotype-go-middleware/v0.1.0`

### Recent Activity
- Latest: `chore(agentops): onboard policy federation artifacts`
- Significant refactor: decomposed monolithic kits into standalone packages (commit a87f8ce)
- Active governance: contracts for branch protection, workflow consistency

### Observations
- **Multi-language coverage:** Python (5), Go (5), TypeScript (1), Multi (1)
- **Governance-first:** Heavy emphasis on contracts, branch protection, consistency
- **Decomposed design:** Moved from monolithic to modular packages
- **Version inconsistency:** Python packages at 0.1.0 (pre-release); TypeScript toolkit at 1.0.0 (stable)
- **No central registry:** Consumed via git imports or local npm/pip; no pub.dev or PyPI publishing visible

---

## 3. phenotypeActions

**Location:** `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotypeActions`

### Structure
Shared GitHub Actions and reusable workflows for Phenotype repositories.

#### Actions Available
| Action | File | Purpose |
|--------|------|---------|
| `review-orchestrator` | `actions/review-orchestrator/action.yml` | Trigger review bots with cooldown markers & bounded retries |
| `template-sync` | `actions/template-sync/action.yml` | Sync template files across repos |

#### Workflow Definitions
| Workflow | File | Purpose |
|----------|------|---------|
| review-wave-orchestrator | `.github/workflows/review-wave-orchestrator.yml` | Multi-wave review bot orchestration |
| policy-gate | `.github/workflows/policy-gate.yml` | PR policy enforcement |
| template-sync | `.github/workflows/template-sync.yml` | Template consistency sync |
| security-guard | `.github/workflows/security-guard.yml` | Security validation |
| security-guard-hook-audit | `.github/workflows/security-guard-hook-audit.yml` | Audit security hooks |
| validate-packaging | `.github/workflows/validate-packaging.yml` | Packaging validation |

### Repository Metrics
- **Total YAML Config Files:** 6 workflows + 2 actions + pre-commit config
- **Root Governance:** `README.md` + `CLAUDE.md`
- **Worktree Activity:** `PROJECT-wtrees/policy-gate/` (active development branch)

### Versioning & Release Strategy
- **Current Tag:** `v0` (minimal versioning)
- **Recent Release:** `feat: add template-sync composite action and workflow (#5)` (commit e748fb7)
- **Consumption Pattern:** Pinned to release tags or full commit SHA
  ```yaml
  uses: KooshaPari/phenotypeActions/.github/workflows/review-wave-orchestrator.yml@v0
  ```

### Recent Activity
Latest commits show:
1. Stabilization of review-wave runtime context
2. Template-sync composite action addition
3. CI/security hardening (GH_TOKEN context)
4. Governance bootstrap (CLAUDE.md, branch policies)

### Observations
- **Minimal versioning:** Only one tag (`v0`); should adopt semantic versioning
- **Actively maintained:** Recent activity in policy-gate worktree for improvements
- **Integration pattern clear:** Composite actions + reusable workflows reduce copy/paste
- **Security-conscious:** Multiple security-guard workflows for hook auditing
- **Worktree discipline:** Using `PROJECT-wtrees/policy-gate/` for development

---

## Cross-Repository Patterns

### Governance Alignment
1. **CLAUDE.md in all three:** Delegation policy, child-agent patterns, stability focus
2. **Worktree discipline:** Both phenotypeActions and template-commons use `-wtrees` for feature work
3. **Tag-based versioning:** Repos use git tags as primary versioning mechanism
4. **No central registry:** Packages consumed via git imports; PyPI/npm publishing not visible

### Package Usage Flow
```
heliosCLI (and other projects)
  ├─ imports: phenotype-go-* packages (go.mod, git tag)
  ├─ imports: @phenotype/toolkit (npm, git tag)
  └─ consumes: phenotypeActions workflows (GitHub Actions, git tag @v0)
```

### Versioning Inconsistency
- **Python packages:** All at v0.1.0 (pre-release stance)
- **TypeScript toolkit:** At 1.0.0 (stable)
- **Go packages:** Tagged via git (`phenotype-go-auth/v0.1.0`)
- **Actions:** Single tag (`v0`), no semantic versioning

---

## Summary Table

| Repo | Type | Files | LOC | Versioning | Release Strategy | Status |
|------|------|-------|-----|------------|------------------|--------|
| **phenotype-go-kit** | Go lib | 4 | ~63 | go.mod | Git tag | Minimal, focused |
| **template-commons** | Multi-lang suite | 38 | ~3,033 | Semantic (v0.1.0, 1.0.0) | Git tags per package | Active, decomposed |
| **phenotypeActions** | Workflows/Actions | 8+ YAML | N/A | v0 (insufficient) | Single tag | Active, needs semver |

---

## Recommendations

### Immediate (High Priority)
1. **phenotypeActions:** Adopt semantic versioning
   - Current state: Only `v0` tag
   - Action: Tag commits as `v0.1.0`, `v0.2.0`, etc. for clarity
   - Reason: Consuming projects need clear upgrade paths

2. **template-commons Python packages:** Clarify pre-release intent
   - All at v0.1.0; confirm if intentional or blocking release
   - If blocking: roadmap for v1.0.0
   - If intentional: document pre-release stability guarantees

3. **Cross-repo publishing:** Document consumption patterns
   - Create shared CONSUMPTION.md or INTEGRATION.md
   - Example: "Go packages consumed via go.mod with git tags; TypeScript via npm; actions via GitHub Actions"

### Medium Priority
1. **Go packages on go.dev:** Publish phenotype-go-* packages to pkg.go.dev for discoverability
2. **Python packages on PyPI:** Publish phenotype-logging, phenotype-api, etc. for pip install
3. **Unified versioning:** Align all packages to same major.minor.patch scheme (e.g., all at 0.1.0 → 1.0.0)

### Long-Term (Stability)
1. **Central registry:** Consider publishing packages to public registries (PyPI, npm, go.dev) for ecosystem visibility
2. **Breaking change policy:** Document breaking changes and upgrade paths in CHANGELOG.md per repo
3. **Dependency audit:** Regular audits of transitive dependencies across all packages

---

**Generated:** 2026-03-02 | **Audit Coverage:** Complete structural and versioning review
