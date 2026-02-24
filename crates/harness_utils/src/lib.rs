//! Common utilities for heliosHarness

use thiserror::Error;

/// Error types for utils
#[derive(Debug, Error)]
pub enum UtilsError {
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Overflow: {0}")]
    Overflow(String),
}

/// Fast string hashing (FNV-1a variant)
pub fn hash_str(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in s.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Parse key-value pairs from string
pub fn parse_kv(s: &str, delimiter: char, pair_sep: char) -> Vec<(String, String)> {
    s.split(delimiter)
        .filter_map(|pair| {
            let mut parts = pair.split(pair_sep);
            match (parts.next(), parts.next()) {
                (Some(k), Some(v)) => Some((k.trim().to_string(), v.trim().to_string())),
                _ => None,
            }
        })
        .collect()
}

/// Parse tags from string
pub fn parse_tags(s: &str) -> Vec<String> {
    s.split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

/// Check if string is palindrome
pub fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes.iter().zip(bytes.iter().rev()).all(|(a, b)| a == b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash_str("test"), hash_str("test"));
        assert_ne!(hash_str("test"), hash_str("other"));
    }

    #[test]
    fn test_parse_kv() {
        let result = parse_kv("a=1,b=2", ',', '=');
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("a".to_string(), "1".to_string()));
    }

    #[test]
    fn test_parse_kv_with_spaces() {
        let result = parse_kv(" a = 1 , b = 2 ", ',', '=');
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_parse_kv_invalid() {
        let result = parse_kv("invalid", ',', '=');
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_tags() {
        let tags = parse_tags("tag1,tag2,tag3");
        assert_eq!(tags.len(), 3);
    }

    #[test]
    fn test_parse_tags_with_spaces() {
        let tags = parse_tags(" tag1 , tag2 , tag3 ");
        assert_eq!(tags.len(), 3);
    }

    #[test]
    fn test_parse_tags_empty() {
        let tags = parse_tags("");
        assert!(tags.is_empty());
    }

    #[test]
    fn test_palindrome() {
        assert!(is_palindrome("radar"));
        assert!(is_palindrome("level"));
        assert!(is_palindrome(""));
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn test_palindrome_single_char() {
        assert!(is_palindrome("a"));
    }
}
