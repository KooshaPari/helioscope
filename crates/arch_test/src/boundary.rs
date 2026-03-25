//! Boundary enforcement tests for hexagonal architecture

use std::path::Path;

/// Layer in hexagonal architecture
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layer {
    Domain,
    Application,
    Ports,
    Infrastructure,
    Adapters,
}

impl Layer {
    pub fn from_path(path: &Path) -> Option<Self> {
        let s = path.to_string_lossy();
        if s.contains("/domain/") { Some(Layer::Domain) }
        else if s.contains("/application/") { Some(Layer::Application) }
        else if s.contains("/ports/") { Some(Layer::Ports) }
        else if s.contains("/infrastructure/") { Some(Layer::Infrastructure) }
        else if s.contains("/adapters/") { Some(Layer::Adapters) }
        else { None }
    }
    
    pub fn allowed(&self) -> Vec<Layer> {
        match self {
            Layer::Domain => vec![],
            Layer::Application => vec![Layer::Domain, Layer::Ports],
            Layer::Ports => vec![],
            Layer::Infrastructure => vec![Layer::Domain, Layer::Ports],
            Layer::Adapters => vec![Layer::Domain, Layer::Ports, Layer::Application],
        }
    }
}

/// Boundary enforcer
pub struct BoundaryEnforcer {
    violations: Vec<(String, String)>,
}

impl BoundaryEnforcer {
    pub fn new() -> Self {
        Self { violations: Vec::new() }
    }
    
    pub fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }
}

impl Default for BoundaryEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_domain_layer() {
        assert!(Layer::from_path(Path::new("src/domain/model.rs")).is_some());
    }
    
    #[test]
    fn test_boundary_clean() {
        assert!(BoundaryEnforcer::new().is_clean());
    }
}
