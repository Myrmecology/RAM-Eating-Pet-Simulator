// src/game.rs
// RAM Eating Pet Simulator - Main Game Logic

use anyhow::Result;
use colored::*;
use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crate::config::{Config, feeding};
use crate::graphics::renderer::Renderer;
use crate::pet::Pet;
use crate::system::memory::MemoryManager;
use crate::system::monitor::SystemMonitor;

/// Main game state
pub struct Game {
    /// The pet instance
    pet: Pet,
    /// Memory manager for RAM allocation
    memory_manager: MemoryManager,
    /// System monitor for RAM stats
    system_monitor: SystemMonitor,
    /// Renderer for drawing
    renderer: Renderer,
    /// Game configuration
    config: Config,
    /// Last update time
    last_update: Instant,
    /// Show help overlay
    show_help: bool,
    /// Game messages to display
    messages: Vec<(String, Instant, ColoredString)>,
    /// Game score/stats
    stats: GameStats,
}

/// Game statistics
struct GameStats {
    total_mb_eaten: usize,
    feeding_count: usize,
    max_size_reached: usize,
    play_time: Duration,
    session_start: Instant,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            total_mb_eaten: 0,
            feeding_count: 0,
            max_size_reached: 0,
            play_time: Duration::from_secs(0),
            session_start: Instant::now(),
        }
    }
}

impl Game {
    /// Create a new game instance
    pub fn new() -> Result<Self> {
        let config = Config::default();
        let pet = Pet::new(&config)?;
        let memory_manager = MemoryManager::new(config.system.min_free_ram_mb);
        let system_monitor = SystemMonitor::new();
        let renderer = Renderer::new(config.graphics.use_colors);
        
        Ok(Game {
            pet,
            memory_manager,
            system_monitor,
            renderer,
            config,
            last_update: Instant::now(),
            show_help: false,
            messages: Vec::new(),
            stats: GameStats {
                session_start: Instant::now(),
                ..Default::default()
            },
        })
    }
    
    /// Update game state
    pub async fn update(&mut self) -> Result<()> {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;
        
        // Update play time
        self.stats.play_time = now.duration_since(self.stats.session_start);
        
        // Update pet metabolism
        self.pet.metabolize(delta)?;
        
        // Update pet mood based on hunger
        self.pet.update_mood(delta);
        
        // Update system monitor
        self.system_monitor.update()?;
        
        // Check for critical RAM levels
        self.check_ram_levels()?;
        
        // Clean up old messages (keep messages for 5 seconds instead of 3)
        self.messages.retain(|(_, time, _)| {
            now.duration_since(*time).as_secs() < 5
        });
        
        // Update max size stat
        if self.pet.get_size_mb() > self.stats.max_size_reached {
            self.stats.max_size_reached = self.pet.get_size_mb();
        }
        
        Ok(())
    }
    
    /// Feed the pet with specified amount of RAM
    pub async fn feed_pet(&mut self, amount_mb: usize) -> Result<()> {
        // Check if we have enough free RAM
        let free_ram = self.system_monitor.get_free_ram_mb();
        if free_ram < amount_mb + self.config.system.min_free_ram_mb {
            self.add_message(
                "Not enough free RAM! Close some programs first!".to_string(),
                "❌".to_string().red(),
            );
            return Ok(());
        }
        
        // Actually allocate the memory
        self.memory_manager.allocate(amount_mb)?;
        
        // Feed the pet
        self.pet.eat(amount_mb)?;
        
        // Update stats
        self.stats.total_mb_eaten += amount_mb;
        self.stats.feeding_count += 1;
        
        // Add feeding message
        let food_name = feeding::get_feeding_name(amount_mb);
        self.add_message(
            format!("Fed {} ({} MB)", food_name, amount_mb),
            format!("{}!", self.pet.get_reaction()).green(),
        );
        
        // Sound effect
        if self.config.game.sound_enabled {
            print!("\x07"); // Terminal bell
            let _ = stdout().flush();
        }
        
        Ok(())
    }
    
    /// Feed pet its favorite food
    pub async fn feed_pet_favorite(&mut self) -> Result<()> {
        let favorite_amount = self.pet.get_favorite_food_size();
        
        // Special message for favorite food
        self.add_message(
            format!("Favorite food! ({} MB)", favorite_amount),
            "✨ PURE JOY ✨".bright_green(),
        );
        
        self.feed_pet(favorite_amount).await?;
        self.pet.boost_happiness();
        
        Ok(())
    }
    
    /// Render the game screen using the fixed frame renderer
    pub fn render(&mut self) -> Result<()> {
        // Use the new fixed frame renderer for stable display
        self.renderer.draw_frame(
            &self.pet,
            &self.system_monitor,
            &self.messages,
            self.stats.total_mb_eaten,
            self.stats.play_time,
            self.show_help
        )?;
        
        stdout().flush()?;
        Ok(())
    }
    
    /// Check if pet has died
    pub fn is_pet_dead(&self) -> bool {
        self.pet.is_dead()
    }
    
    /// Show death screen
    pub fn show_death_screen(&mut self) -> Result<()> {
        self.renderer.draw_death_screen(
            &self.pet,
            self.stats.total_mb_eaten,
            self.stats.play_time,
            self.stats.max_size_reached,
        )?;
        Ok(())
    }
    
    /// Save game state
    pub fn save_game(&mut self) -> Result<()> {
        let save_data = SaveData {
            pet: self.pet.clone(),
            total_mb_eaten: self.stats.total_mb_eaten,
            feeding_count: self.stats.feeding_count,
            max_size_reached: self.stats.max_size_reached,
        };
        
        let json = serde_json::to_string_pretty(&save_data)?;
        std::fs::create_dir_all("saves")?;
        std::fs::write(&self.config.game.save_path, json)?;
        
        self.add_message(
            "Game saved successfully!".to_string(),
            "💾".to_string().bright_cyan(),
        );
        
        Ok(())
    }
    
    /// Load game state
    pub fn load_game(&mut self) -> Result<()> {
        if !std::path::Path::new(&self.config.game.save_path).exists() {
            self.add_message(
                "No save file found!".to_string(),
                "❌".to_string().bright_red(),
            );
            return Ok(());
        }
        
        let json = std::fs::read_to_string(&self.config.game.save_path)?;
        let save_data: SaveData = serde_json::from_str(&json)?;
        
        self.pet = save_data.pet;
        self.stats.total_mb_eaten = save_data.total_mb_eaten;
        self.stats.feeding_count = save_data.feeding_count;
        self.stats.max_size_reached = save_data.max_size_reached;
        
        // Reallocate memory to match pet size
        self.memory_manager.clear();
        self.memory_manager.allocate(self.pet.get_size_mb())?;
        
        self.add_message(
            "Game loaded successfully!".to_string(),
            "📂".to_string().bright_cyan(),
        );
        
        Ok(())
    }
    
    /// Emergency exit (pet dies immediately)
    pub fn emergency_exit(&mut self) -> Result<()> {
        self.pet.kill();
        self.add_message(
            "EMERGENCY EXIT ACTIVATED!".to_string(),
            "☠️".to_string().bright_red(),
        );
        Ok(())
    }
    
    /// Toggle help display
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Check if help is currently showing     
pub fn is_help_showing(&self) -> bool {    
    self.show_help                         
}                                           
    
    /// Add a message to display
    fn add_message(&mut self, text: String, icon: ColoredString) {
        self.messages.push((text, Instant::now(), icon));
        
        // Keep only last 3 messages for cleaner display
        if self.messages.len() > 3 {
            self.messages.remove(0);
        }
    }
    
    /// Check RAM levels and warn if necessary
    fn check_ram_levels(&mut self) -> Result<()> {
        let free_ram = self.system_monitor.get_free_ram_mb();
        
        // Only warn every so often to avoid message spam
        static mut LAST_WARNING: Option<Instant> = None;
        let now = Instant::now();
        
        unsafe {
            if let Some(last) = LAST_WARNING {
                if now.duration_since(last).as_secs() < 10 {
                    return Ok(()); // Don't warn too frequently
                }
            }
        }
        
        if free_ram < self.config.system.warning_threshold_mb {
            if free_ram < self.config.system.min_free_ram_mb {
                self.add_message(
                    "CRITICAL: RAM dangerously low!".to_string(),
                    "⚠️".to_string().bright_red(),
                );
            } else if free_ram < self.config.system.warning_threshold_mb / 2 {
                self.add_message(
                    format!("Warning: Only {} MB RAM free", free_ram),
                    "⚠️".to_string().yellow(),
                );
            }
            
            unsafe {
                LAST_WARNING = Some(now);
            }
        }
        
        Ok(())
    }
}

/// Save data structure
#[derive(serde::Serialize, serde::Deserialize)]
struct SaveData {
    pet: Pet,
    total_mb_eaten: usize,
    feeding_count: usize,
    max_size_reached: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_game_creation() {
        let game = Game::new();
        assert!(game.is_ok());
    }
    
    #[tokio::test]
    async fn test_feed_pet() {
        let mut game = Game::new().unwrap();
        let initial_size = game.pet.get_size_mb();
        
        // Try to feed pet (may fail if not enough RAM)
        let _ = game.feed_pet(10).await;
        
        // Size should either increase or stay same (if feeding failed)
        assert!(game.pet.get_size_mb() >= initial_size);
    }
}