"""Configuration management for heliosHarness.

Provides environment-based config with validation.
"""

import os
import json
import yaml
from dataclasses import dataclass, field
from typing import Any, Dict, Optional, Union
from pathlib import Path
from enum import Enum


class ConfigSource(Enum):
    """Configuration source."""
    ENV = "env"
    FILE = "file"
    DEFAULT = "default"


@dataclass
class Config:
    """Base configuration class."""
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary."""
        result = {}
        for key, value in self.__dict__.items():
            if isinstance(value, Enum):
                result[key] = value.value
            elif hasattr(value, 'to_dict'):
                result[key] = value.to_dict()
            else:
                result[key] = value
        return result
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'Config':
        """Create from dictionary."""
        return cls(**{k: v for k, v in data.items() if hasattr(cls, k)})


def get_env(key: str, default: Any = None, required: bool = False) -> Any:
    """Get environment variable with validation.
    
    Usage:
        debug = get_env("DEBUG", False)
        api_key = get_env("API_KEY", required=True)
    """
    value = os.environ.get(key, default)
    
    if required and value is None:
        raise EnvironmentError(f"Required environment variable {key} is not set")
    
    return value


def load_yaml(path: str) -> Dict[str, Any]:
    """Load YAML configuration file."""
    with open(path, 'r') as f:
        return yaml.safe_load(f)


def load_json(path: str) -> Dict[str, Any]:
    """Load JSON configuration file."""
    with open(path, 'r') as f:
        return json.load(f)


class ConfigManager:
    """Centralized configuration management."""
    
    def __init__(self, base_path: Optional[str] = None):
        self.base_path = Path(base_path) if base_path else Path.cwd()
        self._config: Dict[str, Any] = {}
        self._sources: Dict[str, ConfigSource] = {}
    
    def load_env_prefix(self, prefix: str = "HELIOS_"):
        """Load all environment variables with given prefix."""
        for key, value in os.environ.items():
            if key.startswith(prefix):
                config_key = key[len(prefix):].lower()
                self._config[config_key] = self._parse_value(value)
                self._sources[config_key] = ConfigSource.ENV
        
        return self
    
    def load_file(self, path: str, source: str = "yaml"):
        """Load configuration from file."""
        full_path = self.base_path / path if not Path(path).is_absolute() else Path(path)
        
        if source.lower() == "yaml":
            data = load_yaml(str(full_path))
        elif source.lower() == "json":
            data = load_json(str(full_path))
        else:
            raise ValueError(f"Unsupported file format: {source}")
        
        self._config.update(data)
        self._sources[path] = ConfigSource.FILE
        
        return self
    
    def get(self, key: str, default: Any = None) -> Any:
        """Get configuration value."""
        return self._config.get(key, default)
    
    def set(self, key: str, value: Any, source: ConfigSource = ConfigSource.DEFAULT):
        """Set configuration value."""
        self._config[key] = value
        self._sources[key] = source
    
    def _parse_value(self, value: str) -> Any:
        """Parse string value to appropriate type."""
        # Boolean
        if value.lower() in ('true', '1', 'yes'):
            return True
        if value.lower() in ('false', '0', 'no'):
            return False
        
        # None
        if value.lower() == 'none' or value.lower() == 'null':
            return None
        
        # Number
        try:
            if '.' in value:
                return float(value)
            return int(value)
        except ValueError:
            pass
        
        # Return as string
        return value
    
    def to_dict(self) -> Dict[str, Any]:
        """Export configuration as dictionary."""
        return self._config.copy()
    
    def source_of(self, key: str) -> ConfigSource:
        """Get the source of a configuration key."""
        return self._sources.get(key, ConfigSource.DEFAULT)


# Global config manager
_config_manager: Optional[ConfigManager] = None


def get_config_manager(base_path: Optional[str] = None) -> ConfigManager:
    """Get or create the global config manager."""
    global _config_manager
    if _config_manager is None:
        _config_manager = ConfigManager(base_path)
    return _config_manager


# Common configurations
@dataclass
class DatabaseConfig(Config):
    """Database configuration."""
    host: str = "localhost"
    port: int = 5432
    name: str = "helios"
    user: str = "helios"
    password: str = ""
    pool_size: int = 10


@dataclass
class RedisConfig(Config):
    """Redis configuration."""
    host: str = "localhost"
    port: int = 6379
    db: int = 0
    password: str = ""
    ssl: bool = False


@dataclass  
class APIConfig(Config):
    """API configuration."""
    host: str = "0.0.0.0"
    port: int = 8080
    workers: int = 4
    timeout: int = 30
    cors_origins: str = "*"


# Usage examples
if __name__ == "__main__":
    # From environment
    os.environ["HELIOS_DEBUG"] = "true"
    os.environ["HELIOS_PORT"] = "8080"
    
    cm = get_config_manager()
    cm.load_env_prefix("HELIOS_")
    
    print(f"Debug: {cm.get('debug')}")
    print(f"Port: {cm.get('port')}")
    
    # From file
    # cm.load_file("config.yaml")
