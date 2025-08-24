// src/graphics/renderer.rs
// RAM Eating Pet Simulator - Main Rendering Engine

use anyhow::Result;
use colored::*;
use crossterm::{cursor, execute};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crate::pet::Pet;
use crate::system::monitor::SystemMonitor;
use super::{ROUNDED_BORDER, DOUBLE_BORDER, create_meter, create_box, format_duration, center_text};

/// Main renderer for the game
pub struct Renderer {
    use_colors: bool,
}

impl Renderer {
    /// Create a new renderer
    pub fn new(use_colors: bool) -> Self {
        Renderer { use_colors }
    }
    
    /// Draw the game header
    pub fn draw_header(&self, pet: &Pet) -> Result<()> {
        println!("{}", "═".repeat(60).bright_blue());
        println!("{} {} {}",
            "🎮".to_string(),
            center_text("RAM EATING PET SIMULATOR", 56).bright_green().bold(),
            "🎮".to_string()
        );
        println!("{}", center_text(&format!("Pet: {}", pet.name), 60).bright_cyan());
        println!("{}", "═".repeat(60).bright_blue());
        println!();
        Ok(())
    }
    
    /// Draw the pet
    pub fn draw_pet(&self, pet: &Pet) -> Result<()> {
        let ascii_art = pet.get_ascii_art();
        let mood_color = pet.get_mood_color();
        let color = Color::TrueColor { r: mood_color.0, g: mood_color.1, b: mood_color.2 };
        
        // Draw pet in a box
        let pet_box = create_box(ascii_art, &ROUNDED_BORDER, color);
        for line in pet_box {
            println!("{}", center_text(&line, 60));
        }
        
        // Pet status
        println!();
        println!("{}", center_text(
            &format!("State: {} | Mood: {}", 
                pet.get_state().name(),
                pet.get_mood().name()
            ), 60
        ).color(color));
        
        // Pet says something based on personality
        if let Some(comment) = self.get_pet_comment(pet) {
            println!();
            println!("{}", center_text(&format!("\"{}\"", comment), 60).italic().bright_white());
        }
        
        println!();
        Ok(())
    }
    
    /// Draw stats panel
    pub fn draw_stats(&self, pet: &Pet, monitor: &SystemMonitor, total_eaten: usize, play_time: Duration) -> Result<()> {
        println!("{}", "┌─ Stats ─────────────────────────────────────────────┐".bright_blue());
        
        // Pet stats
        println!("│ {} │", format!("Pet Size: {} MB", pet.get_size_mb()).bright_green());
        
        // Hunger meter with color based on level
        let hunger_color = match pet.get_hunger() {
            h if h > 80.0 => Color::Red,
            h if h > 60.0 => Color::Yellow,
            _ => Color::Green,
        };
        println!("│ {} │", create_meter("Hunger  ", pet.get_hunger(), 100.0, hunger_color));
        
        // Happiness meter
        let happiness_color = match pet.get_happiness() {
            h if h > 70.0 => Color::Green,
            h if h > 40.0 => Color::Yellow,
            _ => Color::Red,
        };
        println!("│ {} │", create_meter("Happiness", pet.get_happiness(), 100.0, happiness_color));
        
        println!("│{}│", " ".repeat(57));
        
        // System stats
        let total_ram = monitor.get_total_ram_mb();
        let used_ram = monitor.get_used_ram_mb();
        let free_ram = monitor.get_free_ram_mb();
        
        println!("│ {} │", format!("System RAM: {} / {} MB", 
            used_ram.to_string().bright_red(),
            total_ram.to_string().bright_green()
        ));
        
        println!("│ {} │", create_meter("RAM Usage", used_ram as f32, total_ram as f32, Color::Cyan));
        
        println!("│{}│", " ".repeat(57));
        
        // Game stats
        println!("│ {} │", format!("Total Eaten: {} MB", total_eaten).bright_yellow());
        println!("│ {} │", format!("Play Time: {}", format_duration(play_time)).bright_cyan());
        
        println!("{}", "└──────────────────────────────────────────────────────┘".bright_blue());
        println!();
        
        Ok(())
    }
    
    /// Draw messages
    pub fn draw_messages(&self, messages: &[(String, Instant, ColoredString)]) -> Result<()> {
        if !messages.is_empty() {
            println!("{}", "┌─ Messages ──────────────────────────────────────────┐".yellow());
            for (msg, _, icon) in messages.iter().rev().take(3) {
                println!("│ {} {} │", icon, msg.bright_white());
            }
            println!("{}", "└──────────────────────────────────────────────────────┘".yellow());
            println!();
        }
        Ok(())
    }
    
    /// Draw controls
    pub fn draw_controls(&self) -> Result<()> {
        println!("{}", "─".repeat(60).bright_black());
        println!("{}", "Controls:".bright_white().bold());
        println!("  {} Feed (50 MB)    {} Favorite Food    {} Save Game",
            "[SPACE]".bright_green(),
            "[F]".bright_cyan(),
            "[S]".bright_yellow()
        );
        println!("  {} Load Game       {} Help            {} Quit",
            "[L]".bright_yellow(),
            "[H]".bright_blue(),
            "[Q/ESC]".bright_red()
        );
        println!("{}", "─".repeat(60).bright_black());
        Ok(())
    }
    
    /// Draw help screen
    pub fn draw_help(&self) -> Result<()> {
        let help_text = vec![
            "╔════════════════════ HELP ════════════════════╗".bright_cyan().to_string(),
            "║                                               ║".bright_cyan().to_string(),
            format!("║ {} ║", "Your pet eats RAM to survive!".bright_white()),
            format!("║ {} ║", "Feed it regularly or it will die.".bright_white()),
            "║                                               ║".bright_cyan().to_string(),
            format!("║ {} ║", "Feeding Guide:".bright_yellow()),
            format!("║ {} ║", "• Small snacks: Keep hunger at bay".bright_white()),
            format!("║ {} ║", "• Big meals: Make pet happy".bright_white()),
            format!("║ {} ║", "• Favorite food: Maximum happiness!".bright_white()),
            "║                                               ║".bright_cyan().to_string(),
            format!("║ {} ║", "Watch your system RAM!".bright_red()),
            format!("║ {} ║", "Too much feeding = system crash".bright_red()),
            "║                                               ║".bright_cyan().to_string(),
            format!("║ {} ║", "Press [H] to close help".italic()),
            "╚═══════════════════════════════════════════════╝".bright_cyan().to_string(),
        ];
        
        for line in help_text {
            println!("{}", center_text(&line, 60));
        }
        Ok(())
    }
    
    /// Draw death screen
    pub fn draw_death_screen(&self, pet: &Pet, total_eaten: usize, play_time: Duration, max_size: usize) -> Result<()> {
        execute!(stdout(), cursor::MoveTo(0, 0))?;
        
        println!();
        println!();
        println!("{}", "════════════════════════════════════════".bright_red());
        println!("{}", "           YOUR PET HAS DIED".bright_red().bold());
        println!("{}", "════════════════════════════════════════".bright_red());
        println!();
        
        // Death ASCII art
        let death_art = vec![
            "        ✖     ✖".bright_red().to_string(),
            "          ___".bright_red().to_string(),
            "         /   \\".bright_red().to_string(),
            "        | RIP |".bright_white().to_string(),
            "        |     |".bright_white().to_string(),
            "     ___|_____|___".bright_black().to_string(),
        ];
        
        for line in death_art {
            println!("{}", center_text(&line, 60));
        }
        
        println!();
        println!("{}", center_text(&format!("{} lived a good life", pet.name), 60).bright_cyan());
        println!();
        
        // Final stats
        println!("{}", "Final Statistics:".bright_yellow().bold());
        println!("{}", format!("  Total RAM Consumed: {} MB", total_eaten).bright_white());
        println!("{}", format!("  Maximum Size Reached: {} MB", max_size).bright_white());
        println!("{}", format!("  Survived For: {}", format_duration(play_time)).bright_white());
        println!();
        
        // Cause of death
        let cause = if pet.get_hunger() >= 100.0 {
            "Died of starvation 💀"
        } else {
            "Terminated by user 🔌"
        };
        println!("{}", center_text(cause, 60).bright_red());
        
        println!();
        println!("{}", "════════════════════════════════════════".bright_red());
        println!("{}", center_text("Press any key to exit...", 60).blink());
        
        Ok(())
    }
    
    /// Get a random comment from the pet
    fn get_pet_comment(&self, pet: &Pet) -> Option<String> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Only show comments sometimes
        if rng.gen_bool(0.3) {
            let hunger = pet.get_hunger();
            let happiness = pet.get_happiness();
            
            let comment = if hunger > 80.0 {
                vec!["I'm starving!", "FEED ME!", "So... hungry..."]
            } else if hunger > 60.0 {
                vec!["Getting hungry...", "Food would be nice", "My stomach is rumbling"]
            } else if happiness > 80.0 {
                vec!["Life is good!", "I love you!", "Best day ever!"]
            } else if happiness < 30.0 {
                vec!["I'm sad...", "Why don't you love me?", "This isn't fun anymore"]
            } else {
                vec!["*yawn*", "What's that process over there?", "RAM tastes good", "Hi!"]
            };
            
            Some(comment[rng.gen_range(0..comment.len())].to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    
    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new(true);
        assert!(renderer.use_colors);
    }
}