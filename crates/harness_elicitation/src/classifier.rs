//! Intent classifier
//! Intent classifier

use crate::error::Result;
use crate::intent::{ClassifiedIntent, Entity, EntityType, Intent};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

/// Precompiled regex vectors to avoid repeated runtime compilation and panics.
static FIX_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\bfix\b").expect("invalid regex"),
        Regex::new(r"(?i)\bbug\b").expect("invalid regex"),
        Regex::new(r"(?i)\brepair\b").expect("invalid regex"),
        Regex::new(r"(?i)\bdebug\b").expect("invalid regex"),
        Regex::new(r"(?i)\berror\b").expect("invalid regex"),
        Regex::new(r"(?i)\bcrash\b").expect("invalid regex"),
        Regex::new(r"(?i)\bfails?\b").expect("invalid regex"),
        Regex::new(r"(?i)\bbroken\b").expect("invalid regex"),
    ]
});

static FEATURE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\badd\b").expect("invalid regex"),
        Regex::new(r"(?i)\bimplement\b").expect("invalid regex"),
        Regex::new(r"(?i)\bcreate\b").expect("invalid regex"),
        Regex::new(r"(?i)\bnew\b").expect("invalid regex"),
        Regex::new(r"(?i)\bfeature\b").expect("invalid regex"),
        Regex::new(r"(?i)\benable\b").expect("invalid regex"),
    ]
});

static REFACTOR_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\brefactor\b").expect("invalid regex"),
        Regex::new(r"(?i)\brewrite\b").expect("invalid regex"),
        Regex::new(r"(?i)\bcleanup\b").expect("invalid regex"),
        Regex::new(r"(?i)\brename\b").expect("invalid regex"),
        Regex::new(r"(?i)\bmove\b").expect("invalid regex"),
    ]
});

static DEPLOY_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\bdeploy\b").expect("invalid regex"),
        Regex::new(r"(?i)\brelease\b").expect("invalid regex"),
        Regex::new(r"(?i)\bpush\b").expect("invalid regex"),
        Regex::new(r"(?i)\bpublish\b").expect("invalid regex"),
    ]
});

static TEST_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\btest\b").expect("invalid regex"),
        Regex::new(r"(?i)\bspec\b").expect("invalid regex"),
        Regex::new(r"(?i)\bverify\b").expect("invalid regex"),
    ]
});

static RESEARCH_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\bresearch\b").expect("invalid regex"),
        Regex::new(r"(?i)\binvestigate\b").expect("invalid regex"),
        Regex::new(r"(?i)\bfind\b").expect("invalid regex"),
        Regex::new(r"(?i)\blook\s+up\b").expect("invalid regex"),
    ]
});

static DOCUMENT_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\bdocument\b").expect("invalid regex"),
        Regex::new(r"(?i)\bdocs?\b").expect("invalid regex"),
        Regex::new(r"(?i)\bwrite\b").expect("invalid regex"),
    ]
});

static CONFIGURE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\bconfig\b").expect("invalid regex"),
        Regex::new(r"(?i)\bconfigure\b").expect("invalid regex"),
        Regex::new(r"(?i)\bsetup\b").expect("invalid regex"),
        Regex::new(r"(?i)\bsettings?\b").expect("invalid regex"),
    ]
});

static REVIEW_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\breview\b").expect("invalid regex"),
        Regex::new(r"(?i)\baudit\b").expect("invalid regex"),
        Regex::new(r"(?i)\bcheck\b").expect("invalid regex"),
    ]
});

static OPTIMIZE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\boptimize\b").expect("invalid regex"),
        Regex::new(r"(?i)\bimprove\b").expect("invalid regex"),
        Regex::new(r"(?i)\bfaster\b").expect("invalid regex"),
        Regex::new(r"(?i)\bperformance\b").expect("invalid regex"),
    ]
});

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

        patterns.insert(Intent::Fix, FIX_PATTERNS.clone());
        patterns.insert(Intent::Feature, FEATURE_PATTERNS.clone());
        patterns.insert(Intent::Refactor, REFACTOR_PATTERNS.clone());
        patterns.insert(Intent::Deploy, DEPLOY_PATTERNS.clone());
        patterns.insert(Intent::Test, TEST_PATTERNS.clone());
        patterns.insert(Intent::Research, RESEARCH_PATTERNS.clone());
        patterns.insert(Intent::Document, DOCUMENT_PATTERNS.clone());
        patterns.insert(Intent::Configure, CONFIGURE_PATTERNS.clone());
        patterns.insert(Intent::Review, REVIEW_PATTERNS.clone());
        patterns.insert(Intent::Optimize, OPTIMIZE_PATTERNS.clone());

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
            .max_by(|a, b| a.1.total_cmp(b.1))
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
        static FILE_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(\S+\.(rs|py|js|ts|yaml|json|toml))").expect("invalid regex")
        });
        for cap in FILE_REGEX.captures_iter(input) {
            if let Some(m) = cap.get(1) {
                entities.push(Entity {
                    entity_type: EntityType::File,
                    value: m.as_str().to_string(),
                    confidence: 0.9,
                });
            }
        }

        // Extract function names (simple pattern)
        static FUNC_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?:function|fn|def|method)\s+(\w+)").expect("invalid regex"));
        for cap in FUNC_REGEX.captures_iter(input) {
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
        let result = classifier
            .classify("Fix the auth bug in login")
            .expect("classify failed");
        assert_eq!(result.intent, Intent::Fix);
        assert!(result.confidence > 0.1);
    }

    #[test]
    fn test_classify_feature() {
        let classifier = IntentClassifier::new();
        let result = classifier
            .classify("Add OAuth login support")
            .expect("classify failed");
        assert_eq!(result.intent, Intent::Feature);
    }

    #[test]
    fn test_classify_refactor() {
        let classifier = IntentClassifier::new();
        let result = classifier
            .classify("Refactor the user service")
            .expect("classify failed");
        assert_eq!(result.intent, Intent::Refactor);
    }

    #[test]
    fn test_extract_entities() {
        let classifier = IntentClassifier::new();
        let result = classifier.classify("Fix auth.rs").expect("classify failed");
        assert!(!result.entities.is_empty());
    }
}
