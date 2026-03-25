//! TDD patterns - Red-Green-Refactor cycle

/// TDD phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TddPhase {
    Red,
    Green,
    Refactor,
}

/// Test-Driven Development orchestrator
pub struct TestDriven {
    phase: TddPhase,
    cycles: u32,
}

impl TestDriven {
    pub fn new() -> Self {
        Self {
            phase: TddPhase::Red,
            cycles: 0,
        }
    }
    
    pub fn next_phase(&mut self) {
        self.phase = match self.phase {
            TddPhase::Red => TddPhase::Green,
            TddPhase::Green => TddPhase::Refactor,
            TddPhase::Refactor => {
                self.cycles += 1;
                TddPhase::Red
            }
        };
    }
    
    pub fn phase(&self) -> TddPhase {
        self.phase
    }
    
    pub fn cycles(&self) -> u32 {
        self.cycles
    }
}

impl Default for TestDriven {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cycle() {
        let mut tdd = TestDriven::new();
        assert_eq!(tdd.phase(), TddPhase::Red);
        tdd.next_phase();
        assert_eq!(tdd.phase(), TddPhase::Green);
        tdd.next_phase();
        assert_eq!(tdd.phase(), TddPhase::Refactor);
        tdd.next_phase();
        assert_eq!(tdd.phase(), TddPhase::Red);
        assert_eq!(tdd.cycles(), 1);
    }
}
