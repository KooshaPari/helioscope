"""Plugin system for heliosHarness extensions.

Defines the plugin protocol and extension points for modular code.
"""

from typing import Protocol, runtime_checkable
from dataclasses import dataclass
from enum import Enum
from abc import ABC, abstractmethod
from pathlib import Path


class PluginType(Enum):
    """Types of plugins."""
    HELIOS_CLI_EXTENSION = "helioscli_extension"
    HARBOR_PLUGIN = "harbor_plugin"
    PORTAGE_MODULE = "portage_module"
    ADAPTER = "adapter"
    TEMPLATE = "template"


@dataclass
class PluginMetadata:
    """Plugin metadata."""
    name: str
    version: str
    plugin_type: PluginType
    source_project: str
    description: str = ""
    author: str = ""
    tags: list[str] = None
    
    def __post_init__(self):
        if self.tags is None:
            self.tags = []


@dataclass
class PluginConfig:
    """Plugin configuration."""
    enabled: bool = True
    priority: int = 100
    settings: dict = None
    
    def __post_init__(self):
        if self.settings is None:
            self.settings = {}


@runtime_checkable
class Plugin(Protocol):
    """Base plugin protocol."""
    
    @property
    def metadata(self) -> PluginMetadata:
        """Plugin metadata."""
        ...
    
    def initialize(self, config: dict) -> None:
        """Initialize the plugin with configuration."""
        ...
    
    def execute(self, *args, **kwargs):
        """Execute the plugin."""
        ...
    
    def shutdown(self) -> None:
        """Cleanup resources."""
        ...


    def validate(self) -> bool:
        """Validate plugin setup."""
        return True


class PluginLoader:
    """Loads and manages plugins."""
    
    def __init__(self, plugin_dir: Path = None):
        self.plugin_dir = plugin_dir or Path("plugins")
        self._plugins: dict[str, Plugin] = {}
        self._metadata: dict[str, PluginMetadata] = {}
    
    def discover(self) -> list[PluginMetadata]:
        """Discover available plugins."""
        discovered = []
        
        if not self.plugin_dir.exists():
            return discovered
            
        for plugin_path in self.plugin_dir.rglob("plugin.yaml"):
            with open(plugin_path) as f:
                # Parse YAML and create metadata
                pass
        
        return discovered
    
    def load(self, name: str) -> Plugin:
        """Load a plugin by name."""
        if name in self._plugins:
            return self._plugins[name]
        
        # Dynamic import
        module = __import__(f"plugins.{name}", fromlist=["Plugin"])
        plugin = module.Plugin()
        
        self._plugins[name] = plugin
        return plugin
    
    def unload(self, name: str) -> None:
        """Unload a plugin."""
        if name in self._plugins:
            self._plugins[name].shutdown()
            del self._plugins[name]


class ExtensionRegistry:
    """Registry for extension points."""
    
    def __init__(self):
        self._extensions: dict[str, list[type]] = {}
    
    def register(self, extension_point: str, extension: type):
        """Register an extension for a point."""
        if extension_point not in self._extensions:
            self._extensions[extension_point] = []
        self._extensions[extension_point].append(extension)
    
    def get_extensions(self, extension_point: str) -> list[type]:
        """Get all extensions for a point."""
        return self._extensions.get(extension_point, [])


# Extension points
EXTENSION_POINTS = {
    "codex_command": "Codex CLI command extension",
    "harbor_adapter": "Harbor system adapter",
    "portage_module": "Portage module extension",
    "template_renderer": "Template renderer extension",
}
