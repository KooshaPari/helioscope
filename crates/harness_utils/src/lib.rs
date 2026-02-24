//! Common utilities for heliosHarness

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
    }

    #[test]
    fn test_parse_kv() {
        let result = parse_kv("a=1,b=2", ',', '=');
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_palindrome() {
        assert!(is_palindrome("radar"));
        assert!(!is_palindrome("hello"));
    }
}
