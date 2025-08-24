// src/config.rs
// RAM Eating Pet Simulator - Configuration

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Main configuration for the RAM Pet Simulator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub pet: PetConfig,
    pub graphics: GraphicsConfig,
    pub system: SystemConfig,
    pub game: GameConfig,
}

/// Pet-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetConfig {
    /// Starting size in MB
    pub starting_size_mb: usize,
    /// Maximum size in MB before "full"
    pub max_size_mb: usize,
    /// Rate at which pet digests RAM (MB per second)
    pub metabolism_rate: f32,
    /// How fast the pet gets hungry (hunger per second)
    pub hunger_rate: f32,
    /// Critical hunger level (pet starts dying)
    pub critical_hunger: f32,
    /// Happiness decay rate
    pub happiness_decay: f32,
}

/// Graphics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsConfig {
    /// Enable colored output
    pub use_colors: bool,
    /// Enable animations
    pub animations: bool,
    /// Frame rate (updates per second)
    pub fps: u32,
    /// Show debug info
    pub debug_mode: bool,
}

/// System configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Minimum free RAM to maintain (MB)
    pub min_free_ram_mb: usize,
    /// Warning threshold for low RAM (MB)
    pub warning_threshold_mb: usize,
    /// Enable system monitoring
    pub monitoring: bool,
    /// Update interval for system stats
    pub update_interval: Duration,
}

/// Game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Auto-save interval (seconds)
    pub autosave_interval: u64,
    /// Enable sound effects (terminal bell)
    pub sound_enabled: bool,
    /// Difficulty level (affects hunger rate)
    pub difficulty: Difficulty,
    /// Save file path
    pub save_path: String,
}

/// Game difficulty levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Nightmare,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            pet: PetConfig {
                starting_size_mb: 50,
                max_size_mb: 2048, // 2GB max
                metabolism_rate: 1.0, // 1 MB per second
                hunger_rate: 2.0,
                critical_hunger: 80.0,
                happiness_decay: 1.0,
            },
            graphics: GraphicsConfig {
                use_colors: true,
                animations: true,
                fps: 10,
                debug_mode: false,
            },
            system: SystemConfig {
                min_free_ram_mb: 1024, // Keep at least 1GB free
                warning_threshold_mb: 2048,
                monitoring: true,
                update_interval: Duration::from_secs(1),
            },
            game: GameConfig {
                autosave_interval: 60, // Auto-save every minute
                sound_enabled: true,
                difficulty: Difficulty::Normal,
                save_path: "saves/pet_save.json".to_string(),
            },
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = toml::de::from_str(&contents)?;
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save_to_file(&self, path: &str) -> anyhow::Result<()> {
        let contents = toml::ser::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
    
    /// Get config with difficulty adjustments
    pub fn with_difficulty(mut self, difficulty: Difficulty) -> Self {
        match difficulty {
            Difficulty::Easy => {
                self.pet.hunger_rate *= 0.5;
                self.pet.metabolism_rate *= 0.5;
            }
            Difficulty::Normal => {
                // Default values
            }
            Difficulty::Hard => {
                self.pet.hunger_rate *= 1.5;
                self.pet.metabolism_rate *= 1.5;
            }
            Difficulty::Nightmare => {
                self.pet.hunger_rate *= 2.0;
                self.pet.metabolism_rate *= 2.0;
                self.pet.critical_hunger = 60.0;
            }
        }
        self.game.difficulty = difficulty;
        self
    }
}

/// Feeding amounts in MB
pub mod feeding {
    pub const SNACK: usize = 10;
    pub const MEAL: usize = 50;
    pub const FEAST: usize = 100;
    pub const GORGE: usize = 500;
    
    /// Get feeding amount name
    pub fn get_feeding_name(amount: usize) -> &'static str {
        match amount {
            0..=15 => "Tiny Snack",
            16..=30 => "Snack",
            31..=75 => "Meal",
            76..=150 => "Big Meal",
            151..=300 => "Feast",
            301..=600 => "Banquet",
            _ => "MEGA GORGE",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.pet.starting_size_mb, 50);
        assert_eq!(config.game.difficulty, Difficulty::Normal);
    }
    
    #[test]
    fn test_difficulty_scaling() {
        let config = Config::default().with_difficulty(Difficulty::Hard);
        assert!(config.pet.hunger_rate > Config::default().pet.hunger_rate);
    }
}