// src/graphics/renderer.rs
// RAM Eating Pet Simulator - Main Rendering Engine

use anyhow::Result;
use colored::*;
use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crate::pet::Pet;
use crate::system::monitor::SystemMonitor;
use super::{ROUNDED_BORDER, create_meter, create_box, format_duration, center_text};

/// Main renderer for the game with fixed layout
pub struct Renderer {
    use_colors: bool,
    last_render: Instant,
    frame_count: u64,
    last_comment: Option<String>,  // Store last comment to prevent jumping
}

impl Renderer {
    /// Create a new renderer
    pub fn new(use_colors: bool) -> Self {
        Renderer { 
            use_colors,
            last_render: Instant::now(),
            frame_count: 0,
            last_comment: None,
        }
    }
    
    /// Clear entire screen and reset
    pub fn full_clear(&self) -> Result<()> {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        Ok(())
    }
    
    /// Draw complete frame with fixed positioning
    pub fn draw_frame(&mut self, 
                      pet: &Pet, 
                      monitor: &SystemMonitor,
                      messages: &[(String, Instant, ColoredString)],
                      total_eaten: usize,
                      play_time: Duration,
                      show_help: bool) -> Result<()> {
        
        let mut stdout = stdout();
        
        // Move to top-left
        execute!(stdout, cursor::MoveTo(0, 0))?;
        
        // Line 1-4: Header (always 4 lines)
        self.draw_header_fixed(pet)?;
        
        // Line 5-15: Pet (always 11 lines including state/mood)
        self.draw_pet_fixed(pet)?;
        
        // Line 16-17: Pet comment (always 2 lines, even if empty)
        self.draw_comment_fixed(pet)?;
        
        // Line 18-29: Stats (always 12 lines)
        self.draw_stats_fixed(pet, monitor, total_eaten, play_time)?;
        
        // Line 30-34: Messages (always 5 lines, even if no messages)
        self.draw_messages_fixed(messages)?;
        
        // Line 35-40: Controls or Help (always 6 lines)
        if show_help {
            self.draw_help_fixed()?;
        } else {
            self.draw_controls_fixed()?;
        }
        
        // Ensure everything is drawn
        stdout.flush()?;
        Ok(())
    }
    
    /// Draw the game header - Fixed 4 lines
    fn draw_header_fixed(&self, pet: &Pet) -> Result<()> {
        println!("{:60}", "═".repeat(60).bright_blue());
        println!("{:^60}", 
            format!("🎮 {} 🎮", 
                "RAM EATING PET SIMULATOR".bright_green().bold()
            )
        );
        println!("{:^60}", format!("Pet: {}", pet.name).bright_cyan());
        println!("{:60}", "═".repeat(60).bright_blue());
        Ok(())
    }
    
    /// Draw the pet - Fixed 11 lines
    fn draw_pet_fixed(&self, pet: &Pet) -> Result<()> {
        let ascii_art = pet.get_ascii_art();
        let mood_color = pet.get_mood_color();
        let color = Color::TrueColor { r: mood_color.0, g: mood_color.1, b: mood_color.2 };
        
        // Ensure we always print exactly 8 lines for the pet box
        let pet_box = create_box(ascii_art.clone(), &ROUNDED_BORDER, color);
        
        // Pad to exactly 8 lines
        let mut lines_printed = 0;
        for line in pet_box.iter().take(8) {
            println!("{:^60}", line);
            lines_printed += 1;
        }
        // Fill remaining lines if pet art is smaller
        for _ in lines_printed..8 {
            println!("{:60}", " ");
        }
        
        // Line 9: Empty
        println!();
        
        // Line 10: State and mood
        println!("{:^60}", 
            format!("State: {} | Mood: {}", 
                pet.get_state().name(),
                pet.get_mood().name()
            ).color(color)
        );
        
        // Line 11: Empty
        println!();
        
        Ok(())
    }
    
    /// Draw pet comment - Fixed 2 lines
    fn draw_comment_fixed(&mut self, pet: &Pet) -> Result<()> {
        // Update comment occasionally
        if rand::random::<f32>() < 0.05 {  // 5% chance to change comment
            self.last_comment = self.get_pet_comment(pet);
        }
        
        if let Some(ref comment) = self.last_comment {
            println!("{:^60}", format!("\"{}\"", comment).italic().bright_white());
        } else {
            println!("{:60}", " ");  // Empty line to maintain spacing
        }
        println!();  // Always have blank line after comment
        
        Ok(())
    }
    
    /// Draw stats panel - Fixed 12 lines
    fn draw_stats_fixed(&mut self, pet: &Pet, monitor: &SystemMonitor, total_eaten: usize, play_time: Duration) -> Result<()> {
        println!("{:60}", "┌─ Stats ─────────────────────────────────────────────┐".bright_blue());
        println!("{:60}", format!("│ Pet Size: {:44} │", format!("{} MB", pet.get_size_mb()).bright_green()));
        
        // Hunger meter
        let hunger_color = match pet.get_hunger() {
            h if h > 80.0 => Color::Red,
            h if h > 60.0 => Color::Yellow,
            _ => Color::Green,
        };
        let hunger_bar = create_meter("Hunger  ", pet.get_hunger(), 100.0, hunger_color);
        println!("│ {:54} │", hunger_bar);
        
        // Happiness meter
        let happiness_color = match pet.get_happiness() {
            h if h > 70.0 => Color::Green,
            h if h > 40.0 => Color::Yellow,
            _ => Color::Red,
        };
        let happiness_bar = create_meter("Happiness", pet.get_happiness(), 100.0, happiness_color);
        println!("│ {:54} │", happiness_bar);
        
        println!("│{:56}│", " ");
        
        // System stats
        let total_ram = monitor.get_total_ram_mb();
        let used_ram = monitor.get_used_ram_mb();
        
        println!("{:60}", format!("│ System RAM: {} / {} MB{:>26} │", 
            used_ram.to_string().bright_red(),
            total_ram.to_string().bright_green(),
            " "
        ));
        
        let ram_bar = create_meter("RAM Usage", used_ram as f32, total_ram as f32, Color::Cyan);
        println!("│ {:54} │", ram_bar);
        
        println!("│{:56}│", " ");
        
        // Game stats
        println!("{:60}", format!("│ Total Eaten: {:42} │", format!("{} MB", total_eaten).bright_yellow()));
        println!("{:60}", format!("│ Play Time: {:44} │", format_duration(play_time).bright_cyan()));
        
        println!("{:60}", "└──────────────────────────────────────────────────────┘".bright_blue());
        Ok(())
    }
    
    /// Draw messages - Fixed 5 lines
    fn draw_messages_fixed(&self, messages: &[(String, Instant, ColoredString)]) -> Result<()> {
        if !messages.is_empty() {
            println!("{:60}", "┌─ Messages ──────────────────────────────────────────┐".yellow());
            
            let mut lines_printed = 0;
            for (msg, _, icon) in messages.iter().rev().take(3) {
                let msg_str = format!("{} {}", icon, msg.bright_white());
                println!("│ {:54} │", msg_str);
                lines_printed += 1;
            }
            
            // Pad to always have 3 message lines
            for _ in lines_printed..3 {
                println!("│{:56}│", " ");
            }
            
            println!("{:60}", "└──────────────────────────────────────────────────────┘".yellow());
        } else {
            // Print 5 empty lines when no messages
            for _ in 0..5 {
                println!("{:60}", " ");
            }
        }
        Ok(())
    }
    
    /// Draw controls - Fixed 6 lines
    fn draw_controls_fixed(&self) -> Result<()> {
        println!("{:60}", "─".repeat(60).bright_black());
        println!("{:60}", "Controls:".bright_white().bold());
        println!("{:60}", format!("  {} Feed (50 MB)    {} Favorite Food    {} Save",
            "[SPACE]".bright_green(),
            "[F]".bright_cyan(),
            "[S]".bright_yellow()
        ));
        println!("{:60}", format!("  {} Load Game       {} Help            {} Quit",
            "[L]".bright_yellow(),
            "[H]".bright_blue(),
            "[Q/ESC]".bright_red()
        ));
        println!("{:60}", "─".repeat(60).bright_black());
        println!();  // Bottom padding
        Ok(())
    }
    
    /// Draw help - Fixed 6 lines (condensed)
    fn draw_help_fixed(&self) -> Result<()> {
        println!("{:60}", "╔════════════ HELP ═══════════════╗".bright_cyan());
        println!("{:60}", "║ Feed regularly or pet dies!     ║".bright_yellow());
        println!("{:60}", "║ Watch system RAM usage!         ║".bright_red());
        println!("{:60}", "║ Favorite food = Max happiness   ║".bright_green());
        println!("{:60}", "║ Press [H] to close help         ║".bright_white());
        println!("{:60}", "╚═════════════════════════════════╝".bright_cyan());
        Ok(())
    }
    
    // Keep all the original methods but updated
    
    pub fn draw_header(&self, pet: &Pet) -> Result<()> {
        self.draw_header_fixed(pet)
    }
    
    pub fn draw_pet(&self, pet: &Pet) -> Result<()> {
        self.draw_pet_fixed(pet)
    }
    
    pub fn draw_stats(&mut self, pet: &Pet, monitor: &SystemMonitor, total_eaten: usize, play_time: Duration) -> Result<()> {
        self.draw_stats_fixed(pet, monitor, total_eaten, play_time)
    }
    
    pub fn draw_messages(&self, messages: &[(String, Instant, ColoredString)]) -> Result<()> {
        self.draw_messages_fixed(messages)
    }
    
    pub fn draw_controls(&self) -> Result<()> {
        self.draw_controls_fixed()
    }
    
    pub fn draw_help(&self) -> Result<()> {
        self.draw_help_fixed()
    }
    
    /// Draw death screen
    pub fn draw_death_screen(&self, pet: &Pet, total_eaten: usize, play_time: Duration, max_size: usize) -> Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0))?;
        
        println!();
        println!();
        println!("{:^60}", "════════════════════════════════════════".bright_red());
        println!("{:^60}", "YOUR PET HAS DIED".bright_red().bold());
        println!("{:^60}", "════════════════════════════════════════".bright_red());
        println!();
        
        // Death ASCII art
        println!("{:^60}", "✖     ✖".bright_red());
        println!("{:^60}", "  ___".bright_red());
        println!("{:^60}", " /   \\".bright_red());
        println!("{:^60}", "| RIP |".bright_white());
        println!("{:^60}", "|     |".bright_white());
        println!("{:^60}", "___|_____|___".bright_black());
        
        println!();
        println!("{:^60}", format!("{} lived a good life", pet.name).bright_cyan());
        println!();
        
        println!("{:^60}", "Final Statistics:".bright_yellow().bold());
        println!("{:^60}", format!("Total RAM Consumed: {} MB", total_eaten).bright_white());
        println!("{:^60}", format!("Maximum Size Reached: {} MB", max_size).bright_white());
        println!("{:^60}", format!("Survived For: {}", format_duration(play_time)).bright_white());
        println!();
        
        let cause = if pet.get_hunger() >= 100.0 {
            "Died of starvation 💀"
        } else {
            "Terminated by user 🔌"
        };
        println!("{:^60}", cause.bright_red());
        
        println!();
        println!("{:^60}", "════════════════════════════════════════".bright_red());
        println!("{:^60}", "Press any key to exit...");
        
        Ok(())
    }
    
    /// Get a random comment from the pet (less frequently)
    fn get_pet_comment(&self, pet: &Pet) -> Option<String> {
        let hunger = pet.get_hunger();
        let happiness = pet.get_happiness();
        
        let comment = if hunger > 80.0 {
            vec!["I'm starving!", "FEED ME!", "So... hungry..."]
        } else if hunger > 60.0 {
            vec!["Getting hungry...", "Food would be nice", "Rumble rumble"]
        } else if happiness > 80.0 {
            vec!["Life is good!", "I love you!", "Best day ever!"]
        } else if happiness < 30.0 {
            vec!["I'm sad...", "This isn't fun", "Sigh..."]
        } else {
            vec!["*yawn*", "Hmm...", "RAM tastes good", "Hi there!"]
        };
        
        Some(comment[rand::random::<usize>() % comment.len()].to_string())
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