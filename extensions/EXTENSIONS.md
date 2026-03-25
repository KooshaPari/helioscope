# Extension Points for heliosCLI System

## Overview

Extensions allow heliosCLI (and harbor framework) to be extended without modifying core code. This document defines the extension system.

## Extension Types

### 1. CLI Commands

```yaml
# Extension point: codex_command
# Location: clones/codex/cli/extensions/
command:
  name: my-extension
  description: "My custom command"
  arguments:
    - name: input
      type: string
      required: true
  handler: extensions.commands.my_extension
```

### 2. Harbor Adapters

```yaml
# Extension point: harbor_adapter  
# Location: portage/harbor/adapters/
adapter:
  name: custom-backend
  type: http
  config:
    base_url: https://api.example.com
  handlers:
    - on_request
    - on_response
```

### 3. Portage Modules

```python
# Extension point: portage_module
# Location: portage/portage_modules/
class MyModule(PortageModule):
    name = "my_module"
    version = "1.0.0"
    
    def process(self, context):
        pass
```

### 4. Template Renderers

```python
# Extension point: template_renderer
class CustomRenderer(TemplateRenderer):
    extensions = [".custom"]
    
    def render(self, template, context):
        pass
```

## Extension Discovery

Extensions are discovered via:
1. `pyproject.toml` entry points
2. Extension directories (configurable)
3. External plugin registries

## Loading Order

1. Core extensions (built-in)
2. System extensions (`~/.helios/extensions/`)
3. Project extensions (`./helios_extensions/`)
4. Dynamic (runtime loaded)

## Example Extension Structure

```
extensions/
├── pyproject.toml
├── src/
│   └── my_extension/
│       ├── __init__.py
│       ├── plugin.yaml
│       ├── handlers.py
│       └── tests/
└── README.md
```

## plugin.yaml Schema

```yaml
name: my-extension
version: 1.0.0
helios_version: ">=2.0.0"
entry_points:
  commands:
    - my-command
  adapters:
    - my-adapter
dependencies:
  - package>=1.0
  - another-package
```

## Publishing Extensions

```bash
# Publish to PyPI
helios extension publish

# Install from PyPI
helios extension install my-extension
```
