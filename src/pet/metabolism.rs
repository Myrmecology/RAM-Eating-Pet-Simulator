// src/pet/metabolism.rs
// RAM Eating Pet Simulator - Metabolism System

use serde::{Deserialize, Serialize};

/// Manages how the pet digests RAM over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metabolism {
    /// Base metabolic rate (MB per second)
    base_rate: f32,
    /// Current metabolic modifier
    modifier: f32,
    /// Accumulated digestion time
    digestion_timer: f32,
}

impl Metabolism {
    /// Create a new metabolism system
    pub fn new(base_rate: f32) -> Self {
        Metabolism {
            base_rate,
            modifier: 1.0,
            digestion_timer: 0.0,
        }
    }
    
    /// Process metabolism for a time delta
    /// Returns the amount of MB to digest
    pub fn process(&mut self, current_size: usize, delta_time: f32) -> usize {
        // Don't digest if too small
        if current_size < 10 {
            return 0;
        }
        
        // Accumulate digestion time
        self.digestion_timer += delta_time;
        
        // Calculate effective rate based on size
        let size_modifier = self.calculate_size_modifier(current_size);
        let effective_rate = self.base_rate * self.modifier * size_modifier;
        
        // Calculate how much to digest
        let to_digest = (effective_rate * self.digestion_timer) as usize;
        
        if to_digest > 0 {
            // Reset timer, keeping remainder
            self.digestion_timer = self.digestion_timer.fract();
            to_digest.min(current_size / 2) // Never digest more than half
        } else {
            0
        }
    }
    
    /// Calculate metabolic modifier based on size
    fn calculate_size_modifier(&self, size_mb: usize) -> f32 {
        // Larger pets digest faster
        match size_mb {
            0..=100 => 0.5,      // Baby metabolism - slow
            101..=300 => 0.8,    // Young metabolism
            301..=600 => 1.0,    // Normal metabolism
            601..=1000 => 1.2,   // Fast metabolism
            1001..=1500 => 1.5,  // Very fast
            _ => 2.0,            // Extreme metabolism
        }
    }
    
    /// Boost metabolism temporarily
    pub fn boost(&mut self, multiplier: f32) {
        self.modifier = (self.modifier * multiplier).min(3.0);
    }
    
    /// Slow metabolism temporarily
    pub fn slow(&mut self, divisor: f32) {
        self.modifier = (self.modifier / divisor).max(0.1);
    }
    
    /// Reset metabolism to normal
    pub fn reset(&mut self) {
        self.modifier = 1.0;
        self.digestion_timer = 0.0;
    }
    
    /// Get current metabolic rate
    pub fn get_rate(&self, size_mb: usize) -> f32 {
        self.base_rate * self.modifier * self.calculate_size_modifier(size_mb)
    }
}

/// Metabolism states for special conditions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetabolismState {
    Normal,
    Hibernating,  // Very slow metabolism
    Hyperactive,  // Very fast metabolism
    Sick,         // Irregular metabolism
}

impl MetabolismState {
    /// Get modifier for this state
    pub fn get_modifier(&self) -> f32 {
        match self {
            MetabolismState::Normal => 1.0,
            MetabolismState::Hibernating => 0.2,
            MetabolismState::Hyperactive => 2.5,
            MetabolismState::Sick => 0.7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metabolism_creation() {
        let metabolism = Metabolism::new(1.0);
        assert_eq!(metabolism.base_rate, 1.0);
        assert_eq!(metabolism.modifier, 1.0);
    }
    
    #[test]
    fn test_metabolism_process() {
        let mut metabolism = Metabolism::new(10.0);
        let digested = metabolism.process(100, 1.0);
        assert!(digested > 0);
        assert!(digested <= 50); // Should not digest more than half
    }
    
    #[test]
    fn test_size_affects_metabolism() {
        let metabolism = Metabolism::new(1.0);
        let small_rate = metabolism.get_rate(50);
        let large_rate = metabolism.get_rate(1500);
        assert!(large_rate > small_rate);
    }
}