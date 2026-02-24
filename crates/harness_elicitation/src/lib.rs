//! Elicitation Handler
//! 
//! Converts natural language ideas into specifications for autonomous execution.

pub mod classifier;
pub mod generator;
pub mod error;
pub mod intent;

pub use classifier::*;
pub use generator::*;
pub use error::*;
pub use intent::*;
