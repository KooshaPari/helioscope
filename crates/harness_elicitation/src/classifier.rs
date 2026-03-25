//! Intent classifier

use crate::error::Result;
use crate::intent::{ClassifiedIntent, Entity, EntityType, Intent};
use regex::Regex;
use std::collections::HashMap;

/// Intent classifier
pub struct IntentClassifier {
    patterns: HashMap<Intent, Vec<Regex>>,
}

impl Default for IntentClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl IntentClassifier {
    /// Create new classifier
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // Fix patterns
        patterns.insert(
            Intent::Fix,
            vec![
                Regex::new(r"(?i)\bfix\b").unwrap(),
                Regex::new(r"(?i)\bbug\b").unwrap(),
                Regex::new(r"(?i)\brepair\b").unwrap(),
                Regex::new(r"(?i)\bdebug\b").unwrap(),
                Regex::new(r"(?i)\berror\b").unwrap(),
                Regex::new(r"(?i)\bcrash\b").unwrap(),
                Regex::new(r"(?i)\bfails?\b").unwrap(),
                Regex::new(r"(?i)\bbroken\b").unwrap(),
            ],
        );

        // Feature patterns
        patterns.insert(
            Intent::Feature,
            vec![
                Regex::new(r"(?i)\badd\b").unwrap(),
                Regex::new(r"(?i)\bimplement\b").unwrap(),
                Regex::new(r"(?i)\bcreate\b").unwrap(),
                Regex::new(r"(?i)\bnew\b").unwrap(),
                Regex::new(r"(?i)\bfeature\b").unwrap(),
                Regex::new(r"(?i)\benable\b").unwrap(),
            ],
        );

        // Refactor patterns
        patterns.insert(
            Intent::Refactor,
            vec![
                Regex::new(r"(?i)\brefactor\b").unwrap(),
                Regex::new(r"(?i)\brewrite\b").unwrap(),
                Regex::new(r"(?i)\bcleanup\b").unwrap(),
                Regex::new(r"(?i)\brename\b").unwrap(),
                Regex::new(r"(?i)\bmove\b").unwrap(),
            ],
        );

        // Deploy patterns
        patterns.insert(
            Intent::Deploy,
            vec![
                Regex::new(r"(?i)\bdeploy\b").unwrap(),
                Regex::new(r"(?i)\brelease\b").unwrap(),
                Regex::new(r"(?i)\bpush\b").unwrap(),
                Regex::new(r"(?i)\bpublish\b").unwrap(),
            ],
        );

        // Test patterns
        patterns.insert(
            Intent::Test,
            vec![
                Regex::new(r"(?i)\btest\b").unwrap(),
                Regex::new(r"(?i)\bspec\b").unwrap(),
                Regex::new(r"(?i)\bverify\b").unwrap(),
            ],
        );

        // Research patterns
        patterns.insert(
            Intent::Research,
            vec![
                Regex::new(r"(?i)\bresearch\b").unwrap(),
                Regex::new(r"(?i)\binvestigate\b").unwrap(),
                Regex::new(r"(?i)\bfind\b").unwrap(),
                Regex::new(r"(?i)\blook\s+up\b").unwrap(),
            ],
        );

        // Document patterns
        patterns.insert(
            Intent::Document,
            vec![
                Regex::new(r"(?i)\bdocument\b").unwrap(),
                Regex::new(r"(?i)\bdocs?\b").unwrap(),
                Regex::new(r"(?i)\bwrite\b").unwrap(),
            ],
        );

        // Configure patterns
        patterns.insert(
            Intent::Configure,
            vec![
                Regex::new(r"(?i)\bconfig\b").unwrap(),
                Regex::new(r"(?i)\bconfigure\b").unwrap(),
                Regex::new(r"(?i)\bsetup\b").unwrap(),
                Regex::new(r"(?i)\bsettings?\b").unwrap(),
            ],
        );

        // Review patterns
        patterns.insert(
            Intent::Review,
            vec![
                Regex::new(r"(?i)\breview\b").unwrap(),
                Regex::new(r"(?i)\baudit\b").unwrap(),
                Regex::new(r"(?i)\bcheck\b").unwrap(),
            ],
        );

        // Optimize patterns
        patterns.insert(
            Intent::Optimize,
            vec![
                Regex::new(r"(?i)\boptimize\b").unwrap(),
                Regex::new(r"(?i)\bimprove\b").unwrap(),
                Regex::new(r"(?i)\bfaster\b").unwrap(),
                Regex::new(r"(?i)\bperformance\b").unwrap(),
            ],
        );

        Self { patterns }
    }

    /// Classify intent from input
    pub fn classify(&self, input: &str) -> Result<ClassifiedIntent> {
        let input_lower = input.to_lowercase();

        // Score each intent
        let mut scores: HashMap<Intent, f64> = HashMap::new();

        for (intent, regexes) in &self.patterns {
            let mut score = 0.0;
            for regex in regexes {
                if regex.is_match(&input_lower) {
                    score += 1.0;
                }
            }
            if score > 0.0 {
                scores.insert(intent.clone(), score);
            }
        }

        // Find best match
        let best = scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, s)| (i.clone(), *s));

        let (intent, score) = match best {
            Some((intent, score)) => (intent, score),
            None => (Intent::Unknown, 0.0),
        };

        // Calculate confidence (normalize by max possible score)
        let max_score = self
            .patterns
            .get(&intent)
            .map(|r| r.len() as f64)
            .unwrap_or(1.0);
        let confidence = (score / max_score).min(1.0);

        // Extract entities
        let entities = self.extract_entities(input);

        Ok(ClassifiedIntent {
            intent,
            confidence,
            entities,
            original_input: input.to_string(),
        })
    }

    /// Extract entities from input
    fn extract_entities(&self, input: &str) -> Vec<Entity> {
        let mut entities = Vec::new();

        // Extract file paths
        let file_regex = Regex::new(r"(\S+\.(rs|py|js|ts|yaml|json|toml))").unwrap();
        for cap in file_regex.captures_iter(input) {
            if let Some(m) = cap.get(1) {
                entities.push(Entity {
                    entity_type: EntityType::File,
                    value: m.as_str().to_string(),
                    confidence: 0.9,
                });
            }
        }

        // Extract function names (simple pattern)
        let func_regex = Regex::new(r"(?:function|fn|def|method)\s+(\w+)").unwrap();
        for cap in func_regex.captures_iter(input) {
            if let Some(m) = cap.get(1) {
                entities.push(Entity {
                    entity_type: EntityType::Function,
                    value: m.as_str().to_string(),
                    confidence: 0.8,
                });
            }
        }

        entities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_fix() {
        let classifier = IntentClassifier::new();
        let result = classifier.classify("Fix the auth bug in login").unwrap();
        assert_eq!(result.intent, Intent::Fix);
        assert!(result.confidence > 0.1);
    }

    #[test]
    fn test_classify_feature() {
        let classifier = IntentClassifier::new();
        let result = classifier.classify("Add OAuth login support").unwrap();
        assert_eq!(result.intent, Intent::Feature);
    }

    #[test]
    fn test_classify_refactor() {
        let classifier = IntentClassifier::new();
        let result = classifier.classify("Refactor the user service").unwrap();
        assert_eq!(result.intent, Intent::Refactor);
    }

    #[test]
    fn test_extract_entities() {
        let classifier = IntentClassifier::new();
        let result = classifier.classify("Fix auth.rs").unwrap();
        assert!(!result.entities.is_empty());
    }
}
