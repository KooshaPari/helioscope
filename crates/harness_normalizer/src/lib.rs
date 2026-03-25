//! Normalizer module - Data normalization for heliosHarness

use std::collections::HashMap;

/// Normalization result
#[derive(Debug, Clone)]
pub struct NormalizedData {
    pub value: String,
    pub normalized: bool,
    pub metadata: HashMap<String, String>,
}

impl NormalizedData {
    pub fn new(value: String) -> Self {
        Self {
            value,
            normalized: false,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: &str, val: &str) -> Self {
        self.metadata.insert(key.to_string(), val.to_string());
        self
    }
}

/// Normalizer for different data types
pub struct Normalizer {
    trim: bool,
    lowercase: bool,
    remove_special: bool,
}

impl Normalizer {
    pub fn new() -> Self {
        Self {
            trim: true,
            lowercase: false,
            remove_special: false,
        }
    }

    pub fn with_trim(mut self, enabled: bool) -> Self {
        self.trim = enabled;
        self
    }
    pub fn with_lowercase(mut self, enabled: bool) -> Self {
        self.lowercase = enabled;
        self
    }
    pub fn with_remove_special(mut self, enabled: bool) -> Self {
        self.remove_special = enabled;
        self
    }

    pub fn normalize(&self, input: &str) -> NormalizedData {
        let mut result = input.to_string();

        if self.trim {
            result = result.trim().to_string();
        }
        if self.lowercase {
            result = result.to_lowercase();
        }
        if self.remove_special {
            result = result
                .chars()
                .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                .collect();
        }

        NormalizedData::new(result).with_metadata("normalizer", "default")
    }

    pub fn normalize_json(&self, json: &str) -> Result<NormalizedData, String> {
        // Basic JSON normalization - remove whitespace
        let normalized: String = json.chars().filter(|c| !c.is_whitespace()).collect();
        Ok(NormalizedData::new(normalized).with_metadata("type", "json"))
    }

    pub fn normalize_url(&self, url: &str) -> NormalizedData {
        let normalized = url.trim().to_lowercase();
        NormalizedData::new(normalized).with_metadata("type", "url")
    }

    pub fn normalize_path(&self, path: &str) -> NormalizedData {
        let normalized = path.replace("\\", "/");
        NormalizedData::new(normalized).with_metadata("type", "path")
    }
}

impl Default for Normalizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_normalization() {
        let n = Normalizer::new().with_lowercase(true);
        let result = n.normalize("  HELLO  ");
        assert_eq!(result.value, "hello");
    }

    #[test]
    fn test_url_normalization() {
        let n = Normalizer::new();
        let result = n.normalize_url("HTTP://Example.COM/Path ");
        assert_eq!(result.value, "http://example.com/path");
    }
}
