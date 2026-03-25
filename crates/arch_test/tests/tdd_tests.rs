//! # TDD Tests
//!
//! Test-Driven Development pattern verification.
//!
//! ## TDD Cycle
//!
//! ```text
//! ┌────────────────────────────────────────┐
//! │           TDD CYCLE                      │
//! ├────────────────────────────────────────┤
//! │                                        │
//! │   ┌─────┐   Red    ┌─────────┐          │
//! │   │Write│────────▶│ Failing │          │
//! │   │Test │         │  Test   │          │
//! │   └─────┘         └────┬────┘          │
//! │                         │                │
//! │                         ▼                │
//! │   ┌─────┐   Green   ┌─────────┐        │
//! │   │Write│◀─────────│Implement│        │
//! │   │Code │           │  Code   │        │
//! │   └─────┘           └────┬────┘        │
//! │                         │                │
//! │                         ▼                │
//! │   ┌─────┐  Refactor  ┌─────────┐        │
//! │   │Clean│◀──────────│  Tests  │        │
//! │   │ Code│           │  Pass   │        │
//! │   └─────┘           └─────────┘        │
//! │                                        │
//! └────────────────────────────────────────┘
//! ```

use std::collections::HashMap;

/// Example: TDD - HashMap operations
/// These tests follow the TDD pattern: write test first, then implementation

/// Test: Insert and retrieve value
#[test]
fn test_insert_and_get() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");

    assert_eq!(map.get("key1"), Some(&"value1"));
}

/// Test: Update existing value
#[test]
fn test_update_existing_value() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key1", "value2");

    assert_eq!(map.get("key1"), Some(&"value2"));
    assert_eq!(map.len(), 1);
}

/// Test: Remove value
#[test]
fn test_remove_value() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.remove("key1");

    assert_eq!(map.get("key1"), None);
    assert!(map.is_empty());
}

/// Test: Contains key
#[test]
fn test_contains_key() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");

    assert!(map.contains_key("key1"));
    assert!(!map.contains_key("key2"));
}

/// TDD Principle: Test behavior, not implementation
/// These tests verify the contract/interface, not internal details

/// Example: Stack behavior tests
#[test]
fn test_stack_push_pop() {
    let mut stack: Vec<i32> = Vec::new();

    // Push
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Verify order
    assert_eq!(stack.len(), 3);

    // Pop - LIFO order
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None); // Empty stack
}

/// Test: Stack underflow is handled
#[test]
fn test_stack_underflow_returns_none() {
    let mut stack: Vec<i32> = Vec::new();

    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
}

/// TDD Pattern: Given-When-Then
/// Test structure follows BDD scenarios

/// Scenario: User registration with valid data
#[test]
fn test_user_registration_valid() {
    // Given
    let email = "user@example.com";
    let password = "securepassword123";

    // When
    let result = validate_registration(email, password);

    // Then
    assert!(result.is_ok());
}

/// Scenario: User registration with invalid email
#[test]
fn test_user_registration_invalid_email() {
    // Given
    let email = "invalid-email";
    let password = "securepassword123";

    // When
    let result = validate_registration(email, password);

    // Then
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("email"));
}

/// Scenario: User registration with weak password
#[test]
fn test_user_registration_weak_password() {
    // Given
    let email = "user@example.com";
    let password = "123"; // Too short

    // When
    let result = validate_registration(email, password);

    // Then
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("password"));
}

// Validation function - TDD: write test first, then implementation
fn validate_registration(email: &str, password: &str) -> Result<(), String> {
    // Email validation
    if !email.contains('@') || !email.contains('.') {
        return Err("Invalid email format".to_string());
    }

    // Password validation
    if password.len() < 8 {
        return Err("Password too short".to_string());
    }

    Ok(())
}
