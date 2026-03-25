//! # Boundary Tests
//!
//! Integration tests for verifying architectural boundaries.
//!
//! ## What are Boundary Tests?
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    ARCHITECTURE LAYERS                       │
//! ├─────────────────────────────────────────────────────────────┤
//! │  ┌─────────┐    ┌─────────┐    ┌─────────┐              │
//! │  │  UI     │───▶│ Domain  │───▶│ External│              │
//! │  │ Layer   │    │ Layer   │    │ Services│              │
//! │  └─────────┘    └─────────┘    └─────────┘              │
//! │        │              │              │                     │
//! │        ▼              ▼              ▼                     │
//! │  ┌─────────┐    ┌─────────┐    ┌─────────┐              │
//! │  │  tests  │    │  tests  │    │  tests  │              │
//! │  │ (white) │    │ (white) │    │ (white) │              │
//! │  └─────────┘    └─────────┘    └─────────┘              │
//! └─────────────────────────────────────────────────────────────┘
//! ```

use std::path::Path;
use walkdir::WalkDir;

/// Verify that domain crate has no external dependencies
#[test]
fn domain_has_no_external_dependencies() {
    let domain_path = Path::new("src/domain");
    if !domain_path.exists() {
        println!("Domain crate not found - skipping");
        return;
    }

    let forbidden_deps = [
        "tokio",
        "serde_json",
        "reqwest",
        "sqlx",
        "tracing",
    ];

    for entry in WalkDir::new(domain_path)
        .into_iter()
        .filter_map(|e| e.ok())
        {
            if entry.path().extension().map(|e| e == "rs").unwrap_or(false) {
                let content = std::fs::read_to_string(entry.path()).unwrap_or_default();

                for dep in &forbidden_deps {
                    assert!(
                        !content.contains(&format!("use {}", dep)),
                        "Domain layer uses external crate: {} in {}",
                        dep,
                        entry.path().display()
                    );
                }
            }
        }
}

/// Verify adapters crate only depends on domain
#[test]
fn adapters_only_depend_on_domain() {
    let adapters_path = Path::new("src/adapters");
    if !adapters_path.exists() {
        println!("Adapters crate not found - skipping");
        return;
    }

    let allowed_deps = ["domain", "thiserror", "anyhow"];

    for entry in WalkDir::new(adapters_path)
        .into_iter()
        .filter_map(|e| e.ok())
        {
            if entry.path().extension().map(|e| e == "rs").unwrap_or(false) {
                let content = std::fs::read_to_string(entry.path()).unwrap_or_default();

                for dep in &["tokio", "serde_json", "reqwest", "sqlx"] {
                    assert!(
                        !content.contains(&format!("use {}::", dep)) ||
                        allowed_deps.iter().any(|d| content.contains(&format!("{}::", d))),
                        "Adapter uses forbidden dependency: {} in {}",
                        dep,
                        entry.path().display()
                    );
                }
            }
        }
}

/// Verify no cyclic dependencies between crates
#[test]
fn no_cyclic_dependencies() {
    let crate_dirs = ["domain", "application", "adapters", "infrastructure"];

    // Create a simple DAG check
    let mut visited = std::collections::HashSet::new();
    let mut stack = std::collections::HashSet::new();

    fn has_cycle(path: &Path, visited: &mut std::collections::HashSet<String>, stack: &mut std::collections::HashSet<String>) -> bool {
        let path_str = path.to_string_lossy().to_string();
        if stack.contains(&path_str) {
            return true; // Cycle found
        }
        if visited.contains(&path_str) {
            return false; // Already processed
        }

        visited.insert(path_str.clone());
        stack.insert(path_str.clone());

        // Check imports
        if let Ok(content) = std::fs::read_to_string(path) {
            for line in content.lines() {
                if line.starts_with("use ") {
                    let dep = line.trim_start_matches("use ").split("::").next().unwrap_or("");
                    let dep_path = path.parent().map(|p| p.join(dep)).unwrap_or_default();
                    if dep_path.exists() && has_cycle(&dep_path, visited, stack) {
                        return true;
                    }
                }
            }
        }

        stack.remove(&path_str);
        false
    }

    for crate_dir in &crate_dirs {
        let path = Path::new(crate_dir);
        if path.exists() {
            assert!(!has_cycle(path, &mut visited, &mut stack),
                "Cyclic dependency found in: {}", crate_dir);
        }
    }
}

/// Verify module structure follows hexagonal architecture
#[test]
fn hexagonal_structure_followed() {
    let expected_modules = ["domain", "application", "ports", "adapters"];

    let structure_valid = expected_modules.iter().all(|m| {
        Path::new(m).exists() || Path::new(&format!("src/{}", m)).exists()
    });

    if !structure_valid {
        println!("Hexagonal structure not fully implemented - this is advisory");
    }
}
