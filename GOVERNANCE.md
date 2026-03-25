# heliosHarness Governance Model

## Purpose

**heliosHarness** is a private research & planning monorepo for developing, analyzing, and organizing the heliosCLI system architecture. It does **not** contain production code - instead, it researches, prototypes, and defines how to extend other projects.

## Project Shelf Model

Like `kush/` - heliosHarness serves as a **project shelf** for organizing dependencies and research.

## Module Categories

### 1. Clones (forked external projects)

```
clones/
├── codex/          # Forked Codex CLI research
├── goose/           # Forked Goose research
├── cline/           # Forked Cline research
├── aider/           # Forked Aider research
├── opencode/         # Forked OpenCode research
└── [other forks/]
```

### 2. Extensions (plugin systems)

```
extensions/
├── codex-plugins/      # Codex CLI extensibility
├── harbor-plugins/      # Harbor framework plugins
├── portage-plugins/     # Portage module system
└── [project]-plugins/
```

### 3. Plugins (modular extension system)

```
plugins/
├── protocol/        # Plugin protocol definitions
├── loader/          # Plugin loader/runtimes
├── registry/         # Plugin registry
└── templates/       # Plugin templates
```

### 4. Submodules (component definitions)

```
modules/
├── harness_core/    # Core harness interfaces
├── adapters/        # Adapter definitions
├── handlers/        # Handler specs
└── validators/      # Validation contracts
```

### 5. Research & Analysis

```
research/
├── [domain]-analysis/   # Research documents
├── [project]-specs/     # Specification documents
└── prototypes/           # Prototype code/tests
```

## Extension Pattern

### Plugin Contract

```python
# plugins/protocol/base.py
class Plugin(Protocol):
    name: str
    version: str

    def initialize(self, config: dict) -> None: ...
    def execute(self, ctx: Context) -> Result: ...
    def shutdown(self) -> None: ...
```

### Extension Points

- **codex extensions** → Extend heliosCLI
- **harbor plugins** → Harbor framework integration
- **portage modules** → Portage package system
- **custom adapters** → External system connectors

## Governance Rules

1. **No production code** - Research/prototypes only
2. **Clear provenance** - Document source projects
3. **Plugin-first** - Use extension patterns over hardcoded deps
4. **Modular** - Independent, composable components
5. **Documented** - ADRs for architectural decisions

## Directory Structure

```
heliosHarness/
├── clones/           # Forked external projects
├── plugins/          # Plugin system
├── extensions/       # Extension definitions
├── modules/          # Component interfaces
├── research/         # Analysis documents
├── prototypes/       # Experimental code
├── specs/            # Specification documents
└── artifacts/        # Generated artifacts
```

## Plugin Registry Example

```yaml
# plugins/registry.yaml
plugins:
  - name: codex-extension
    type: helioscli-extension
    source: clones/codex
    path: extensions/codex-plugins/

  - name: harbor-adapter
    type: harbor-plugin
    source: portage/harbor
    path: extensions/harbor-plugins/
```
