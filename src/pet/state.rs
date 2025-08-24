// src/pet/state.rs
// RAM Eating Pet Simulator - Pet States and Visuals

use serde::{Deserialize, Serialize};
use crate::pet::personality::Mood;

/// Pet development states based on size
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PetState {
    Baby,      // 0-50 MB
    Child,     // 51-150 MB
    Teen,      // 151-300 MB
    Adult,     // 301-500 MB
    Chubby,    // 501-1000 MB
    Fat,       // 1001-1500 MB
    Huge,      // 1501-2000 MB
    Gigantic,  // 2000+ MB
}

impl PetState {
    /// Get ASCII art for the current state and mood
    pub fn get_ascii_art(&self, mood: &Mood) -> Vec<String> {
        match self {
            PetState::Baby => self.baby_art(mood),
            PetState::Child => self.child_art(mood),
            PetState::Teen => self.teen_art(mood),
            PetState::Adult => self.adult_art(mood),
            PetState::Chubby => self.chubby_art(mood),
            PetState::Fat => self.fat_art(mood),
            PetState::Huge => self.huge_art(mood),
            PetState::Gigantic => self.gigantic_art(mood),
        }
    }
    
    /// Baby state ASCII art
    fn baby_art(&self, mood: &Mood) -> Vec<String> {
        let eyes = match mood {
            Mood::Happy => "◕ ◕",
            Mood::Excited => "★ ★",
            Mood::Hungry => "◔ ◔",
            Mood::Starving => "✖ ✖",
            Mood::Sad => "╥ ╥",
            Mood::Dead => "✖ ✖",
            _ => "• •",
        };
        
        let mouth = match mood {
            Mood::Happy | Mood::Excited => "◡",
            Mood::Sad | Mood::Hungry => "╰",
            Mood::Starving => "〜",
            Mood::Dead => "_",
            _ => "◡",
        };
        
        vec![
            format!("  ╭─╮  "),
            format!(" │{}│ ", eyes),
            format!(" │ {} │ ", mouth),
            format!("  ╰─╯  "),
        ]
    }
    
    /// Child state ASCII art
    fn child_art(&self, mood: &Mood) -> Vec<String> {
        let eyes = match mood {
            Mood::Happy => "◉ ◉",
            Mood::Excited => "✧ ✧",
            Mood::Hungry => "◎ ◎",
            Mood::Starving => "⊗ ⊗",
            Mood::Sad => "┬ ┬",
            Mood::Dead => "✖ ✖",
            _ => "○ ○",
        };
        
        let mouth = match mood {
            Mood::Happy | Mood::Excited => "▽",
            Mood::Sad => "△",
            Mood::Hungry | Mood::Starving => "〰",
            Mood::Dead => "✖",
            _ => "━",
        };
        
        vec![
            format!("  ╭───╮  "),
            format!(" │ {} │ ", eyes),
            format!(" │  {}  │ ", mouth),
            format!(" ╰─┬─┬─╯ "),
            format!("   ╰─╯   "),
        ]
    }
    
    /// Teen state ASCII art
    fn teen_art(&self, mood: &Mood) -> Vec<String> {
        let eyes = match mood {
            Mood::Happy => "◕   ◕",
            Mood::Excited => "★   ★",
            Mood::Hungry => "◔   ◔",
            Mood::Starving => "☓   ☓",
            Mood::Sad => "╥   ╥",
            Mood::Dead => "✖   ✖",
            _ => "●   ●",
        };
        
        vec![
            format!("   ╭─────╮   "),
            format!("  │ {} │  ", eyes),
            format!("  │   {}   │  ", self.get_mouth(mood)),
            format!("  ╰──┬┬──╯  "),
            format!("     ╰╯     "),
        ]
    }
    
    /// Adult state ASCII art
    fn adult_art(&self, mood: &Mood) -> Vec<String> {
        vec![
            format!("    ╭───────╮    "),
            format!("   │  {}  {}  │   ", self.get_eye(mood), self.get_eye(mood)),
            format!("   │    {}    │   ", self.get_mouth(mood)),
            format!("   │         │   "),
            format!("   ╰────┬────╯   "),
            format!("        ╰╯       "),
        ]
    }
    
    /// Chubby state ASCII art
    fn chubby_art(&self, mood: &Mood) -> Vec<String> {
        vec![
            format!("     ╭─────────╮     "),
            format!("   ╱  {}     {}  ╲   ", self.get_eye(mood), self.get_eye(mood)),
            format!("  │      {}      │  ", self.get_mouth(mood)),
            format!("  │             │  "),
            format!("   ╲           ╱   "),
            format!("    ╰────┬────╯    "),
            format!("         ╰╯        "),
        ]
    }
    
    /// Fat state ASCII art
    fn fat_art(&self, mood: &Mood) -> Vec<String> {
        vec![
            format!("      ╭───────────╮      "),
            format!("    ╱   {}     {}   ╲    ", self.get_eye(mood), self.get_eye(mood)),
            format!("   │       {}       │   ", self.get_mouth(mood)),
            format!("  │                 │  "),
            format!("  │                 │  "),
            format!("   ╲               ╱   "),
            format!("    ╰──────┬──────╯    "),
            format!("           ╰╯          "),
        ]
    }
    
    /// Huge state ASCII art
    fn huge_art(&self, mood: &Mood) -> Vec<String> {
        vec![
            format!("       ╭─────────────╮       "),
            format!("     ╱    {}     {}    ╲     ", self.get_eye(mood), self.get_eye(mood)),
            format!("    │        {}        │    ", self.get_mouth(mood)),
            format!("   │                   │   "),
            format!("   │     C H O N K     │   "),
            format!("   │                   │   "),
            format!("    ╲                 ╱    "),
            format!("     ╰───────┬───────╯     "),
            format!("             ╰╯            "),
        ]
    }
    
    /// Gigantic state ASCII art
    fn gigantic_art(&self, mood: &Mood) -> Vec<String> {
        vec![
            format!("        ╭───────────────╮        "),
            format!("      ╱     {}     {}     ╲      ", self.get_eye(mood), self.get_eye(mood)),
            format!("     │         {}         │     ", self.get_mouth(mood)),
            format!("    │                     │    "),
            format!("    │   A B S O L U T E   │    "),
            format!("    │      U N I T        │    "),
            format!("    │                     │    "),
            format!("     ╲                   ╱     "),
            format!("      ╰────────┬────────╯      "),
            format!("               ╰╯              "),
        ]
    }
    
    /// Get eye character based on mood
    fn get_eye(&self, mood: &Mood) -> &str {
        match mood {
            Mood::Happy => "◕",
            Mood::Excited => "★",
            Mood::Hungry => "◔",
            Mood::Starving => "⊗",
            Mood::Sad => "╥",
            Mood::Angry => "▼",
            Mood::Sleepy => "━",
            Mood::Dead => "✖",
            _ => "●",
        }
    }
    
    /// Get mouth character based on mood
    fn get_mouth(&self, mood: &Mood) -> &str {
        match mood {
            Mood::Happy => "◡",
            Mood::Excited => "▽",
            Mood::Hungry => "〰",
            Mood::Starving => "╰",
            Mood::Sad => "╯",
            Mood::Angry => "▼",
            Mood::Sleepy => "━",
            Mood::Dead => "✖",
            _ => "─",
        }
    }
    
    /// Get state name
    pub fn name(&self) -> &str {
        match self {
            PetState::Baby => "Baby",
            PetState::Child => "Child",
            PetState::Teen => "Teen",
            PetState::Adult => "Adult",
            PetState::Chubby => "Chubby",
            PetState::Fat => "Fat",
            PetState::Huge => "Huge",
            PetState::Gigantic => "GIGANTIC",
        }
    }
    
    /// Get state description
    pub fn description(&self) -> &str {
        match self {
            PetState::Baby => "Just a tiny RAM nibbler",
            PetState::Child => "Growing and learning to eat properly",
            PetState::Teen => "Appetite increasing rapidly",
            PetState::Adult => "Fully grown and hungry",
            PetState::Chubby => "Well-fed and happy",
            PetState::Fat => "Perhaps a bit too well-fed",
            PetState::Huge => "An impressive specimen",
            PetState::Gigantic => "THE ABSOLUTE UNIT OF RAM CONSUMPTION",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_state_art_generation() {
        let state = PetState::Baby;
        let art = state.get_ascii_art(&Mood::Happy);
        assert!(!art.is_empty());
        assert!(art.len() >= 4);
    }
    
    #[test]
    fn test_all_states_have_art() {
        let states = vec![
            PetState::Baby,
            PetState::Child,
            PetState::Teen,
            PetState::Adult,
            PetState::Chubby,
            PetState::Fat,
            PetState::Huge,
            PetState::Gigantic,
        ];
        
        for state in states {
            let art = state.get_ascii_art(&Mood::Happy);
            assert!(!art.is_empty());
        }
    }
}