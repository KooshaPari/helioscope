"""Structured logging configuration for heliosHarness.

Provides consistent JSON logging with correlation IDs for distributed tracing.
"""

import json
import logging
import sys
import os
from datetime import datetime
from typing import Any, Dict, Optional
from dataclasses import dataclass, field
from contextvars import ContextVar
import traceback


# Context variable for correlation ID
correlation_id_var: ContextVar[Optional[str]] = ContextVar("correlation_id", default=None)


@dataclass
class LogContext:
    """Additional context for logging."""

    correlation_id: Optional[str] = None
    user_id: Optional[str] = None
    session_id: Optional[str] = None
    request_id: Optional[str] = None
    extra: Dict[str, Any] = field(default_factory=dict)


# Global context
_log_context: LogContext = LogContext()


class JSONFormatter(logging.Formatter):
    """JSON formatter for structured logging."""

    def __init__(self, include_extra: bool = True):
        super().__init__()
        self.include_extra = include_extra

    def format(self, record: logging.LogRecord) -> str:
        """Format log record as JSON."""
        log_data = {
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "level": record.levelname,
            "logger": record.name,
            "message": record.getMessage(),
            "module": record.module,
            "function": record.funcName,
            "line": record.lineno,
        }

        # Add correlation ID from context
        corr_id = correlation_id_var.get()
        if corr_id:
            log_data["correlation_id"] = corr_id

        # Add context
        if _log_context.user_id:
            log_data["user_id"] = _log_context.user_id
        if _log_context.session_id:
            log_data["session_id"] = _log_context.session_id
        if _log_context.request_id:
            log_data["request_id"] = _log_context.request_id

        # Add extra fields
        if self.include_extra and hasattr(record, "extra"):
            log_data.update(record.extra)

        # Add exception info
        if record.exc_info:
            log_data["exception"] = {
                "type": record.exc_info[0].__name__ if record.exc_info[0] else None,
                "message": str(record.exc_info[1]) if record.exc_info[1] else None,
                "traceback": traceback.format_exception(*record.exc_info) if record.exc_info else [],
            }

        return json.dumps(log_data)


def get_logger(name: str, level: Optional[str] = None) -> logging.Logger:
    """Get a configured logger with JSON formatting."""
    logger = logging.getLogger(name)

    if not logger.handlers:
        handler = logging.StreamHandler(sys.stdout)
        handler.setFormatter(JSONFormatter())
        logger.addHandler(handler)

    log_level = level or os.environ.get("LOG_LEVEL", "INFO")
    logger.setLevel(getattr(logging, log_level.upper()))

    return logger


def set_correlation_id(corr_id: str):
    """Set correlation ID for current context."""
    correlation_id_var.set(corr_id)


def get_correlation_id() -> Optional[str]:
    """Get correlation ID from current context."""
    return correlation_id_var.get()


def clear_correlation_id():
    """Clear correlation ID from current context."""
    correlation_id_var.set(None)


def set_log_context(**kwargs):
    """Set additional context for logging."""
    for key, value in kwargs.items():
        setattr(_log_context, key, value)


def clear_log_context():
    """Clear all log context."""
    global _log_context
    _log_context = LogContext()


# Default logger
logger = get_logger("helios")
