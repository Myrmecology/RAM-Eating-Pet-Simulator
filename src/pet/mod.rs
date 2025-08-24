// src/pet/mod.rs
// RAM Eating Pet Simulator - Pet Module

pub mod metabolism;
pub mod personality;
pub mod state;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::config::Config;
use personality::{Personality, Mood};
use state::PetState;
use metabolism::Metabolism;

/// The main Pet structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pet {
    /// Pet's name
    pub name: String,
    /// Current size in MB
    size_mb: usize,
    /// Pet's personality traits
    personality: Personality,
    /// Pet's current state
    state: PetState,
    /// Metabolism system
    metabolism: Metabolism,
    /// Current mood
    mood: Mood,
    /// Hunger level (0-100)
    hunger: f32,
    /// Happiness level (0-100)
    happiness: f32,
    /// Is the pet alive?
    alive: bool,
    /// Birth time
    #[serde(skip)]
    birth_time: Option<Instant>,
}

impl Pet {
    /// Create a new pet
    pub fn new(config: &Config) -> Result<Self> {
        let personality = Personality::generate_random();
        let name = personality.generate_name();
        
        Ok(Pet {
            name: name.clone(),
            size_mb: config.pet.starting_size_mb,
            personality,
            state: PetState::Baby,
            metabolism: Metabolism::new(config.pet.metabolism_rate),
            mood: Mood::Happy,
            hunger: 30.0,
            happiness: 80.0,
            alive: true,
            birth_time: Some(Instant::now()),
        })
    }
    
    /// Feed the pet (consume RAM)
    pub fn eat(&mut self, amount_mb: usize) -> Result<()> {
        if !self.alive {
            return Ok(());
        }
        
        self.size_mb += amount_mb;
        self.hunger = (self.hunger - (amount_mb as f32 * 2.0)).max(0.0);
        self.happiness = (self.happiness + (amount_mb as f32 * 0.5)).min(100.0);
        
        // Update state based on new size
        self.update_state();
        
        // Update mood
        self.mood = self.calculate_mood();
        
        Ok(())
    }
    
    /// Process metabolism (digest RAM over time)
    pub fn metabolize(&mut self, delta_time: f32) -> Result<()> {
        if !self.alive {
            return Ok(());
        }
        
        // Digest some RAM
        let digested = self.metabolism.process(self.size_mb, delta_time);
        if digested > 0 {
            self.size_mb = self.size_mb.saturating_sub(digested);
        }
        
        // Increase hunger over time
        self.hunger = (self.hunger + delta_time * 2.0).min(100.0);
        
        // Decrease happiness if too hungry
        if self.hunger > 70.0 {
            self.happiness = (self.happiness - delta_time * 3.0).max(0.0);
        }
        
        // Check if pet dies from starvation
        if self.hunger >= 100.0 {
            self.alive = false;
        }
        
        Ok(())
    }
    
    /// Update pet's mood based on stats
    pub fn update_mood(&mut self, _delta_time: f32) {
        self.mood = self.calculate_mood();
    }
    
    /// Calculate mood from current stats
    fn calculate_mood(&self) -> Mood {
        if !self.alive {
            return Mood::Dead;
        }
        
        match (self.hunger, self.happiness) {
            (h, _) if h > 90.0 => Mood::Starving,
            (h, _) if h > 70.0 => Mood::Hungry,
            (_, hp) if hp < 20.0 => Mood::Sad,
            (_, hp) if hp > 80.0 => Mood::Excited,
            (h, hp) if h < 30.0 && hp > 60.0 => Mood::Happy,
            _ => Mood::Content,
        }
    }
    
    /// Update state based on size
    fn update_state(&mut self) {
        self.state = match self.size_mb {
            0..=50 => PetState::Baby,
            51..=150 => PetState::Child,
            151..=300 => PetState::Teen,
            301..=500 => PetState::Adult,
            501..=1000 => PetState::Chubby,
            1001..=1500 => PetState::Fat,
            1501..=2000 => PetState::Huge,
            _ => PetState::Gigantic,
        };
    }
    
    /// Get pet's reaction to feeding
    pub fn get_reaction(&self) -> &str {
        self.personality.get_feeding_reaction(&self.mood)
    }
    
    /// Get favorite food size based on personality
    pub fn get_favorite_food_size(&self) -> usize {
        self.personality.get_favorite_food_size()
    }
    
    /// Boost happiness (for favorite food)
    pub fn boost_happiness(&mut self) {
        self.happiness = (self.happiness + 20.0).min(100.0);
    }
    
    /// Kill the pet
    pub fn kill(&mut self) {
        self.alive = false;
        self.mood = Mood::Dead;
    }
    
    // Getters
    pub fn get_size_mb(&self) -> usize { self.size_mb }
    pub fn get_state(&self) -> &PetState { &self.state }
    pub fn get_mood(&self) -> &Mood { &self.mood }
    pub fn get_hunger(&self) -> f32 { self.hunger }
    pub fn get_happiness(&self) -> f32 { self.happiness }
    pub fn is_dead(&self) -> bool { !self.alive }
    pub fn get_personality(&self) -> &Personality { &self.personality }
    
    /// Get ASCII art for current state
    pub fn get_ascii_art(&self) -> Vec<String> {
        self.state.get_ascii_art(&self.mood)
    }
    
    /// Get color for current mood
    pub fn get_mood_color(&self) -> (u8, u8, u8) {
        self.mood.get_color()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pet_creation() {
        let config = Config::default();
        let pet = Pet::new(&config);
        assert!(pet.is_ok());
    }
    
    #[test]
    fn test_pet_feeding() {
        let config = Config::default();
        let mut pet = Pet::new(&config).unwrap();
        let initial_size = pet.get_size_mb();
        pet.eat(50).unwrap();
        assert_eq!(pet.get_size_mb(), initial_size + 50);
    }
}