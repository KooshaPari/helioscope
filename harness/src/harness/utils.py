def is_palindrome(s: str) -> bool:
    """Check if a string is a palindrome.

    Args:
        s: The string to check.

    Returns:
        True if the string reads the same forwards and backwards, False otherwise.
    """
    cleaned = s.lower().replace(" ", "")
    return cleaned == cleaned[::-1]
