//! Verification pipeline

use crate::error::Result;
use crate::result::{GateDetail, GateResult, VerificationResult};
use crate::runners::run_cargo_test;
use harness_spec::models::{Specification, VerificationRule};

/// Verification pipeline
pub struct VerificationPipeline {
    _runners: PipelineRunners,
}

impl Default for VerificationPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl VerificationPipeline {
    /// Create new pipeline
    pub fn new() -> Self {
        Self {
            _runners: PipelineRunners::default(),
        }
    }

    /// Run verification for a spec
    pub async fn verify(&self, spec: &Specification) -> Result<Vec<VerificationResult>> {
        let mut results = Vec::new();

        for rule in &spec.spec.verification {
            let result = self.run_verification(rule, &spec.spec.name).await?;
            results.push(result);
        }

        Ok(results)
    }

    /// Run single verification
    async fn run_verification(
        &self,
        rule: &VerificationRule,
        spec_id: &str,
    ) -> Result<VerificationResult> {
        match rule {
            VerificationRule::Test {
                name: _,
                timeout_seconds,
            } => {
                // Default to cargo test
                let timeout = *timeout_seconds as u64;
                if timeout > 0 {
                    run_cargo_test(spec_id, timeout).await
                } else {
                    run_cargo_test(spec_id, 300).await // 5 min default
                }
            }
            VerificationRule::Security {
                scanner,
                critical_only: _,
            } => {
                // Placeholder for security scanning
                Ok(VerificationResult {
                    id: uuid::Uuid::new_v4(),
                    spec_id: spec_id.to_string(),
                    verification_type: crate::result::VerificationType::Security,
                    status: crate::result::VerificationStatus::Skipped,
                    started_at: chrono::Utc::now(),
                    completed_at: Some(chrono::Utc::now()),
                    duration_ms: 0,
                    output: format!("Security scanner '{}' not implemented yet", scanner),
                    errors: vec![],
                    metrics: Default::default(),
                })
            }
            VerificationRule::Performance { metric, threshold } => {
                // Placeholder for performance testing
                Ok(VerificationResult {
                    id: uuid::Uuid::new_v4(),
                    spec_id: spec_id.to_string(),
                    verification_type: crate::result::VerificationType::Performance,
                    status: crate::result::VerificationStatus::Skipped,
                    started_at: chrono::Utc::now(),
                    completed_at: Some(chrono::Utc::now()),
                    duration_ms: 0,
                    output: format!(
                        "Performance benchmark '{}' with threshold '{}' not implemented yet",
                        metric, threshold
                    ),
                    errors: vec![],
                    metrics: Default::default(),
                })
            }
            VerificationRule::Custom {
                command,
                expected_exit_code,
            } => {
                // Run custom command
                let output = tokio::process::Command::new("sh")
                    .args(["-c", command])
                    .output()
                    .await?;

                let passed = output.status.code() == Some(*expected_exit_code);

                Ok(VerificationResult {
                    id: uuid::Uuid::new_v4(),
                    spec_id: spec_id.to_string(),
                    verification_type: crate::result::VerificationType::Custom,
                    status: if passed {
                        crate::result::VerificationStatus::Passed
                    } else {
                        crate::result::VerificationStatus::Failed
                    },
                    started_at: chrono::Utc::now(),
                    completed_at: Some(chrono::Utc::now()),
                    duration_ms: 0,
                    output: String::from_utf8_lossy(&output.stdout).to_string(),
                    errors: vec![String::from_utf8_lossy(&output.stderr).to_string()],
                    metrics: Default::default(),
                })
            }
        }
    }

    /// Run verification gates
    pub fn run_gates(
        &self,
        results: &[VerificationResult],
        gates: &[GateConfig],
    ) -> Vec<GateResult> {
        let mut gate_results = Vec::new();

        for gate in gates {
            let passed = self.evaluate_gate(gate, results);

            let details: Vec<GateDetail> = results
                .iter()
                .map(|r| {
                    let check_passed =
                        matches!(r.status, crate::result::VerificationStatus::Passed);
                    GateDetail {
                        check: format!("{:?}", r.verification_type),
                        passed: check_passed,
                        message: r.output.clone(),
                    }
                })
                .collect();

            gate_results.push(GateResult {
                name: gate.name.clone(),
                passed,
                message: if passed {
                    "All gates passed".to_string()
                } else {
                    "Gate check failed".to_string()
                },
                details,
            });
        }

        gate_results
    }

    fn evaluate_gate(&self, gate: &GateConfig, results: &[VerificationResult]) -> bool {
        match gate.criteria.as_str() {
            "all_passed" => results
                .iter()
                .all(|r| matches!(r.status, crate::result::VerificationStatus::Passed)),
            "any_passed" => results
                .iter()
                .any(|r| matches!(r.status, crate::result::VerificationStatus::Passed)),
            "no_failures" => !results
                .iter()
                .any(|r| matches!(r.status, crate::result::VerificationStatus::Failed)),
            _ => false,
        }
    }
}

/// Pipeline runners (extensible)
#[derive(Default)]
pub struct PipelineRunners {
    // Can add custom runners here
}

/// Gate configuration
#[derive(Debug, Clone)]
pub struct GateConfig {
    pub name: String,
    pub criteria: String,
    pub threshold: Option<f64>,
}
