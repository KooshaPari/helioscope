//! Specification parser

use crate::error::{Result, SpecError};
use crate::models::*;

/// Parse specification from YAML content
pub fn parse_yaml(content: &str) -> Result<Specification> {
    serde_yaml::from_str(content).map_err(|e| SpecError::ParseError(e.to_string()))
}

/// Parse specification from JSON content
pub fn parse_json(content: &str) -> Result<Specification> {
    serde_json::from_str(content).map_err(|e| SpecError::JsonParseError(e.to_string()))
}

/// Parse specification with automatic format detection
pub fn parse(content: &str, format: SpecFormat) -> Result<Specification> {
    match format {
        SpecFormat::Yaml => parse_yaml(content),
        SpecFormat::Json => parse_json(content),
    }
}

/// Auto-detect format from content
pub fn parse_auto(content: &str) -> Result<Specification> {
    let trimmed = content.trim();

    if trimmed.starts_with('{') {
        parse_json(trimmed)
    } else {
        parse_yaml(trimmed)
    }
}

/// Specification format
#[derive(Debug, Clone, Copy, Default)]
pub enum SpecFormat {
    #[default]
    Yaml,
    Json,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_yaml() {
        let yaml = r#"
spec:
  name: test-spec
  version: "1.0.0"
  owner: test-team
  verification:
    - type: test
      name: unit_tests
  rollback:
    strategy: git_revert
    checkpoint_required: true
"#;
        let spec = parse_yaml(yaml).unwrap();
        assert_eq!(spec.spec.name, "test-spec");
        assert_eq!(spec.spec.version, "1.0.0");
    }

    #[test]
    fn test_parse_json() {
        let json = r#"{
            "spec": {
                "name": "test-spec",
                "version": "1.0.0",
                "verification": []
            }
        }"#;
        let spec = parse_json(json).unwrap();
        assert_eq!(spec.spec.name, "test-spec");
    }

    #[test]
    fn test_auto_detect_yaml() {
        let content = "spec:\n  name: test\n  verification: []";
        let spec = parse_auto(content).unwrap();
        assert_eq!(spec.spec.name, "test");
    }

    #[test]
    fn test_auto_detect_json() {
        let content = r#"{"spec": {"name": "test", "verification": []}}"#;
        let spec = parse_auto(content).unwrap();
        assert_eq!(spec.spec.name, "test");
    }
}
