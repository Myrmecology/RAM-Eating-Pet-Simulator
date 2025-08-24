// src/lib.rs
// RAM Eating Pet Simulator - Library Root

//! # RAM Eating Pet Simulator
//! 
//! A virtual pet that literally consumes your computer's RAM to survive.
//! The more you feed it, the bigger it grows, and the more RAM it uses!
//! 
//! ## Features
//! - Real RAM consumption visible in Task Manager
//! - Unique personality system for each pet
//! - Colorful terminal graphics
//! - Save/load game state
//! - Multiple feeding modes

pub mod config;
pub mod game;
pub mod graphics;
pub mod pet;
pub mod system;

// Re-export commonly used types
pub use game::Game;
pub use pet::{Pet, Personality};
pub use system::memory::MemoryManager;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

/// Result type for the RAM Pet Simulator
pub type Result<T> = anyhow::Result<T>;

/// Initialize the RAM Pet Simulator
/// 
/// This function sets up any global state needed for the game
pub fn init() -> Result<()> {
    // Any global initialization can go here
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty());
    }
    
    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }
}