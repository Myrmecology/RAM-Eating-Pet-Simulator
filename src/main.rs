// src/main.rs
// RAM Eating Pet Simulator - Main Entry Point

use anyhow::Result;
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::info;
use std::io::stdout;
use std::time::Duration;
use tokio::time::interval;

mod config;
mod game;
mod graphics;
mod pet;
mod system;

use crate::game::Game;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger for debugging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    info!("Starting RAM Eating Pet Simulator...");
    
    // Print welcome message
    print_welcome();
    
    // Setup terminal with alternate screen to prevent flickering
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(
        stdout,
        EnterAlternateScreen,  // Use alternate screen buffer
        terminal::Clear(ClearType::All),
        cursor::Hide
    )?;
    
    // Create and run the game
    let result = run_game().await;
    
    // Cleanup terminal on exit
    terminal::disable_raw_mode()?;
    execute!(
        stdout,
        LeaveAlternateScreen,  // Return to main screen
        cursor::Show
    )?;
    
    // Print goodbye message
    print_goodbye();
    
    result
}

async fn run_game() -> Result<()> {
    let mut game = Game::new()?;
    // Reduced tick rate to prevent flickering
    let mut tick_interval = interval(Duration::from_millis(200));
    let mut last_update = std::time::Instant::now();
    
    // Initial render
    game.render()?;
    
    loop {
        // Check if pet died FIRST before any rendering
        if game.is_pet_dead() {
            // Show death screen
            game.show_death_screen()?;
            
            // Wait for any key press to exit
            loop {
                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(_) = event::read()? {
                        return Ok(()); // Exit the game
                    }
                }
            }
        }
        
        // Handle input with shorter poll time for responsiveness
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                // Check if we should exit
                if !handle_input(&mut game, key_event).await? {
                    return Ok(()); // Exit game
                }
                
                // Always render after input to show changes immediately
                game.render()?;
            }
        }
        
        // Update game state periodically (but not if help is showing)
        let now = std::time::Instant::now();
        if now.duration_since(last_update) >= Duration::from_millis(200) {
            last_update = now;
            
            // Only update if help is not showing
            if !game.is_help_showing() {
                game.update().await?;
            }
            
            // Always render to keep display fresh
            game.render()?;
        }
        
        // Consume the tick to maintain timing
        tick_interval.tick().await;
    }
}

async fn handle_input(game: &mut Game, key: KeyEvent) -> Result<bool> {
    // If help is showing, only allow H to close it or Q to quit
    if game.is_help_showing() {
        match key.code {
            KeyCode::Char('h') | KeyCode::Char('H') => {
                game.toggle_help();
                return Ok(true);
            }
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                return Ok(false);
            }
            _ => {
                // Ignore other keys when help is showing
                return Ok(true);
            }
        }
    }
    
    // Normal input handling
    match key.code {
        KeyCode::Char(' ') => {
            // Space bar - feed the pet
            game.feed_pet(50).await?;
        }
        KeyCode::Char('f') | KeyCode::Char('F') => {
            // F - give favorite food
            game.feed_pet_favorite().await?;
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            // S - save game
            game.save_game()?;
        }
        KeyCode::Char('l') | KeyCode::Char('L') => {
            // L - load game
            game.load_game()?;
        }
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
            // Q or ESC - quit game
            return Ok(false);
        }
        KeyCode::Char('x') | KeyCode::Char('X') => {
            // X - emergency exit (pet dies)
            game.emergency_exit()?;
            // Don't exit immediately - let death screen show
        }
        KeyCode::Char('h') | KeyCode::Char('H') => {
            // H - show help
            game.toggle_help();
        }
        _ => {}
    }
    Ok(true)
}

fn print_welcome() {
    println!("{}", "═".repeat(50).bright_blue());
    println!("{}", "    RAM EATING PET SIMULATOR".bright_green().bold());
    println!("{}", "    A Virtual Pet That Eats Your Memory!".bright_cyan());
    println!("{}", "═".repeat(50).bright_blue());
    println!();
    println!("{}", "Your pet will actually consume RAM to survive!".yellow());
    println!("{}", "Watch your Task Manager to see it grow!".yellow());
    println!();
    println!("{}", "Press any key to start...".bright_white());
    
    // Wait for key press
    let _ = std::io::stdin().read_line(&mut String::new());
}

fn print_goodbye() {
    println!("{}", "═".repeat(50).bright_blue());
    println!("{}", "Thanks for playing RAM Eating Pet Simulator!".bright_green());
    println!("{}", "Your pet will miss you! 💔".bright_red());
    println!("{}", "═".repeat(50).bright_blue());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_main_exists() {
        // Basic test to ensure main compiles
        assert!(true);
    }
}