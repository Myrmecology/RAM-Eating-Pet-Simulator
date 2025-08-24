// src/graphics/colors.rs
// RAM Eating Pet Simulator - Color Definitions and Themes

use colored::Color;

/// Color theme for the game
#[derive(Debug, Clone)]
pub struct ColorTheme {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub info: Color,
    pub background: Color,
    pub text: Color,
}

/// Default color theme
impl Default for ColorTheme {
    fn default() -> Self {
        ColorTheme {
            primary: Color::Blue,
            secondary: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            danger: Color::Red,
            info: Color::Magenta,
            background: Color::Black,
            text: Color::White,
        }
    }
}

/// Neon color theme
pub fn neon_theme() -> ColorTheme {
    ColorTheme {
        primary: Color::TrueColor { r: 0, g: 255, b: 255 },     // Cyan
        secondary: Color::TrueColor { r: 255, g: 0, b: 255 },   // Magenta
        success: Color::TrueColor { r: 0, g: 255, b: 0 },       // Lime
        warning: Color::TrueColor { r: 255, g: 255, b: 0 },     // Yellow
        danger: Color::TrueColor { r: 255, g: 0, b: 128 },      // Hot Pink
        info: Color::TrueColor { r: 128, g: 128, b: 255 },      // Light Blue
        background: Color::TrueColor { r: 16, g: 16, b: 32 },   // Dark Blue
        text: Color::TrueColor { r: 240, g: 240, b: 240 },      // Light Gray
    }
}

/// Retro terminal theme
pub fn retro_theme() -> ColorTheme {
    ColorTheme {
        primary: Color::TrueColor { r: 0, g: 255, b: 0 },       // Classic Green
        secondary: Color::TrueColor { r: 0, g: 200, b: 0 },     // Dark Green
        success: Color::TrueColor { r: 128, g: 255, b: 128 },   // Light Green
        warning: Color::TrueColor { r: 255, g: 165, b: 0 },     // Amber
        danger: Color::TrueColor { r: 255, g: 100, b: 100 },    // Light Red
        info: Color::TrueColor { r: 100, g: 200, b: 100 },      // Mid Green
        background: Color::Black,
        text: Color::TrueColor { r: 0, g: 255, b: 0 },          // Terminal Green
    }
}

/// Get color based on percentage (gradient from red to green)
pub fn gradient_color(percentage: f32) -> Color {
    let p = percentage.clamp(0.0, 1.0);
    
    let r = ((1.0 - p) * 255.0) as u8;
    let g = (p * 255.0) as u8;
    let b = 0;
    
    Color::TrueColor { r, g, b }
}

/// Get color for RAM usage level
pub fn ram_usage_color(percentage: f32) -> Color {
    match percentage {
        p if p >= 0.9 => Color::TrueColor { r: 255, g: 0, b: 0 },      // Critical - Red
        p if p >= 0.75 => Color::TrueColor { r: 255, g: 128, b: 0 },   // High - Orange
        p if p >= 0.5 => Color::TrueColor { r: 255, g: 255, b: 0 },    // Medium - Yellow
        _ => Color::TrueColor { r: 0, g: 255, b: 0 },                  // Low - Green
    }
}

/// Get color for pet mood
pub fn mood_color(mood: &str) -> Color {
    match mood.to_lowercase().as_str() {
        "happy" | "excited" => Color::TrueColor { r: 0, g: 255, b: 0 },
        "content" => Color::TrueColor { r: 0, g: 128, b: 255 },
        "hungry" => Color::TrueColor { r: 255, g: 165, b: 0 },
        "starving" => Color::TrueColor { r: 255, g: 0, b: 0 },
        "sad" => Color::TrueColor { r: 128, g: 128, b: 128 },
        "angry" => Color::TrueColor { r: 255, g: 0, b: 128 },
        "sleepy" => Color::TrueColor { r: 192, g: 192, b: 255 },
        "dead" => Color::TrueColor { r: 64, g: 64, b: 64 },
        _ => Color::White,
    }
}

/// Rainbow colors for special effects
pub fn rainbow_colors() -> Vec<Color> {
    vec![
        Color::TrueColor { r: 255, g: 0, b: 0 },     // Red
        Color::TrueColor { r: 255, g: 127, b: 0 },   // Orange
        Color::TrueColor { r: 255, g: 255, b: 0 },   // Yellow
        Color::TrueColor { r: 0, g: 255, b: 0 },     // Green
        Color::TrueColor { r: 0, g: 0, b: 255 },     // Blue
        Color::TrueColor { r: 75, g: 0, b: 130 },    // Indigo
        Color::TrueColor { r: 148, g: 0, b: 211 },   // Violet
    ]
}

/// Get a random bright color
pub fn random_bright_color() -> Color {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    let colors = vec![
        Color::TrueColor { r: 255, g: 0, b: 128 },
        Color::TrueColor { r: 0, g: 255, b: 255 },
        Color::TrueColor { r: 255, g: 255, b: 0 },
        Color::TrueColor { r: 255, g: 0, b: 255 },
        Color::TrueColor { r: 0, g: 255, b: 0 },
        Color::TrueColor { r: 255, g: 128, b: 0 },
    ];
    
    colors[rng.gen_range(0..colors.len())]
}

/// Pulse effect - returns color with varying brightness
pub fn pulse_color(base_color: Color, time: f32) -> Color {
    let brightness = (time.sin() + 1.0) / 2.0; // 0.0 to 1.0
    
    match base_color {
        Color::TrueColor { r, g, b } => {
            Color::TrueColor {
                r: (r as f32 * brightness) as u8,
                g: (g as f32 * brightness) as u8,
                b: (b as f32 * brightness) as u8,
            }
        }
        _ => base_color,
    }
}

/// Convert health/hunger to emoji
pub fn stat_to_emoji(value: f32, max: f32) -> &'static str {
    let percentage = (value / max * 100.0) as i32;
    
    match percentage {
        90..=100 => "🟢",
        70..=89 => "🟡",
        50..=69 => "🟠",
        30..=49 => "🔴",
        _ => "💀",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gradient_color() {
        let color_low = gradient_color(0.0);
        let color_high = gradient_color(1.0);
        
        match (color_low, color_high) {
            (Color::TrueColor { r: r1, g: g1, b: _ }, 
             Color::TrueColor { r: r2, g: g2, b: _ }) => {
                assert!(r1 > r2);
                assert!(g1 < g2);
            }
            _ => panic!("Expected TrueColor"),
        }
    }
    
    #[test]
    fn test_mood_colors() {
        let happy = mood_color("happy");
        let sad = mood_color("sad");
        
        assert!(matches!(happy, Color::TrueColor { .. }));
        assert!(matches!(sad, Color::TrueColor { .. }));
    }
    
    #[test]
    fn test_stat_emoji() {
        assert_eq!(stat_to_emoji(95.0, 100.0), "🟢");
        assert_eq!(stat_to_emoji(25.0, 100.0), "💀");
    }
}