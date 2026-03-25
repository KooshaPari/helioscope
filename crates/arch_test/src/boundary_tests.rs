//! # Boundary Tests
//!
//! Tests that enforce the hexagonal architecture boundaries.
//!
//! ## Architecture Rules
//!
//! - **Ports**: Domain defines traits (no external deps)
//! - **Adapters**: Implement ports, depend on external services
//! - **Core**: Domain logic has NO dependencies on infrastructure
//!
//! ## BDD Scenarios
//!
//! ```gherkin
//! Scenario: Domain has no infrastructure dependencies
//!   Given a domain crate
//!   When checking its dependencies
//!   Then it should have no external dependencies
//!
//! Scenario: Adapters depend only on ports
//!   Given an adapter crate
//!   When checking its dependencies
//!   Then it should only depend on domain and ports
//! ```

use crate::{ArchConfig, ArchError, ArchResult};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Forbidden dependencies for domain crates
const FORBIDDEN_DOMAIN_DEPS: &[&str] = &[
    "tokio",
    "serde",
    "anyhow",
    "thiserror",
    "tracing",
    "sqlx",
    "reqwest",
];

/// Allowed dependencies for domain crates (stdlib only)
const ALLOWED_DOMAIN_DEPS: &[&str] = &[
    "std",
    "core",
    "alloc",
];

/// Checks if a crate has valid domain boundaries
///
/// ## TDD Cycle
///
/// - **Red**: Write test that fails if domain depends on infra
/// - **Green**: Make domain crate depend only on stdlib
/// - **Refactor**: Ensure all domain crates follow the rule
pub fn check_domain_boundaries(config: &ArchConfig) -> ArchResult {
    let mut errors = Vec::new();

    for crate_name in &config.domain_crates {
        if let Err(e) = check_crate_has_no_forbidden_deps(crate_name, FORBIDDEN_DOMAIN_DEPS, config) {
            errors.push(e);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.into_iter().next().unwrap())
    }
}

/// Checks if a crate has no forbidden dependencies
fn check_crate_has_no_forbidden_deps(
    crate_name: &str,
    forbidden: &[&str],
    config: &ArchConfig,
) -> ArchResult {
    let cargo_toml = config.workspace_root.join("crates").join(crate_name).join("Cargo.toml");

    if !cargo_toml.exists() {
        return Ok(()); // Crate doesn't exist, skip
    }

    let content = fs::read_to_string(&cargo_toml)
        .map_err(|e| crate::ArchError::DependencyViolation {
            crate_name: crate_name.to_string(),
            forbidden_dep: "file://".to_string(),
            message: format!("Cannot read Cargo.toml: {}", e),
        })?;

    for dep in forbidden {
        if content.contains(&format!("\"{}\"", dep)) {
            return Err(crate::ArchError::DependencyViolation {
                crate_name: crate_name.to_string(),
                forbidden_dep: dep.to_string(),
                message: format!(
                    "Domain crate {} should not depend on {} (violates hexagonal architecture)",
                    crate_name, dep
                ),
            });
        }
    }

    Ok(())
}

/// Checks if adapters depend only on ports and domain
pub fn check_adapter_boundaries(config: &ArchConfig) -> ArchResult {
    let mut errors = Vec::new();

    for crate_name in &config.adapter_crates {
        if let Err(e) = check_adapter_only_depends_on_domain(crate_name, config) {
            errors.push(e);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.into_iter().next().unwrap())
    }
}

/// Checks adapter dependencies are only on domain and ports
fn check_adapter_only_depends_on_domain(
    crate_name: &str,
    config: &ArchConfig,
) -> ArchResult {
    let cargo_toml = config.workspace_root.join("crates").join(crate_name).join("Cargo.toml");

    if !cargo_toml.exists() {
        return Ok(()); // Crate doesn't exist, skip
    }

    let content = fs::read_to_string(&cargo_toml)
        .map_err(|e| crate::ArchError::DependencyViolation {
            crate_name: crate_name.to_string(),
            forbidden_dep: "file://".to_string(),
            message: format!("Cannot read Cargo.toml: {}", e),
        })?;

    // Get all workspace crates
    let workspace_crates: HashSet<&str> = config
        .domain_crates
        .iter()
        .chain(config.infra_crates.iter())
        .chain(config.adapter_crates.iter())
        .map(|s| s.as_str())
        .collect();

    // Check each line for dependencies
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('"') && !trimmed.starts_with("//") {
            for dep in &["sqlx", "tokio-postgres", "redis", "mongodb"] {
                if trimmed.contains(&format!("\"{}\"", dep)) {
                    return Err(crate::ArchError::DependencyViolation {
                        crate_name: crate_name.to_string(),
                        forbidden_dep: dep.to_string(),
                        message: format!(
                            "Adapter {} should not depend on {} directly (use ports/traits instead)",
                            crate_name, dep
                        ),
                    });
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_crate_has_no_tokio_dependency() {
        let config = ArchConfig::default();
        let result = check_domain_boundaries(&config);
        assert!(result.is_ok(), "Domain crates should not depend on tokio");
    }

    #[test]
    fn test_adapter_crate_follows_port_pattern() {
        let config = ArchConfig::default();
        let result = check_adapter_boundaries(&config);
        assert!(result.is_ok(), "Adapters should depend only on domain/ports");
    }

    #[test]
    fn test_naming_conventions() {
        let config = ArchConfig::default();
        let crate_name = "harness_schema";

        let cargo_toml = config.workspace_root.join("crates").join(crate_name).join("Cargo.toml");
        assert!(cargo_toml.exists(), "Crate {} should exist", crate_name);
    }
}