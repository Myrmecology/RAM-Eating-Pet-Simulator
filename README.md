# 🎮 RAM Eating Pet Simulator
# Random Access Memory

A virtual pet that **literally** consumes your computer's RAM to survive! Watch in Task Manager as your adorable digital companion grows by eating actual memory.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)

## 🌟 Features

- **Real RAM Consumption**: Your pet actually allocates and uses system memory
- **Unique Personalities**: Each pet has randomly generated traits and quirks
- **Visual Growth**: Watch your pet evolve from tiny (50MB) to GIGANTIC (2GB+)
- **Colorful Terminal Graphics**: Beautiful ASCII art with full RGB colors
- **Mood System**: Keep your pet happy and well-fed or face the consequences
- **Save/Load System**: Your pet persists between sessions

## 🚀 Quick Start

### Prerequisites
- Rust 1.70 or higher
- Windows (Git Bash recommended) / Linux / macOS
- At least 4GB RAM (for safe gameplay)

### Installation

```bash
# Clone the repository
git clone https://github.com/Myrmecology/RAM-Eating-Pet-Simulator.git
cd RAM-Eating-Pet-Simulator

# Build the project
cargo build --release

# Run the game
cargo run --release

🎮 How to Play
Controls

SPACE - Feed your pet (50 MB)
F - Give favorite food (varies by personality)
S - Save game
L - Load game
H - Show help
Q/ESC - Quit game
X - Emergency exit (WARNING: Pet dies!)

Gameplay Tips

Monitor Hunger: Keep hunger below 80% or your pet will become unhappy
Watch System RAM: Don't let your system run out of memory!
Personality Matters: Each pet has different food preferences
Size Evolution: Your pet changes appearance as it grows

🎨 Pet Evolution Stages
StageSizeDescriptionBaby0-50 MBJust a tiny RAM nibblerChild51-150 MBGrowing and learningTeen151-300 MBAppetite increasingAdult301-500 MBFully grownChubby501-1000 MBWell-fedFat1001-1500 MBGetting largeHuge1501-2000 MBImpressive sizeGIGANTIC2000+ MBABSOLUTE UNIT
⚠️ Safety Features

Minimum RAM Reserve: Always keeps 1GB free for system stability
Warning System: Alerts when RAM is running low
Safe Allocation: Prevents system crashes from over-allocation
Clean Exit: Releases all RAM when closing

🛠️ Configuration
Edit config.toml to customize:

Starting pet size
Metabolism rate
Difficulty settings
Graphics options

🐛 Troubleshooting
"Not enough free RAM!"

Close other applications (especially Chrome!)
Reduce pet size by waiting for metabolism
Restart the game

Colors not showing

Ensure your terminal supports ANSI colors
Use Windows Terminal or Git Bash on Windows

Performance issues

Lower the FPS in config.toml
Disable animations

📦 Building from Source
bash# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
🤝 Contributing
Contributions are welcome! Feel free to:

Report bugs
Suggest features
Submit pull requests

📜 License
MIT License - See LICENSE file for details
🙏 Acknowledgments

Built with Rust 🦀
Terminal graphics powered by crossterm and colored
System monitoring via sysinfo

Created with 💙 and a lot of RAM

