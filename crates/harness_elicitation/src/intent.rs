//! Intent types

use serde::{Deserialize, Serialize};

/// Intent types that can be classified from user input
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    /// Fix a bug
    Fix,
    /// Add a feature
    Feature,
    /// Refactor code
    Refactor,
    /// Deploy
    Deploy,
    /// Test
    Test,
    /// Research
    Research,
    /// Document
    Document,
    /// Configure
    Configure,
    /// Review
    Review,
    /// Optimize
    Optimize,
    /// Unknown
    Unknown,
}

impl Intent {
    /// Get default verification rules for this intent
    pub fn default_verification(&self) -> Vec<&'static str> {
        match self {
            Intent::Fix => vec!["test"],
            Intent::Feature => vec!["test", "lint"],
            Intent::Refactor => vec!["test", "lint"],
            Intent::Deploy => vec!["smoke_test"],
            Intent::Test => vec!["test"],
            Intent::Research => vec![],
            Intent::Document => vec!["lint"],
            Intent::Configure => vec!["validate_config"],
            Intent::Review => vec![],
            Intent::Optimize => vec!["benchmark", "test"],
            Intent::Unknown => vec!["test"],
        }
    }
}

/// Classified intent with confidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedIntent {
    /// The intent
    pub intent: Intent,

    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,

    /// Extracted entities
    pub entities: Vec<Entity>,

    /// Original input
    pub original_input: String,
}

/// Entity extracted from input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Entity type
    pub entity_type: EntityType,

    /// Entity value
    pub value: String,

    /// Confidence
    pub confidence: f64,
}

/// Entity types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    /// File or path
    File,
    /// Function or method
    Function,
    /// Component
    Component,
    /// Feature name
    Feature,
    /// Bug description
    Bug,
    /// Configuration key
    Config,
    /// Test name
    Test,
    /// Module
    Module,
}
