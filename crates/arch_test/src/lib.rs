//! # arch_test - Architectural Tests for heliosCLI
//!
//! This crate enforces architectural boundaries and coding standards.
//!
//! ## xDD Compliance
//!
//! - **TDD**: Tests first, implementation follows
//! - **ATDD**: Acceptance criteria enforced via #[test]
//! - **BDD**: Scenario-based tests with clear Given/When/Then
//!
//! ## Architecture Rules
//!
//! 1. Domain crate has NO dependencies on infrastructure
//! 2. Application crate has NO dependencies on external services
//! 3. Adapters depend ONLY on ports (traits)
//! 4. No cyclic dependencies between crates

pub mod boundary_tests;
pub mod dependency_tests;
pub mod naming_tests;
pub mod property_tests;

use std::path::Path;

/// Configuration for architectural tests
pub struct ArchConfig {
    /// Root directory of the workspace
    pub workspace_root: PathBuf,
    /// List of domain crates (no external deps)
    pub domain_crates: Vec<String>,
    /// List of infrastructure crates
    pub infra_crates: Vec<String>,
    /// List of adapter crates
    pub adapter_crates: Vec<String>,
}

impl Default for ArchConfig {
    fn default() -> Self {
        Self {
            workspace_root: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .to_path_buf(),
            domain_crates: vec![
                "harness_spec".to_string(),
                "harness_schema".to_string(),
            ],
            infra_crates: vec![
                "harness_cache".to_string(),
                "harness_runner".to_string(),
            ],
            adapter_crates: vec![
                "harness_elicitation".to_string(),
                "harness_scaling".to_string(),
            ],
        }
    }
}

/// Result type for architectural assertions
pub type ArchResult = Result<(), ArchError>;

/// Errors from architectural violations
#[derive(Debug, Clone)]
pub enum ArchError {
    /// Dependency violation detected
    DependencyViolation {
        crate_name: String,
        forbidden_dep: String,
        message: String,
    },
    /// Naming convention violation
    NamingViolation {
        file: String,
        expected: String,
        found: String,
    },
    /// Boundary violation
    BoundaryViolation {
        from: String,
        to: String,
        message: String,
    },
    /// Cyclic dependency detected
    CyclicDependency {
        crate_name: String,
        cycle: Vec<String>,
    },
}

impl std::fmt::Display for ArchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchError::DependencyViolation { crate_name, forbidden_dep, message } => {
                write!(f, "DependencyViolation: {} depends on {} - {}", crate_name, forbidden_dep, message)
            }
            ArchError::NamingViolation { file, expected, found } => {
                write!(f, "NamingViolation: {} expected {} but found {}", file, expected, found)
            }
            ArchError::BoundaryViolation { from, to, message } => {
                write!(f, "BoundaryViolation: {} -> {} - {}", from, to, message)
            }
            ArchError::CyclicDependency { crate_name, cycle } => {
                write!(f, "CyclicDependency: {} has cycle: {}", crate_name, cycle.join(" -> "))
            }
        }
    }
}

impl std::error::Error for ArchError {}
