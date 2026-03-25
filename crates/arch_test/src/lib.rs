//! # arch_test - Architectural Tests for heliosCLI
//!
//! This crate provides architectural testing infrastructure including:
//! - Boundary enforcement tests
//! - TDD patterns (Red-Green-Refactor)
//! - Property-based testing with proptest

pub mod boundary;
pub mod tdd;
pub mod proptest_patterns;

pub use boundary::BoundaryEnforcer;
pub use tdd::TestDriven;
pub use proptest_patterns::PropertyTest;
