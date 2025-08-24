// src/main.rs
// RAM Eating Pet Simulator - Main Entry Point

use anyhow::Result;
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
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
    
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)?;
    
    // Create and run the game
    let result = run_game().await;
    
    // Cleanup terminal on exit
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show, terminal::Clear(ClearType::All))?;
    
    // Print goodbye message
    print_goodbye();
    
    result
}

async fn run_game() -> Result<()> {
    let mut game = Game::new()?;
    let mut tick_interval = interval(Duration::from_millis(100));
    
    loop {
        // Handle input
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                if !handle_input(&mut game, key_event).await? {
                    break; // Exit game
                }
            }
        }
        
        // Update game state
        tick_interval.tick().await;
        game.update().await?;
        
        // Render
        game.render()?;
        
        // Check if pet died
        if game.is_pet_dead() {
            game.show_death_screen()?;
            tokio::time::sleep(Duration::from_secs(3)).await;
            break;
        }
    }
    
    Ok(())
}

async fn handle_input(game: &mut Game, key: KeyEvent) -> Result<bool> {
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
            return Ok(false);
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
    println!("{}", "Press any key to start...".bright_white().blink());
    
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