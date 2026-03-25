"""Retry logic with exponential backoff.

Provides decorators and utilities for retrying failed operations.
"""

import time
import random
import logging
from functools import wraps
from typing import Callable, Type, Tuple, Optional, Any
from dataclasses import dataclass


logger = logging.getLogger(__name__)


@dataclass
class RetryConfig:
    """Retry configuration."""
    max_attempts: int = 3
    initial_delay: float = 0.1
    max_delay: float = 60.0
    exponential_base: float = 2.0
    jitter: bool = True
    retry_on: Tuple[Type[Exception], ...] = (Exception,)


def calculate_delay(attempt: int, config: RetryConfig) -> float:
    """Calculate delay with exponential backoff and jitter."""
    delay = min(config.initial_delay * (config.exponential_base ** attempt, config.max_delay)
    
    if config.jitter:
        # Add random jitter between 0-25%
        delay = delay * (1 + random.uniform(0, 0.25))
    
    return delay


def retry(config: Optional[RetryConfig] = None, log_errors: bool = True):
    """Decorator to retry a function on failure.
    
    Usage:
        @retry(RetryConfig(max_attempts=3, initial_delay=0.5))
        def my_function():
            ...
    """
    if config is None:
        config = RetryConfig()
    
    def decorator(func: Callable) -> Callable:
        @wraps(func)
        def wrapper(*args, **kwargs) -> Any:
            last_exception = None
            
            for attempt in range(config.max_attempts):
                try:
                    return func(*args, **kwargs)
                except config.retry_on as e:
                    last_exception = e
                    
                    if attempt < config.max_attempts - 1:
                        delay = calculate_delay(attempt, config)
                        
                        if log_errors:
                            logger.warning(
                                f"Attempt {attempt + 1}/{config.max_attempts} failed for {func.__name__}: {e}. "
                                f"Retrying in {delay:.2f}s..."
                            )
                        
                        time.sleep(delay)
                    else:
                        if log_errors:
                            logger.error(
                                f"All {config.max_attempts} attempts failed for {func.__name__}: {e}"
                            )
            
            raise last_exception
        
        return wrapper
    return decorator


class RetryContext:
    """Context manager for manual retry handling."""
    
    def __init__(self, config: Optional[RetryConfig] = None):
        self.config = config or RetryConfig()
        self.attempt = 0
        self.last_exception: Optional[Exception] = None
    
    def __enter__(self):
        self.attempt += 1
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        if exc_type is None:
            return True  # Success, no retry needed
        
        self.last_exception = exc_val
        
        if self.attempt < self.config.max_attempts:
            delay = calculate_delay(self.attempt - 1, self.config)
            logger.warning(
                f"Attempt {self.attempt}/{self.config.max_attempts} failed: {exc_val}. "
                f"Retrying in {delay:.2f}s..."
            )
            time.sleep(delay)
            return False  # Continue retrying
        
        return False  # Don't suppress exception


# Common retry configurations
retry_quick = RetryConfig(max_attempts=3, initial_delay=0.1, max_delay=1.0)
retry_standard = RetryConfig(max_attempts=5, initial_delay=0.5, max_delay=30.0)
retry_persistent = RetryConfig(max_attempts=10, initial_delay=1.0, max_delay=60.0)


def retry_on_exception(*exceptions: Type[Exception], config: Optional[RetryConfig] = None):
    """Retry specifically on certain exceptions."""
    if config is None:
        config = RetryConfig(retry_on=exceptions)
    else:
        config.retry_on = exceptions
    return retry(config)
