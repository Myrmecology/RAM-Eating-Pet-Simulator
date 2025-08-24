// src/graphics/animations.rs
// RAM Eating Pet Simulator - Animation System

use colored::*;
use std::time::{Duration, Instant};

/// Animation frame data
#[derive(Debug, Clone)]
pub struct AnimationFrame {
    pub content: Vec<String>,
    pub duration: Duration,
}

/// Animation sequence
#[derive(Debug, Clone)]
pub struct Animation {
    frames: Vec<AnimationFrame>,
    current_frame: usize,
    last_update: Instant,
    looping: bool,
}

impl Animation {
    /// Create a new animation
    pub fn new(frames: Vec<AnimationFrame>, looping: bool) -> Self {
        Animation {
            frames,
            current_frame: 0,
            last_update: Instant::now(),
            looping,
        }
    }
    
    /// Update animation and return current frame
    pub fn update(&mut self) -> Option<&Vec<String>> {
        if self.frames.is_empty() {
            return None;
        }
        
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        
        if elapsed >= self.frames[self.current_frame].duration {
            self.last_update = now;
            self.current_frame += 1;
            
            if self.current_frame >= self.frames.len() {
                if self.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.frames.len() - 1;
                    return None;
                }
            }
        }
        
        Some(&self.frames[self.current_frame].content)
    }
    
    /// Reset animation to beginning
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.last_update = Instant::now();
    }
}

/// Create eating animation
pub fn create_eating_animation() -> Animation {
    let frames = vec![
        AnimationFrame {
            content: vec![
                "  ╭───╮  ".to_string(),
                " │ ◕ ◕ │ ".to_string(),
                " │  ○  │ ".to_string(),
                "  ╰───╯  ".to_string(),
            ],
            duration: Duration::from_millis(200),
        },
        AnimationFrame {
            content: vec![
                "  ╭───╮  ".to_string(),
                " │ ◕ ◕ │ ".to_string(),
                " │  O  │ ".to_string(),
                "  ╰───╯  ".to_string(),
            ],
            duration: Duration::from_millis(200),
        },
        AnimationFrame {
            content: vec![
                "  ╭───╮  ".to_string(),
                " │ > < │ ".to_string(),
                " │  ~  │ *munch*".to_string(),
                "  ╰───╯  ".to_string(),
            ],
            duration: Duration::from_millis(300),
        },
        AnimationFrame {
            content: vec![
                "  ╭───╮  ".to_string(),
                " │ ◕ ◕ │ ".to_string(),
                " │  ◡  │ ".to_string(),
                "  ╰───╯  ".to_string(),
            ],
            duration: Duration::from_millis(200),
        },
    ];
    
    Animation::new(frames, false)
}

/// Create growing animation
pub fn create_growth_animation() -> Animation {
    let frames = vec![
        AnimationFrame {
            content: vec![
                "  ╭─╮  ".to_string(),
                " │•.•│ ".to_string(),
                "  ╰─╯  ".to_string(),
            ],
            duration: Duration::from_millis(300),
        },
        AnimationFrame {
            content: vec![
                "  ╭──╮  ".to_string(),
                " │ •.• │ ".to_string(),
                "  ╰──╯  ".to_string(),
            ],
            duration: Duration::from_millis(300),
        },
        AnimationFrame {
            content: vec![
                "  ╭───╮  ".to_string(),
                " │ •.• │ ".to_string(),
                " │     │ ".to_string(),
                "  ╰───╯  ".to_string(),
            ],
            duration: Duration::from_millis(300),
        },
        AnimationFrame {
            content: vec![
                "  ╭────╮  ".to_string(),
                " │  •.•  │ ".to_string(),
                " │      │ ".to_string(),
                "  ╰────╯  ".to_string(),
            ],
            duration: Duration::from_millis(300),
        },
    ];
    
    Animation::new(frames, false)
}

/// Create happy dance animation
pub fn create_happy_dance_animation() -> Animation {
    let frames = vec![
        AnimationFrame {
            content: vec![
                "  ╭───╮  ".to_string(),
                " │ ◕ ◕ │ ".to_string(),
                " │  ◡  │ ".to_string(),
                "  ╰┬─┬╯  ".to_string(),
                "   ╯ ╰   ".to_string(),
            ],
            duration: Duration::from_millis(200),
        },
        AnimationFrame {
            content: vec![
                "  ╭───╮  ".to_string(),
                " │ ◕ ◕ │ ♪".to_string(),
                " │  ◡  │ ".to_string(),
                "  ╰┬─┬╯  ".to_string(),
                "   ╰ ╯   ".to_string(),
            ],
            duration: Duration::from_millis(200),
        },
        AnimationFrame {
            content: vec![
                "  ╭───╮  ♫".to_string(),
                " │ ★ ★ │ ".to_string(),
                " │  ▽  │ ".to_string(),
                "  ╰┬─┬╯  ".to_string(),
                "   ╯ ╰   ".to_string(),
            ],
            duration: Duration::from_millis(200),
        },
        AnimationFrame {
            content: vec![
                "  ╭───╮  ".to_string(),
                " │ ◕ ◕ │ ♪".to_string(),
                " │  ◡  │ ".to_string(),
                "  ╰┬─┬╯  ".to_string(),
                "   ╰ ╯   ".to_string(),
            ],
            duration: Duration::from_millis(200),
        },
    ];
    
    Animation::new(frames, true)
}

/// Create starving animation
pub fn create_starving_animation() -> Animation {
    let frames = vec![
        AnimationFrame {
            content: vec![
                "  ╭───╮  ".to_string(),
                " │ x x │ ".to_string(),
                " │  ╰  │ ".to_string(),
                "  ╰───╯  ".to_string(),
            ],
            duration: Duration::from_millis(500),
        },
        AnimationFrame {
            content: vec![
                "  ╭───╮  ".to_string(),
                " │ X X │ ...".to_string(),
                " │  ~  │ ".to_string(),
                "  ╰───╯  ".to_string(),
            ],
            duration: Duration::from_millis(500),
        },
    ];
    
    Animation::new(frames, true)
}

/// Create a loading/digesting animation
pub fn create_digesting_animation() -> Vec<String> {
    vec![
        format!("{}", "Digesting...".bright_yellow()),
        format!("{}", "[████░░░░░░] 40%".bright_green()),
    ]
}

/// Create sparkle effect
pub fn create_sparkle_effect() -> Vec<String> {
    vec![
        "  ✨ ✨ ✨  ".bright_yellow().to_string(),
        " ✨     ✨ ".bright_yellow().to_string(),
        "✨       ✨".bright_yellow().to_string(),
    ]
}

/// Create death animation frames
pub fn create_death_animation() -> Animation {
    let frames = vec![
        AnimationFrame {
            content: vec![
                "  ╭───╮  ".to_string(),
                " │ x x │ ".to_string(),
                " │  _  │ ".to_string(),
                "  ╰───╯  ".to_string(),
            ],
            duration: Duration::from_millis(300),
        },
        AnimationFrame {
            content: vec![
                "  ╭───╮  ".bright_red().to_string(),
                " │ X X │ ".bright_red().to_string(),
                " │  _  │ ".bright_red().to_string(),
                "  ╰───╯  ".bright_red().to_string(),
            ],
            duration: Duration::from_millis(300),
        },
        AnimationFrame {
            content: vec![
                "  _____  ".bright_black().to_string(),
                " │ RIP │ ".bright_black().to_string(),
                " │     │ ".bright_black().to_string(),
                "─┴─────┴─".bright_black().to_string(),
            ],
            duration: Duration::from_millis(1000),
        },
    ];
    
    Animation::new(frames, false)
}

/// Particle effect for feeding
pub fn create_feeding_particles(amount: usize) -> Vec<String> {
    let particles = match amount {
        0..=30 => vec![".", ".", "."],
        31..=100 => vec!["*", "*", "*", "*"],
        101..=300 => vec!["●", "●", "●", "●", "●"],
        _ => vec!["█", "█", "█", "█", "█", "█"],
    };
    
    particles.iter().map(|p| p.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_animation_creation() {
        let anim = create_eating_animation();
        assert!(!anim.frames.is_empty());
    }
    
    #[test]
    fn test_animation_update() {
        let mut anim = create_eating_animation();
        let frame = anim.update();
        assert!(frame.is_some());
    }
    
    #[test]
    fn test_feeding_particles() {
        let particles = create_feeding_particles(50);
        assert!(!particles.is_empty());
    }
}