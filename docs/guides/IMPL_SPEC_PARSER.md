# Implementation Guide: Spec Parser

**Date:** 2026-02-24
**Status:** In Progress

---

## Quick Start

### 1. Define Specification Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["spec"],
  "properties": {
    "spec": {
      "type": "object",
      "required": ["name", "verification"],
      "properties": {
        "name": { "type": "string" },
        "version": { "type": "string" },
        "verification": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "test": { "type": "string" },
              "security": { "type": "string" }
            }
          }
        },
        "rollback": {
          "type": "object",
          "properties": {
            "strategy": { "type": "string", "enum": ["git_revert", "snapshot"] }
          }
        },
        "success_criteria": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    }
  }
}
```

### 2. Implement Parser

```rust
// src/spec/parser.rs

use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Specification {
    pub spec: SpecContent,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecContent {
    pub name: String,
    pub version: Option<String>,
    pub verification: Vec<VerificationRule>,
    pub rollback: Option<RollbackConfig>,
    pub success_criteria: Option<Vec<String>>,
}

pub fn parse_spec(content: &str) -> Result<Specification, SpecError> {
    serde_yaml::from_str(content)
        .map_err(|e| SpecError::ParseError(e.to_string()))
}
```

### 3. Add Validation

```rust
pub fn validate_spec(spec: &Specification) -> Result<(), ValidationError> {
    if spec.spec.name.is_empty() {
        return Err(ValidationError::MissingField("name"));
    }
    if spec.spec.verification.is_empty() {
        return Err(ValidationError::MissingField("verification"));
    }
    Ok(())
}
```

---

## File Structure

```
src/spec/
├── mod.rs          # Module exports
├── parser.rs       # YAML/JSON parsing
├── schema.rs       # JSON Schema validation
├── models.rs       # Data structures
├── generator.rs    # NL to spec conversion
└── validation.rs   # Business logic validation
```

---

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_valid_spec() {
        let yaml = r#"
spec:
  name: test-spec
  verification:
    - test: unit_tests
"#;
        let spec = parse_spec(yaml).unwrap();
        assert_eq!(spec.spec.name, "test-spec");
    }
}
```

---

## Next Steps

1. Add JSON support
2. Implement schema validation
3. Add version resolution
4. Create CLI tool for testing
