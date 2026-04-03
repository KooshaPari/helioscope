//! Benchmarks for harness_spec

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use harness_spec::{
    Checkpoint, ExecutionContext, ExecutionResult, ExecutionStatus, ParseOptions, Resources,
    RollbackConfig, RollbackStrategy, SpecContent, Specification, SuccessCriterion,
    VerificationRule,
};

fn bench_specification_new(c: &mut Criterion) {
    let spec_content = SpecContent {
        name: "test-spec".to_string(),
        version: "1.0.0".to_string(),
        owner: "test-team".to_string(),
        verification: vec![],
        rollback: RollbackConfig::default(),
        success_criteria: vec![],
        behavior: None,
        resources: None,
        metadata: std::collections::HashMap::new(),
    };
    c.bench_function("specification_new", |b| {
        b.iter(|| Specification { spec: black_box(spec_content.clone()) });
    });
}

fn bench_parse_options_default(c: &mut Criterion) {
    c.bench_function("parse_options_default", |b| {
        b.iter(|| ParseOptions::default());
    });
}

fn bench_parse_options_strict(c: &mut Criterion) {
    c.bench_function("parse_options_strict", |b| {
        b.iter(|| ParseOptions::strict());
    });
}

fn bench_verification_rule_test(c: &mut Criterion) {
    c.bench_function("verification_rule_test", |b| {
        b.iter(|| VerificationRule::Test { name: "unit-tests".to_string(), timeout_seconds: 300 });
    });
}

fn bench_verification_rule_security(c: &mut Criterion) {
    c.bench_function("verification_rule_security", |b| {
        b.iter(|| VerificationRule::Security {
            scanner: "bandit".to_string(),
            critical_only: true,
        });
    });
}

fn bench_verification_rule_performance(c: &mut Criterion) {
    c.bench_function("verification_rule_performance", |b| {
        b.iter(|| VerificationRule::Performance {
            metric: "latency_ms".to_string(),
            threshold: "<100".to_string(),
        });
    });
}

fn bench_rollback_config_default(c: &mut Criterion) {
    c.bench_function("rollback_config_default", |b| {
        b.iter(|| RollbackConfig::default());
    });
}

fn bench_rollback_config_custom(c: &mut Criterion) {
    c.bench_function("rollback_config_custom", |b| {
        b.iter(|| RollbackConfig {
            strategy: RollbackStrategy::Snapshot,
            checkpoint_required: true,
            timeout_seconds: 60,
        });
    });
}

fn bench_resources_new(c: &mut Criterion) {
    c.bench_function("resources_new", |b| {
        b.iter(|| Resources {
            cpu_cores: Some(4),
            memory_mb: Some(8192),
            timeout_seconds: Some(3600),
        });
    });
}

fn bench_success_criterion_new(c: &mut Criterion) {
    c.bench_function("success_criterion_new", |b| {
        b.iter(|| SuccessCriterion {
            metric: "accuracy".to_string(),
            threshold: Some(">0.95".to_string()),
            minimum: Some(0.95),
            maximum: None,
        });
    });
}

fn bench_execution_context_new(c: &mut Criterion) {
    c.bench_function("execution_context_new", |b| {
        b.iter(|| ExecutionContext {
            id: uuid::Uuid::new_v4(),
            spec_id: "spec-123".to_string(),
            checkpoint_id: Some("checkpoint-456".to_string()),
            status: ExecutionStatus::Pending,
            started_at: chrono::Utc::now(),
            completed_at: None,
            results: vec![],
        });
    });
}

fn bench_execution_result_new(c: &mut Criterion) {
    c.bench_function("execution_result_new", |b| {
        b.iter(|| ExecutionResult {
            step: "test-step".to_string(),
            success: true,
            message: "All checks passed".to_string(),
            duration_ms: 150,
            error: None,
        });
    });
}

fn bench_checkpoint_new(c: &mut Criterion) {
    c.bench_function("checkpoint_new", |b| {
        b.iter(|| Checkpoint {
            id: uuid::Uuid::new_v4(),
            spec_id: "spec-123".to_string(),
            git_sha: Some("abc123".to_string()),
            config_snapshot: Some(serde_json::json!({"key": "value"})),
            created_at: chrono::Utc::now(),
            status: harness_spec::CheckpointStatus::Pending,
        });
    });
}

fn bench_spec_content_serialization(c: &mut Criterion) {
    let spec_content = SpecContent {
        name: "test-spec".to_string(),
        version: "1.0.0".to_string(),
        owner: "test-team".to_string(),
        verification: vec![
            VerificationRule::Test { name: "unit-tests".to_string(), timeout_seconds: 300 },
            VerificationRule::Security { scanner: "bandit".to_string(), critical_only: true },
        ],
        rollback: RollbackConfig::default(),
        success_criteria: vec![SuccessCriterion {
            metric: "accuracy".to_string(),
            threshold: Some(">0.95".to_string()),
            minimum: Some(0.95),
            maximum: None,
        }],
        behavior: None,
        resources: Some(Resources {
            cpu_cores: Some(4),
            memory_mb: Some(8192),
            timeout_seconds: Some(3600),
        }),
        metadata: std::collections::HashMap::new(),
    };
    c.bench_function("spec_content_serialization", |b| {
        b.iter(|| serde_json::to_string(&black_box(&spec_content)).unwrap());
    });
}

fn bench_spec_content_deserialization(c: &mut Criterion) {
    let json = r#"{
        "name": "test-spec",
        "version": "1.0.0",
        "owner": "test-team",
        "verification": [
            {"type": "test", "name": "unit-tests", "timeout_seconds": 300}
        ],
        "rollback": {"strategy": "git_revert", "checkpoint_required": true, "timeout_seconds": 30},
        "success_criteria": [
            {"metric": "accuracy", "threshold": ">0.95", "minimum": 0.95}
        ]
    }"#;
    c.bench_function("spec_content_deserialization", |b| {
        b.iter(|| serde_json::from_str::<SpecContent>(black_box(json)).unwrap());
    });
}

criterion_group!(
    benches,
    bench_specification_new,
    bench_parse_options_default,
    bench_parse_options_strict,
    bench_verification_rule_test,
    bench_verification_rule_security,
    bench_verification_rule_performance,
    bench_rollback_config_default,
    bench_rollback_config_custom,
    bench_resources_new,
    bench_success_criterion_new,
    bench_execution_context_new,
    bench_execution_result_new,
    bench_checkpoint_new,
    bench_spec_content_serialization,
    bench_spec_content_deserialization
);
criterion_main!(benches);
