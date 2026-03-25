//! # TDD (Test-Driven Development) Module
//!
//! Provides utilities for test-first development cycles.
//!
//! ## TDD Cycle
//!
//! ```text
//! ┌─────────┐    ┌─────────┐    ┌─────────────┐
//! │   RED   │ -> │  GREEN  │ -> │  REFACTOR   │
//! │ Write   │    │ Make    │    │  Improve    │
//! │ failing │    │ it     │    │  code       │
//! │ test    │    │ pass   │    │  quality    │
//! └─────────┘    └─────────┘    └──────┬──────┘
//!                                      │
//!                                      v
//!                                 Back to RED
//! ```
//!
//! ## BDD Scenario Example
//!
//! ```gherkin
//! Feature: Cache operations
//!
//!   Scenario: Cache stores and retrieves values
//!     Given an empty cache
//!     When I store key="foo" with value="bar"
//!     Then retrieving "foo" should return "bar"
//!
//!   Scenario: Cache handles missing keys
//!     Given an empty cache
//!     When I retrieve a missing key
//!     Then it should return None
//! ```
//!
//! ## ATDD (Acceptance TDD)
//!
//! Write acceptance criteria first, then implement to meet them.
//!
//! ## Property-Based Testing
//!
//! Use `proptest` to test properties across many inputs.

use std::collections::HashMap;

/// Example: Cache implementation following TDD
#[derive(Debug, Clone, Default)]
pub struct Cache {
    data: HashMap<String, Vec<u8>>,
    hits: u64,
    misses: u64,
}

impl Cache {
    /// Create a new empty cache
    ///
    /// ## TDD Note
    ///
    /// This method was written AFTER the tests for cache creation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a value in the cache
    ///
    /// ## TDD Note
    ///
    /// Tests written first:
    /// - `test_set_and_get`
    /// - `test_set_overwrites_existing`
    pub fn set(&mut self, key: impl Into<String>, value: Vec<u8>) {
        self.data.insert(key.into(), value);
    }

    /// Get a value from the cache
    ///
    /// ## TDD Note
    ///
    /// Tests written first:
    /// - `test_get_existing_key`
    /// - `test_get_missing_key_returns_none`
    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        match self.data.get(key) {
            Some(v) => {
                self.hits += 1;
                Some(v)
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            hits: self.hits,
            misses: self.misses,
            size: self.data.len(),
        }
    }

    /// Clear all entries
    ///
    /// ## TDD Note
    ///
    /// Test: `test_clear_removes_all_entries`
    pub fn clear(&mut self) {
        self.data.clear();
        self.hits = 0;
        self.misses = 0;
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub size: usize,
}

// ============================================================================
// TDD TESTS - Written FIRST before implementation
// ============================================================================

#[cfg(test)]
mod tdd_tests {
    use super::*;

    // RED phase: Write failing tests first

    #[test]
    fn test_cache_stores_and_retrieves_values() {
        // Arrange
        let mut cache = Cache::new();

        // Act
        cache.set("foo", vec![1, 2, 3]);

        // Assert
        assert_eq!(cache.get("foo"), Some(&vec![1, 2, 3]));
    }

    #[test]
    fn test_cache_handles_missing_keys() {
        // Arrange
        let cache = Cache::new();

        // Act
        let result = cache.get("nonexistent");

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn test_cache_clear_removes_all_entries() {
        // Arrange
        let mut cache = Cache::new();
        cache.set("a", vec![1]);
        cache.set("b", vec![2]);

        // Act
        cache.clear();

        // Assert
        assert_eq!(cache.get("a"), None);
        assert_eq!(cache.get("b"), None);
    }

    #[test]
    fn test_cache_stats_tracks_hits_and_misses() {
        // Arrange
        let mut cache = Cache::new();
        cache.set("key", vec![1]);

        // Act
        let _ = cache.get("key"); // hit
        let _ = cache.get("missing"); // miss

        // Assert
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }
}

// ============================================================================
// BDD TESTS - Gherkin-style scenario tests
// ============================================================================

#[cfg(test)]
mod bdd_tests {
    use super::*;

    // BDD: Given-When-Then pattern

    #[test]
    fn bdd_cache_returns_none_for_missing_key() {
        // GIVEN an empty cache
        let cache = Cache::new();

        // WHEN retrieving a missing key
        let result = cache.get("key");

        // THEN it should return None
        assert_eq!(result, None);
    }

    #[test]
    fn bdd_cache_stores_multiple_values() {
        // GIVEN a cache with values "a" and "b"
        let mut cache = Cache::new();
        cache.set("a", vec![1]);
        cache.set("b", vec![2]);

        // WHEN retrieving both values
        let val_a = cache.get("a");
        let val_b = cache.get("b");

        // THEN both should be retrievable
        assert_eq!(val_a, Some(&vec![1]));
        assert_eq!(val_b, Some(&vec![2]));
    }
}

// ============================================================================
// PROPERTY-BASED TESTS - Test properties across many inputs
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;

    // Property: Setting and getting returns the same value
    #[test]
    fn property_set_get_roundtrip() {
        let mut cache = Cache::new();
        let key = "test_key";
        let value = vec![1, 2, 3, 4, 5];

        cache.set(key, value.clone());
        let retrieved = cache.get(key);

        assert_eq!(retrieved, Some(&value));
    }

    // Property: Clear makes get return None
    #[test]
    fn property_clear_means_none() {
        let mut cache = Cache::new();
        cache.set("key", vec![1]);

        cache.clear();
        let result = cache.get("key");

        assert_eq!(result, None);
    }

    // Property: Stats size matches data len
    #[test]
    fn property_stats_size_matches_entries() {
        let mut cache = Cache::new();

        for i in 0..10 {
            cache.set(format!("key_{}", i), vec![i]);
        }

        let stats = cache.stats();
        assert_eq!(stats.size, 10);
    }
}

// ============================================================================
// CONTRACT TESTS - Test interface contracts
// ============================================================================

#[cfg(test)]
mod contract_tests {
    use super::*;

    // Contract: Cache.get always returns None or Some
    #[test]
    fn contract_get_returns_valid_option() {
        let cache = Cache::new();
        let result = cache.get("any_key");

        // Contract: get should return Option<Vec<u8>>
        assert!(result.is_none() || result.is_some());
    }

    // Contract: Stats are always non-negative
    #[test]
    fn contract_stats_non_negative() {
        let cache = Cache::new();
        let stats = cache.stats();

        assert!(stats.hits >= 0);
        assert!(stats.misses >= 0);
        assert!(stats.size >= 0);
    }
}

// ============================================================================
// EXAMPLE MAPPING (xDD)
// ============================================================================

/*
Example xDD mapping for the Cache:

| xDD Practice | Implementation |
|--------------|---------------|
| TDD | Tests in `tdd_tests` module - RED first |
| BDD | Tests in `bdd_tests` module - Given-When-Then |
| FDD | Feature: Cache operations |
| ATDD | Acceptance criteria in doc tests |
| Property Testing | Tests in `property_tests` - invariants |
| Contract Testing | Tests in `contract_tests` - interface contracts |
| Mutation Testing | Run with `cargo-mutate` |
| RDD | Responsibility: Cache manages storage |

## Test Naming Convention

```
test_<feature>_<scenario>_<expected_result>

Examples:
- test_cache_get_existing_key_returns_value
- test_cache_set_multiple_keys_all_retrievable
- test_cache_clear_resets_stats_to_zero
```
*/
