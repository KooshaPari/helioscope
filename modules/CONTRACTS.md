# Module Contracts

## Base Module Interface

All modules must implement this interface:

```python
from abc import ABC, abstractmethod
from typing import Any, Dict, Optional

class Module(ABC):
    """Base module interface."""
    
    @property
    @abstractmethod
    def name(self) -> str:
        """Module name."""
        pass
    
    @property
    @abstractmethod
    def version(self) -> str:
        """Module version."""
        pass
    
    @property
    def dependencies(self) -> list[str]:
        """Module dependencies."""
        return []
    
    @abstractmethod
    def initialize(self, config: Dict[str, Any]) -> None:
        """Initialize module with configuration."""
        pass
    
    @abstractmethod
    def execute(self, context: Dict[str, Any]) -> Dict[str, Any]:
        """Execute module logic."""
        pass
    
    @abstractmethod
    def shutdown(self) -> None:
        """Cleanup resources."""
        pass
    
    def validate(self) -> bool:
        """Validate module configuration."""
        return True
```

## Handler Contract

```python
class Handler(ABC):
    """Handler interface for processing requests."""
    
    @property
    def event_types(self) -> list[str]:
        """Events this handler subscribes to."""
        return []
    
    async def handle(self, event: Event) -> Response:
        """Process event and return response."""
        pass
    
    def cleanup(self) -> None:
        """Release resources."""
        pass
```

## Adapter Contract

```python
class Adapter(ABC):
    """Adapter for external systems."""
    
    @property
    def name(self) -> str:
        pass
    
    async def connect(self) -> None:
        pass
    
    async def disconnect(self) -> None:
        pass
    
    async def send(self, message: Message) -> Response:
        pass
    
    def health_check(self) -> HealthStatus:
        pass
```

## Validator Contract

```python
class Validator(ABC):
    """Input/output validation."""
    
    @property
    def schema(self) -> dict:
        pass
    
    def validate(self, data: Any) -> ValidationResult:
        pass
    
    def validate_response(self, response: Any) -> bool:
        pass
```

## Registry Pattern

```python
class ModuleRegistry:
    """Central module registration."""
    
    def register(self, module: Module) -> None:
        ...
    
    def get(self, name: str) -> Module:
        ...
    
    def list_modules(self) -> list[str]:
        ...
    
    def unregister(self, name: str) -> None:
        ...
```
