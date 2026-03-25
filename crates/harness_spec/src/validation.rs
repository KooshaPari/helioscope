//! Specification validation

use crate::error::{Result, SpecError};
use crate::models::*;

/// Validate a specification
pub fn validate(spec: &Specification) -> Result<()> {
    // Validate name
    if spec.spec.name.is_empty() {
        return Err(SpecError::MissingField("name".to_string()));
    }

    // Validate version format
    validate_version(&spec.spec.version)?;

    // Validate verification rules
    for rule in &spec.spec.verification {
        validate_verification_rule(rule)?;
    }

    // Validate rollback config
    validate_rollback_config(&spec.spec.rollback)?;

    // Validate success criteria
    for criterion in &spec.spec.success_criteria {
        validate_success_criterion(criterion)?;
    }

    Ok(())
}

/// Validate version string (semver)
fn validate_version(version: &str) -> Result<()> {
    // Basic semver validation
    let parts: Vec<&str> = version.split('.').collect();

    if parts.len() < 2 {
        return Err(SpecError::InvalidValue {
            field: "version".to_string(),
            message: "Version must be in semver format (x.y.z)".to_string(),
        });
    }

    // Check major version is numeric
    parts[0]
        .parse::<u32>()
        .map_err(|_| SpecError::InvalidValue {
            field: "version".to_string(),
            message: "Major version must be numeric".to_string(),
        })?;

    Ok(())
}

/// Validate verification rule
fn validate_verification_rule(rule: &VerificationRule) -> Result<()> {
    match rule {
        VerificationRule::Test {
            name,
            timeout_seconds,
        } => {
            if name.is_empty() {
                return Err(SpecError::InvalidValue {
                    field: "verification.test.name".to_string(),
                    message: "Test name cannot be empty".to_string(),
                });
            }
            if *timeout_seconds == 0 {
                return Err(SpecError::InvalidValue {
                    field: "verification.test.timeout".to_string(),
                    message: "Timeout must be greater than 0".to_string(),
                });
            }
        }
        VerificationRule::Security { scanner, .. } => {
            if scanner.is_empty() {
                return Err(SpecError::InvalidValue {
                    field: "verification.security.scanner".to_string(),
                    message: "Scanner name cannot be empty".to_string(),
                });
            }
        }
        VerificationRule::Performance { metric, threshold } => {
            if metric.is_empty() {
                return Err(SpecError::InvalidValue {
                    field: "verification.performance.metric".to_string(),
                    message: "Metric name cannot be empty".to_string(),
                });
            }
            if threshold.is_empty() {
                return Err(SpecError::InvalidValue {
                    field: "verification.performance.threshold".to_string(),
                    message: "Threshold cannot be empty".to_string(),
                });
            }
        }
        VerificationRule::Custom {
            command,
            expected_exit_code: _expected_exit_code,
        } => {
            if command.is_empty() {
                return Err(SpecError::InvalidValue {
                    field: "verification.custom.command".to_string(),
                    message: "Command cannot be empty".to_string(),
                });
            }
            // expected_exit_code can be any value, no validation needed
        }
    }

    Ok(())
}

/// Validate rollback configuration
fn validate_rollback_config(config: &RollbackConfig) -> Result<()> {
    match config.strategy {
        RollbackStrategy::GitRevert | RollbackStrategy::Snapshot | RollbackStrategy::Hybrid => {
            // All strategies are valid
        }
    }

    if config.timeout_seconds == 0 {
        return Err(SpecError::InvalidValue {
            field: "rollback.timeout".to_string(),
            message: "Timeout must be greater than 0".to_string(),
        });
    }

    Ok(())
}

/// Validate success criterion
fn validate_success_criterion(criterion: &SuccessCriterion) -> Result<()> {
    if criterion.metric.is_empty() {
        return Err(SpecError::InvalidValue {
            field: "success_criteria.metric".to_string(),
            message: "Metric name cannot be empty".to_string(),
        });
    }

    // At least one of threshold, minimum, or maximum should be set
    let has_threshold = criterion.threshold.is_some();
    let has_minimum = criterion.minimum.is_some();
    let has_maximum = criterion.maximum.is_some();

    if !has_threshold && !has_minimum && !has_maximum {
        return Err(SpecError::InvalidValue {
            field: "success_criteria".to_string(),
            message: "At least one of threshold, minimum, or maximum must be set".to_string(),
        });
    }

    Ok(())
}

/// Validate and parse with options
pub fn validate_with_options(spec: &Specification, options: &ParseOptions) -> Result<()> {
    // First do basic validation
    validate(spec)?;

    // If strict mode, do additional checks
    if options.strict {
        // Check owner is set in strict mode
        if spec.spec.owner.is_empty() {
            return Err(SpecError::ValidationError(
                "Owner is required in strict mode".to_string(),
            ));
        }

        // Check at least one verification rule in strict mode
        if spec.spec.verification.is_empty() {
            return Err(SpecError::ValidationError(
                "At least one verification rule required in strict mode".to_string(),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_spec() {
        let spec = Specification {
            spec: SpecContent {
                name: "test".to_string(),
                version: "1.0.0".to_string(),
                owner: "team".to_string(),
                verification: vec![VerificationRule::Test {
                    name: "test".to_string(),
                    timeout_seconds: 60,
                }],
                rollback: RollbackConfig::default(),
                success_criteria: vec![SuccessCriterion {
                    metric: "success_rate".to_string(),
                    threshold: Some(">95%".to_string()),
                    minimum: None,
                    maximum: None,
                }],
                behavior: None,
                resources: None,
                metadata: std::collections::HashMap::new(),
            },
        };

        assert!(validate(&spec).is_ok());
    }

    #[test]
    fn test_validate_missing_name() {
        let spec = Specification {
            spec: SpecContent {
                name: "".to_string(),
                version: "1.0.0".to_string(),
                owner: "".to_string(),
                verification: vec![],
                rollback: RollbackConfig::default(),
                success_criteria: vec![],
                behavior: None,
                resources: None,
                metadata: std::collections::HashMap::new(),
            },
        };

        assert!(validate(&spec).is_err());
    }

    #[test]
    fn test_validate_invalid_version() {
        let spec = Specification {
            spec: SpecContent {
                name: "test".to_string(),
                version: "invalid".to_string(),
                owner: "".to_string(),
                verification: vec![],
                rollback: RollbackConfig::default(),
                success_criteria: vec![],
                behavior: None,
                resources: None,
                metadata: std::collections::HashMap::new(),
            },
        };

        assert!(validate(&spec).is_err());
    }
}
