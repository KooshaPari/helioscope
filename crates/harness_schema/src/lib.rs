//! Schema validation module for heliosHarness
//! 
//! Provides types and validation for configuration schemas.
//!
//! # Example
//!
//! ```rust
//! use harness_schema::{Schema, Command};
//!
//! let schema = Schema {
//!     name: "my_schema".to_string(),
//!     commands: vec![Command {
//!         name: "test".to_string(),
//!         command: "echo test".to_string(),
//!     }],
//! };
//! assert!(schema.validate().is_ok());
//! ```

use serde::{Deserialize, Serialize};

/// Schema definition containing commands and metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    /// Name of the schema
    pub name: String,
    /// List of commands defined in the schema
    pub commands: Vec<Command>,
}

/// A single command within a schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Command {
    /// Name of the command
    pub name: String,
    /// Command string to execute
    pub command: String,
}

impl Schema {
    /// Validates the schema
    /// 
    /// # Errors
    /// 
    /// Returns an error if the schema name is empty
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("schema name required".to_string());
        }
        Ok(())
    }

    /// Returns the number of commands in the schema
    pub fn command_count(&self) -> usize {
        self.commands.len()
    }

    /// Finds a command by name
    pub fn find_command(&self, name: &str) -> Option<&Command> {
        self.commands.iter().find(|c| c.name == name)
    }
}

impl Command {
    /// Creates a new command
    pub fn new(name: &str, command: &str) -> Self {
        Self {
            name: name.to_string(),
            command: command.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_validation_valid() {
        let schema = Schema {
            name: "test".to_string(),
            commands: vec![],
        };
        assert!(schema.validate().is_ok());
    }

    #[test]
    fn test_schema_validation_empty_name() {
        let schema = Schema {
            name: "".to_string(),
            commands: vec![],
        };
        assert!(schema.validate().is_err());
    }

    #[test]
    fn test_command_count() {
        let schema = Schema {
            name: "test".to_string(),
            commands: vec![Command::new("a", "b"), Command::new("c", "d")],
        };
        assert_eq!(schema.command_count(), 2);
    }

    #[test]
    fn test_find_command() {
        let schema = Schema {
            name: "test".to_string(),
            commands: vec![Command::new("test", "echo test")],
        };
        assert!(schema.find_command("test").is_some());
        assert!(schema.find_command("missing").is_none());
    }
}
