//! Property-based testing patterns with proptest

/// Property test configuration
#[derive(Debug, Clone)]
pub struct PropertyTest {
    pub name: String,
    pub iterations: u32,
}

impl PropertyTest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            iterations: 256,
        }
    }
}

/// Common strategies for property-based testing
pub mod strategies {
    use proptest::prelude::*;
    
    /// Strategy for valid UTF-8 strings
    pub fn valid_utf8() -> impl Strategy<Value = String> {
        ".*"
    }
    
    /// Strategy for non-empty strings
    pub fn non_empty_string() -> impl Strategy<Value = String> {
        "[^\\s].*"
    }
    
    /// Strategy for positive integers
    pub fn positive_int() -> impl Strategy<Value = u32> {
        any::<u32>().prop_filter("positive", |&n| n > 0)
    }
    
    /// Strategy for valid identifiers
    pub fn identifier() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9_]{0,63}"
    }
}

/// Invariant checking
pub mod invariants {
    /// Check that an invariant holds
    pub fn check<T>(value: &T, inv: impl Fn(&T) -> bool, name: &str) -> Result<(), String> {
        if inv(value) {
            Ok(())
        } else {
            Err(format!("Invariant violated: {}", name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_invariant() {
        assert!(invariants::check(&42, |v| *v > 0, "positive").is_ok());
    }
}
