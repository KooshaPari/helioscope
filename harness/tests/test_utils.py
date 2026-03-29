"""Tests for utils module."""

from harness.utils import is_palindrome


def test_is_palindrome_true():
    """Test palindrome detection - true case."""
    assert is_palindrome("radar") is True
    assert is_palindrome("level") is True


def test_is_palindrome_false():
    """Test palindrome detection - false case."""
    assert is_palindrome("hello") is False


def test_is_palindrome_empty():
    """Test empty string."""
    assert is_palindrome("") is True
