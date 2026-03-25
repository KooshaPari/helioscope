//! Test runners

use crate::error::{Result, VerifyError};
use crate::result::{
    VerificationMetrics, VerificationResult, VerificationStatus, VerificationType,
};
use chrono::Utc;
use std::process::Command;
use std::time::Instant;
use uuid::Uuid;

/// Run tests using cargo test
pub async fn run_cargo_test(spec_id: &str, timeout_secs: u64) -> Result<VerificationResult> {
    let start = Instant::now();
    let id = Uuid::new_v4();

    let output = tokio::time::timeout(
        std::time::Duration::from_secs(timeout_secs),
        tokio::task::spawn_blocking(move || {
            Command::new("cargo")
                .args(["test", "--", "--nocapture"])
                .output()
        }),
    )
    .await
    .map_err(|_| VerifyError::Timeout("Test execution timed out".to_string()))?
    .map_err(|e| VerifyError::TestRunnerError(e.to_string()))??;

    let duration_ms = start.elapsed().as_millis() as u64;
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();

    let passed = output.status.success();
    let status = if passed {
        VerificationStatus::Passed
    } else {
        VerificationStatus::Failed
    };

    // Parse test results
    let mut test_count = 0u32;
    let mut passed_count = 0u32;
    let mut failed_count = 0u32;

    for line in output_str.lines() {
        if line.contains("test result:") {
            // Parse: "test result: ok. 10 passed; 0 failed"
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "passed;" {
                    if let Some(n) = parts.get(i - 1).and_then(|s| s.parse::<u32>().ok()) {
                        passed_count = n;
                        test_count += n;
                    }
                }
                if *part == "failed" {
                    if let Some(n) = parts.get(i - 1).and_then(|s| s.parse::<u32>().ok()) {
                        failed_count = n;
                        test_count += n;
                    }
                }
            }
        }
    }

    let errors = if !passed { vec![stderr_str] } else { vec![] };

    Ok(VerificationResult {
        id,
        spec_id: spec_id.to_string(),
        verification_type: VerificationType::Test,
        status,
        started_at: Utc::now(),
        completed_at: Some(Utc::now()),
        duration_ms,
        output: output_str,
        errors,
        metrics: VerificationMetrics {
            test_count: Some(test_count),
            passed_count: Some(passed_count),
            failed_count: Some(failed_count),
            ..Default::default()
        },
    })
}

/// Run pytest
pub async fn run_pytest(spec_id: &str, timeout_secs: u64) -> Result<VerificationResult> {
    let start = Instant::now();
    let id = Uuid::new_v4();

    let output = tokio::time::timeout(
        std::time::Duration::from_secs(timeout_secs),
        tokio::task::spawn_blocking(move || {
            Command::new("pytest").args(["-v", "--tb=short"]).output()
        }),
    )
    .await
    .map_err(|_| VerifyError::Timeout("Test execution timed out".to_string()))?
    .map_err(|e| VerifyError::TestRunnerError(e.to_string()))??;

    let duration_ms = start.elapsed().as_millis() as u64;
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();

    let passed = output.status.success();
    let status = if passed {
        VerificationStatus::Passed
    } else {
        VerificationStatus::Failed
    };

    let errors = if !passed { vec![stderr_str] } else { vec![] };

    Ok(VerificationResult {
        id,
        spec_id: spec_id.to_string(),
        verification_type: VerificationType::Test,
        status,
        started_at: Utc::now(),
        completed_at: Some(Utc::now()),
        duration_ms,
        output: output_str,
        errors,
        metrics: Default::default(),
    })
}
