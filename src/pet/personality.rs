// src/pet/personality.rs
// RAM Eating Pet Simulator - Personality System

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

/// Pet personality traits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    /// Sass level (0.0 = polite, 1.0 = absolute menace)
    sass_level: f32,
    /// How often it wants attention
    attention_need: f32,
    /// Love for chaos and dangerous operations
    chaos_affinity: f32,
    /// How dramatic the pet is
    drama_level: f32,
    /// Preferred food size
    food_preference: FoodPreference,
    /// Pet's unique quirks
    quirks: Vec<Quirk>,
}

/// Food preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FoodPreference {
    SmallFrequentMeals,  // Likes many small meals
    BingeEater,          // Wants huge chunks
    Gourmet,             // Specific amounts (42 MB, 69 MB, etc.)
    Chaotic,             // Random preferences
}

/// Unique quirks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Quirk {
    DramaQueen,          // Overreacts to everything
    Philosopher,         // Makes deep observations
    Comedian,            // Tells jokes
    Gremlin,            // Causes chaos
    Sweetheart,         // Always positive
    Grumpy,             // Never satisfied
    Nerd,               // References tech stuff
    Artist,             // Poetic responses
}

/// Pet moods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Mood {
    Happy,
    Excited,
    Content,
    Hungry,
    Starving,
    Sad,
    Angry,
    Sleepy,
    Dead,
}

impl Personality {
    /// Generate a random personality
    pub fn generate_random() -> Self {
        let mut rng = thread_rng();
        
        // Generate random traits
        let sass_level = rng.gen_range(0.0..1.0);
        let attention_need = rng.gen_range(0.0..1.0);
        let chaos_affinity = rng.gen_range(0.0..1.0);
        let drama_level = rng.gen_range(0.0..1.0);
        
        // Random food preference
        let food_preference = match rng.gen_range(0..4) {
            0 => FoodPreference::SmallFrequentMeals,
            1 => FoodPreference::BingeEater,
            2 => FoodPreference::Gourmet,
            _ => FoodPreference::Chaotic,
        };
        
        // Generate 1-3 random quirks
        let num_quirks = rng.gen_range(1..=3);
        let mut quirks = Vec::new();
        for _ in 0..num_quirks {
            let quirk = match rng.gen_range(0..8) {
                0 => Quirk::DramaQueen,
                1 => Quirk::Philosopher,
                2 => Quirk::Comedian,
                3 => Quirk::Gremlin,
                4 => Quirk::Sweetheart,
                5 => Quirk::Grumpy,
                6 => Quirk::Nerd,
                _ => Quirk::Artist,
            };
            if !quirks.iter().any(|q| std::mem::discriminant(q) == std::mem::discriminant(&quirk)) {
                quirks.push(quirk);
            }
        }
        
        Personality {
            sass_level,
            attention_need,
            chaos_affinity,
            drama_level,
            food_preference,
            quirks,
        }
    }
    
    /// Generate a name based on personality
    pub fn generate_name(&self) -> String {
        let mut rng = thread_rng();
        
        let prefix = if self.chaos_affinity > 0.7 {
            ["Chaos", "Havoc", "Mayhem", "Riot", "Anarchy"]
        } else if self.sass_level > 0.7 {
            ["Sir", "Lady", "Captain", "Professor", "Dr."]
        } else if self.drama_level > 0.7 {
            ["Drama", "Diva", "Star", "Prima", "Maestro"]
        } else {
            ["Byte", "Pixel", "Bit", "Nano", "Mega"]
        };
        
        let suffix = if self.has_quirk(&Quirk::Sweetheart) {
            ["Cuddles", "Snuggles", "Sweetie", "Honey", "Sugar"]
        } else if self.has_quirk(&Quirk::Grumpy) {
            ["Grumps", "Grouch", "Cranky", "Grizzle", "Sour"]
        } else if self.has_quirk(&Quirk::Nerd) {
            ["Cache", "Buffer", "Stack", "Heap", "Core"]
        } else {
            ["Munch", "Chomps", "Nibbles", "Gobbler", "Eater"]
        };
        
        format!("{} {}", 
            prefix[rng.gen_range(0..prefix.len())],
            suffix[rng.gen_range(0..suffix.len())]
        )
    }
    
    /// Get feeding reaction based on mood
    pub fn get_feeding_reaction(&self, mood: &Mood) -> &str {
        let reactions = match mood {
            Mood::Happy => {
                if self.sass_level > 0.7 {
                    vec!["Finally, some good food", "About time", "I suppose this will do"]
                } else if self.has_quirk(&Quirk::Sweetheart) {
                    vec!["Yummy!", "Thank you so much!", "You're the best!"]
                } else {
                    vec!["Nom nom!", "Delicious!", "Tasty bytes!"]
                }
            },
            Mood::Excited => {
                if self.has_quirk(&Quirk::DramaQueen) {
                    vec!["THIS IS THE BEST DAY EVER!", "I'M LITERALLY DYING OF JOY!", "INCREDIBLE!"]
                } else {
                    vec!["AMAZING!", "YES YES YES!", "MORE MORE MORE!"]
                }
            },
            Mood::Hungry => {
                if self.has_quirk(&Quirk::Grumpy) {
                    vec!["Finally...", "Took you long enough", "Still hungry though"]
                } else {
                    vec!["I needed that", "Much better", "Keep it coming"]
                }
            },
            Mood::Starving => {
                vec!["FEED ME NOW!", "I'M WASTING AWAY!", "EMERGENCY FOOD REQUIRED!"]
            },
            _ => {
                vec!["Munch munch", "Nom", "...", "*eating sounds*"]
            }
        };
        
        reactions[thread_rng().gen_range(0..reactions.len())]
    }
    
    /// Get favorite food size based on personality
    pub fn get_favorite_food_size(&self) -> usize {
        match self.food_preference {
            FoodPreference::SmallFrequentMeals => thread_rng().gen_range(10..30),
            FoodPreference::BingeEater => thread_rng().gen_range(200..500),
            FoodPreference::Gourmet => {
                // Specific "perfect" amounts
                let gourmet_sizes = [42, 69, 100, 128, 256, 314, 420];
                gourmet_sizes[thread_rng().gen_range(0..gourmet_sizes.len())]
            },
            FoodPreference::Chaotic => thread_rng().gen_range(1..1000),
        }
    }
    
    /// Check if pet has a specific quirk
    fn has_quirk(&self, quirk: &Quirk) -> bool {
        self.quirks.iter().any(|q| std::mem::discriminant(q) == std::mem::discriminant(quirk))
    }
    
    /// Get a random comment based on personality
    pub fn get_random_comment(&self) -> String {
        let mut rng = thread_rng();
        
        if self.has_quirk(&Quirk::Philosopher) {
            let comments = [
                "What is RAM but temporary existence?",
                "I eat, therefore I am",
                "Is memory real if it's virtual?",
                "In the end, aren't we all just consuming resources?",
            ];
            comments[rng.gen_range(0..comments.len())].to_string()
        } else if self.has_quirk(&Quirk::Comedian) {
            let comments = [
                "Why did the RAM cross the motherboard? To get to the other byte!",
                "I'm not fat, I'm just... allocated",
                "RAM? More like YUM!",
                "I've got a giga-bite!",
            ];
            comments[rng.gen_range(0..comments.len())].to_string()
        } else if self.has_quirk(&Quirk::Nerd) {
            let comments = [
                "My complexity is O(nom)",
                "Segmentation fault: hunger at 0x0",
                "sudo feed me",
                "Error 404: Food not found",
            ];
            comments[rng.gen_range(0..comments.len())].to_string()
        } else {
            "...".to_string()
        }
    }
}

impl Mood {
    /// Get RGB color for mood
    pub fn get_color(&self) -> (u8, u8, u8) {
        match self {
            Mood::Happy => (0, 255, 0),      // Green
            Mood::Excited => (255, 255, 0),   // Yellow
            Mood::Content => (0, 128, 255),   // Blue
            Mood::Hungry => (255, 165, 0),    // Orange
            Mood::Starving => (255, 0, 0),    // Red
            Mood::Sad => (128, 128, 128),     // Gray
            Mood::Angry => (255, 0, 128),     // Red-Pink
            Mood::Sleepy => (192, 192, 255),  // Light Blue
            Mood::Dead => (64, 64, 64),       // Dark Gray
        }
    }
    
    /// Get mood name
    pub fn name(&self) -> &str {
        match self {
            Mood::Happy => "Happy",
            Mood::Excited => "Excited",
            Mood::Content => "Content",
            Mood::Hungry => "Hungry",
            Mood::Starving => "STARVING",
            Mood::Sad => "Sad",
            Mood::Angry => "Angry",
            Mood::Sleepy => "Sleepy",
            Mood::Dead => "Dead",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_personality_generation() {
        let personality = Personality::generate_random();
        assert!(personality.sass_level >= 0.0 && personality.sass_level <= 1.0);
        assert!(!personality.quirks.is_empty());
    }
    
    #[test]
    fn test_name_generation() {
        let personality = Personality::generate_random();
        let name = personality.generate_name();
        assert!(!name.is_empty());
        assert!(name.contains(' '));
    }
}