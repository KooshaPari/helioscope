//! Specification generator

use crate::classifier::IntentClassifier;
use crate::error::{ElicitationError, Result};
use crate::intent::{ClassifiedIntent, Entity, Intent};
use chrono::Utc;
use harness_spec::models::{
    BehaviorSpec, RollbackConfig, SpecContent, Specification, SuccessCriterion, VerificationRule,
};

/// Specification generator
pub struct SpecGenerator;

impl Default for SpecGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl SpecGenerator {
    /// Create new generator
    pub fn new() -> Self {
        Self
    }

    /// Generate specification from classified intent
    pub fn generate(&self, intent: &ClassifiedIntent) -> Result<Specification> {
        // Generate spec name
        let name = self.generate_name(intent);

        // Generate verification rules
        let verification = self.generate_verification(intent);

        // Generate success criteria
        let success_criteria = self.generate_success_criteria(intent);

        // Generate behavior (BDD-style)
        let behavior = self.generate_behavior(intent);

        let spec = Specification {
            spec: SpecContent {
                name,
                version: "1.0.0".to_string(),
                owner: "elicitated".to_string(),
                verification,
                rollback: RollbackConfig::default(),
                success_criteria,
                behavior: Some(behavior),
                resources: None,
                metadata: self.generate_metadata(intent),
            },
        };

        Ok(spec)
    }

    /// Generate spec name
    fn generate_name(&self, intent: &ClassifiedIntent) -> String {
        let action = match intent.intent {
            Intent::Fix => "fix",
            Intent::Feature => "feature",
            Intent::Refactor => "refactor",
            Intent::Deploy => "deploy",
            Intent::Test => "test",
            Intent::Research => "research",
            Intent::Document => "document",
            Intent::Configure => "configure",
            Intent::Review => "review",
            Intent::Optimize => "optimize",
            Intent::Unknown => "unknown",
        };

        // Try to extract key entity
        let entity = intent
            .entities
            .first()
            .map(|e| e.value.clone())
            .unwrap_or_else(|| " unspecified".to_string());

        format!("{}-{}", action, entity.replace(['/', '.'], "_"))
    }

    /// Generate verification rules
    fn generate_verification(&self, intent: &ClassifiedIntent) -> Vec<VerificationRule> {
        intent
            .intent
            .default_verification()
            .iter()
            .map(|v| match *v {
                "test" => VerificationRule::Test {
                    name: "unit_tests".to_string(),
                    timeout_seconds: 300,
                },
                "lint" => VerificationRule::Custom {
                    command: "cargo clippy".to_string(),
                    expected_exit_code: 0,
                },
                "smoke_test" => VerificationRule::Test {
                    name: "smoke_tests".to_string(),
                    timeout_seconds: 60,
                },
                "validate_config" => VerificationRule::Custom {
                    command: "validate-config".to_string(),
                    expected_exit_code: 0,
                },
                "benchmark" => VerificationRule::Performance {
                    metric: "latency".to_string(),
                    threshold: "<100ms".to_string(),
                },
                _ => VerificationRule::Test {
                    name: v.to_string(),
                    timeout_seconds: 300,
                },
            })
            .collect()
    }

    /// Generate success criteria
    fn generate_success_criteria(&self, intent: &ClassifiedIntent) -> Vec<SuccessCriterion> {
        match intent.intent {
            Intent::Fix => vec![SuccessCriterion {
                metric: "tests_pass".to_string(),
                threshold: Some("true".to_string()),
                minimum: None,
                maximum: None,
            }],
            Intent::Feature => vec![
                SuccessCriterion {
                    metric: "tests_pass".to_string(),
                    threshold: Some("true".to_string()),
                    minimum: None,
                    maximum: None,
                },
                SuccessCriterion {
                    metric: "test_coverage".to_string(),
                    threshold: None,
                    minimum: Some(80.0),
                    maximum: None,
                },
            ],
            Intent::Optimize => vec![SuccessCriterion {
                metric: "performance_improvement".to_string(),
                threshold: None,
                minimum: Some(10.0),
                maximum: None,
            }],
            _ => vec![SuccessCriterion {
                metric: "tests_pass".to_string(),
                threshold: Some("true".to_string()),
                minimum: None,
                maximum: None,
            }],
        }
    }

    /// Generate behavior spec (BDD-style)
    fn generate_behavior(&self, intent: &ClassifiedIntent) -> BehaviorSpec {
        let (given, when, then): (String, &str, &str) = match intent.intent {
            Intent::Fix => (
                format!(
                    "system has a bug in {}",
                    intent
                        .entities
                        .first()
                        .map(|e| e.value.as_str())
                        .unwrap_or("unknown component")
                ),
                "agent executes fix",
                "bug is resolved and tests pass",
            ),
            Intent::Feature => (
                "user requests new functionality".to_string(),
                "agent implements feature",
                "feature works as specified",
            ),
            Intent::Refactor => (
                "code needs improvement".to_string(),
                "agent refactors code",
                "functionality preserved, code quality improved",
            ),
            Intent::Deploy => (
                "code is ready for deployment".to_string(),
                "agent deploys to target environment",
                "deployment successful and verified",
            ),
            _ => (
                "user provides direction".to_string(),
                "agent executes task",
                "task completes successfully",
            ),
        };

        BehaviorSpec {
            given,
            when: when.to_string(),
            then: then.to_string(),
            and: vec![],
        }
    }

    /// Generate metadata
    fn generate_metadata(
        &self,
        intent: &ClassifiedIntent,
    ) -> std::collections::HashMap<String, String> {
        let mut metadata = std::collections::HashMap::new();

        metadata.insert("elicited_at".to_string(), Utc::now().to_rfc3339());
        metadata.insert("original_input".to_string(), intent.original_input.clone());
        metadata.insert("intent_type".to_string(), format!("{:?}", intent.intent));
        metadata.insert("confidence".to_string(), intent.confidence.to_string());

        metadata
    }
}

/// Main elicitation handler
pub struct ElicitationHandler {
    classifier: IntentClassifier,
    generator: SpecGenerator,
}

impl Default for ElicitationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ElicitationHandler {
    /// Create new handler
    pub fn new() -> Self {
        Self {
            classifier: IntentClassifier::new(),
            generator: SpecGenerator::new(),
        }
    }

    /// Process user input and generate specification
    pub fn process(&self, input: &str) -> Result<Specification> {
        // Classify intent
        let intent = self.classifier.classify(input)?;

        // Check confidence threshold
        if intent.confidence < 0.1 {
            return Err(ElicitationError::AmbiguousError(format!(
                "Input '{}' is ambiguous (confidence: {:.2})",
                input, intent.confidence
            )));
        }

        // Generate specification
        let spec = self.generator.generate(&intent)?;

        Ok(spec)
    }

    /// Just classify without generating spec
    pub fn classify(&self, input: &str) -> Result<ClassifiedIntent> {
        self.classifier.classify(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_fix_spec() {
        let handler = ElicitationHandler::new();
        let spec = handler.process("Fix the login bug in auth.rs").unwrap();

        assert!(spec.spec.name.contains("fix"));
        assert!(!spec.spec.verification.is_empty());
    }

    #[test]
    fn test_generate_feature_spec() {
        let handler = ElicitationHandler::new();
        let spec = handler.process("Add OAuth login").unwrap();

        assert!(spec.spec.name.contains("feature"));
    }

    #[test]
    fn test_behavior_generation() {
        let handler = ElicitationHandler::new();
        let spec = handler.process("Fix the bug").unwrap();

        assert!(spec.spec.behavior.is_some());
    }
}
