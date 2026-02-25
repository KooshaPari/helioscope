"""Request tracing with correlation IDs for distributed tracing.

Provides middleware for generating and propagating trace IDs across service calls.
"""

import uuid
import time
from contextvars import ContextVar
from dataclasses import dataclass, field
from typing import Optional, Dict, Any, Callable
from functools import wraps
from enum import Enum


class TraceContext:
    """Trace context for distributed tracing."""
    
    # Context variable for trace ID
    trace_id_var: ContextVar[Optional[str]] = ContextVar('trace_id', default=None)
    span_id_var: ContextVar[Optional[str]] = ContextVar('span_id', default=None)
    parent_span_var: ContextVar[Optional[str]] = ContextVar('parent_span', default=None)
    
    @classmethod
    def generate_id(cls) -> str:
        """Generate a unique ID."""
        return uuid.uuid4().hex[:16]
    
    @classmethod
    def start_trace(cls, parent_span: Optional[str] = None) -> Dict[str, str]:
        """Start a new trace."""
        trace_id = cls.generate_id()
        span_id = cls.generate_id()
        
        cls.trace_id_var.set(trace_id)
        cls.span_id_var.set(span_id)
        cls.parent_span_var.set(parent_span)
        
        return {
            "trace_id": trace_id,
            "span_id": span_id,
            "parent_span": parent_span or ""
        }
    
    @classmethod
    def get_trace_id(cls) -> Optional[str]:
        """Get current trace ID."""
        return cls.trace_id_var.get()
    @classmethod
    def get_span_id(cls) -> Optional[str]:
        """Get current span ID."""
        return cls.span_id_var.get()
    
    @classmethod
    def clear(cls):
        """Clear trace context."""
        cls.trace_id_var.set(None)
        cls.span_id_var.set(None)
        cls.parent_span_var.set(None)


@dataclass
class Span:
    """Represents a trace span."""
    name: str
    trace_id: str
    span_id: str
    parent_span_id: Optional[str]
    start_time: float = field(default_factory=time.time)
    end_time: Optional[float] = None
    tags: Dict[str, Any] = field(default_factory=dict)
    logs: list = field(default_factory=list)
    
    def finish(self):
        """Mark span as finished."""
        self.end_time = time.time()
    
    def duration_ms(self) -> float:
        """Get span duration in milliseconds."""
        if self.end_time:
            return (self.end_time - self.start_time) * 1000
        return (time.time() - self.start_time) * 1000
    
    def add_tag(self, key: str, value: Any):
        """Add a tag to the span."""
        self.tags[key] = value
    
    def add_log(self, message: str, **kwargs):
        """Add a log event to the span."""
        self.logs.append({
            "timestamp": time.time(),
            "message": message,
            **kwargs
        })
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert span to dictionary."""
        return {
            "name": self.name,
            "trace_id": self.trace_id,
            "span_id": self.span_id,
            "parent_span_id": self.parent_span_id,
            "start_time": self.start_time,
            "end_time": self.end_time,
            "duration_ms": self.duration_ms(),
            "tags": self.tags,
            "logs": self.logs
        }


class Tracer:
    """Tracer for creating and managing spans."""
    
    def __init__(self, service_name: str = "helios"):
        self.service_name = service_name
        self._spans: Dict[str, Span] = {}
    
    def start_span(self, name: str, tags: Optional[Dict[str, Any]] = None) -> Span:
        """Start a new span."""
        parent_span = TraceContext.get_span_id()
        context = TraceContext.start_trace(parent_span)
        
        span = Span(
            name=name,
            trace_id=context["trace_id"],
            span_id=context["span_id"],
            parent_span_id=parent_span
        )
        
        if tags:
            for key, value in tags.items():
                span.add_tag(key, value)
        
        span.add_tag("service.name", self.service_name)
        
        self._spans[span.span_id] = span
        return span
    
    def finish_span(self, span: Span):
        """Finish a span."""
        span.finish()
        TraceContext.clear()
    
    def get_span(self, span_id: str) -> Optional[Span]:
        """Get a span by ID."""
        return self._spans.get(span_id)
    
    def get_all_spans(self) -> list:
        """Get all spans."""
        return list(self._spans.values())
    
    def clear(self):
        """Clear all spans."""
        self._spans.clear()


# Global tracer
_tracer: Optional[Tracer] = None


def get_tracer(service_name: str = "helios") -> Tracer:
    """Get or create the global tracer."""
    global _tracer
    if _tracer is None:
        _tracer = Tracer(service_name)
    return _tracer


def trace(name: str, tags: Optional[Dict[str, Any]] = None):
    """Decorator to trace a function."""
    def decorator(func: Callable):
        @wraps(func)
        def wrapper(*args, **kwargs):
            tracer = get_tracer()
            span = tracer.start_span(name, tags)
            try:
                result = func(*args, **kwargs)
                span.add_tag("error", False)
                return result
            except Exception as e:
                span.add_tag("error", True)
                span.add_tag("error.message", str(e))
                raise
            finally:
                tracer.finish_span(span)
        return wrapper
    return decorator


class TraceContextManager:
    """Context manager for tracing."""
    
    def __init__(self, name: str, tags: Optional[Dict[str, Any]] = None):
        self.name = name
        self.tags = tags or {}
        self.tracer = get_tracer()
        self.span: Optional[Span] = None
    
    def __enter__(self):
        """Enter context."""
        self.span = self.tracer.start_span(self.name, self.tags)
        return self.span
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        """Exit context."""
        if self.span:
            if exc_type:
                self.span.add_tag("error", True)
                self.span.add_tag("error.message", str(exc_val))
            self.tracer.finish_span(self.span)
