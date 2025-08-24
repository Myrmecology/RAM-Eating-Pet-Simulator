// src/graphics/mod.rs
// RAM Eating Pet Simulator - Graphics Module

pub mod animations;
pub mod colors;
pub mod renderer;

use colored::*;

/// Border styles for UI elements
pub struct BorderStyle {
    pub top_left: &'static str,
    pub top_right: &'static str,
    pub bottom_left: &'static str,
    pub bottom_right: &'static str,
    pub horizontal: &'static str,
    pub vertical: &'static str,
}

/// Modern rounded border style
pub const ROUNDED_BORDER: BorderStyle = BorderStyle {
    top_left: "╭",
    top_right: "╮",
    bottom_left: "╰",
    bottom_right: "╯",
    horizontal: "─",
    vertical: "│",
};

/// Double-line border style
pub const DOUBLE_BORDER: BorderStyle = BorderStyle {
    top_left: "╔",
    top_right: "╗",
    bottom_left: "╚",
    bottom_right: "╝",
    horizontal: "═",
    vertical: "║",
};

/// Create a progress bar
pub fn create_progress_bar(current: f32, max: f32, width: usize, filled_color: Color, empty_color: Color) -> String {
    let percentage = (current / max).clamp(0.0, 1.0);
    let filled = (percentage * width as f32) as usize;
    let empty = width.saturating_sub(filled);
    
    let filled_part = "█".repeat(filled);
    let empty_part = "░".repeat(empty);
    
    format!("{}{}",
        filled_part.color(filled_color),
        empty_part.color(empty_color)
    )
}

/// Create a colored meter with percentage
pub fn create_meter(label: &str, current: f32, max: f32, color: Color) -> String {
    let percentage = ((current / max) * 100.0).clamp(0.0, 100.0) as i32;
    let bar = create_progress_bar(current, max, 20, color, Color::TrueColor { r: 64, g: 64, b: 64 });
    
    format!("{}: {} {}%", 
        label.bright_white(),
        bar,
        percentage.to_string().color(color)
    )
}

/// Center text within a given width
pub fn center_text(text: &str, width: usize) -> String {
    let text_len = text.len();
    if text_len >= width {
        text.to_string()
    } else {
        let padding = (width - text_len) / 2;
        let left_pad = " ".repeat(padding);
        let right_pad = " ".repeat(width - text_len - padding);
        format!("{}{}{}", left_pad, text, right_pad)
    }
}

/// Create a boxed text with border
pub fn create_box(content: Vec<String>, border: &BorderStyle, color: Color) -> Vec<String> {
    let max_width = content.iter().map(|s| s.len()).max().unwrap_or(0);
    let mut result = Vec::new();
    
    // Top border
    result.push(format!("{}{}{}",
        border.top_left.color(color),
        border.horizontal.repeat(max_width + 2).color(color),
        border.top_right.color(color)
    ));
    
    // Content with side borders
    for line in content {
        let padded = format!(" {} ", line);
        let padding = " ".repeat(max_width + 2 - padded.len());
        result.push(format!("{} {}{}{}",
            border.vertical.color(color),
            padded,
            padding,
            border.vertical.color(color)
        ));
    }
    
    // Bottom border
    result.push(format!("{}{}{}",
        border.bottom_left.color(color),
        border.horizontal.repeat(max_width + 2).color(color),
        border.bottom_right.color(color)
    ));
    
    result
}

/// Format time duration into readable string
pub fn format_duration(duration: std::time::Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Create a sparkline graph
pub fn create_sparkline(values: &[f32], width: usize) -> String {
    if values.is_empty() {
        return " ".repeat(width);
    }
    
    let sparks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let max = values.iter().fold(0.0_f32, |a, &b| a.max(b));
    let min = values.iter().fold(f32::MAX, |a, &b| a.min(b));
    let range = max - min;
    
    if range == 0.0 {
        return sparks[4].to_string().repeat(width.min(values.len()));
    }
    
    values.iter()
        .take(width)
        .map(|&v| {
            let normalized = ((v - min) / range * 7.0) as usize;
            sparks[normalized.min(7)]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_progress_bar() {
        let bar = create_progress_bar(50.0, 100.0, 10, Color::Green, Color::Red);
        assert!(bar.contains("█"));
        assert!(bar.contains("░"));
    }
    
    #[test]
    fn test_center_text() {
        let centered = center_text("test", 10);
        assert_eq!(centered.len(), 10);
        assert!(centered.contains("test"));
    }
    
    #[test]
    fn test_format_duration() {
        let duration = std::time::Duration::from_secs(3661);
        let formatted = format_duration(duration);
        assert_eq!(formatted, "1h 1m 1s");
    }
}