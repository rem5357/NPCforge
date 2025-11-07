# NPCForge

> A Rust CLI application that uses local AI to generate comprehensive D&D 2024 NPCs

Generate fully-fledged D&D characters with complete character sheets, stats, spells, equipment, and detailed backstories using Ollama and Qwen2.5.

## Features

- 🎲 **Complete Character Generation** - Race, class, subclass, level, ability scores, skills, and more
- ⚔️ **Combat Ready** - Attacks, spells, AC, HP, initiative automatically calculated
- 🎭 **Multiclass Support** - Up to 3 classes with automatic or manual level distribution
- 🗡️ **Fighting Styles** - Melee, ranged, or versatile combat preferences
- 📜 **Rich Backstories** - Detailed 3-5 paragraph backstories covering childhood, education, relationships, and current situation
- 🏘️ **Role-Based NPCs** - Combat roles (Mercenary, Soldier) or working NPCs (Farmer, Merchant, Scholar)
- 🎨 **High Variety** - Advanced randomization prevents repetitive characters
- ⚙️ **Customizable** - Extensive CLI parameters for fine-grained control
- 📦 **Batch Generation** - Generate up to 25 NPCs at once
- 💾 **JSON Export** - Clean, structured JSON output with level breakdown for multiclass

## Quick Start

### Prerequisites

1. **Install Rust** - https://rustup.rs/
2. **Install Ollama** - https://ollama.com/download
3. **Pull the AI Model**:
   ```bash
   ollama pull qwen2.5:32b-instruct
   ```

### Installation

```bash
# Clone the repository
git clone https://github.com/rem5357/NPCforge.git
cd NPCforge

# Build the project
cargo build --release
```

### Usage

```bash
# Generate one random NPC
cargo run

# Generate 5 random NPCs
cargo run -- -n 5

# Generate a Dwarf Ranger, level 5, Chaotic Good
cargo run -- -r "Dwarf" -c "Ranger" -l 5 -a "CG"

# Generate an NPC with a specific name
cargo run -- --name "Thorin Oakenshield"
```

## Command-Line Options

| Option | Description | Example |
|--------|-------------|---------|
| `-n, --count` | Number of NPCs to generate (max 25) | `-n 10` |
| `--name` | Specific character name | `--name "Gandalf"` |
| `-r, --race` | Character race | `-r "Elf"` |
| `-c, --class` | Character class (comma-separated for multiclass) | `-c "Wizard"` or `-c "Fighter,Wizard"` |
| `-l, --level` | Character level (1-20, total for multiclass) | `-l 15` |
| `--lvl1`, `--lvl2`, `--lvl3` | Manual level distribution for multiclass | `--lvl1 7 --lvl2 3` |
| `--low`, `--high` | Level range for random generation | `--low 5 --high 10` |
| `-a, --alignment` | Character alignment | `-a "LG"` |
| `--role` | Role/occupation | `--role "Farmer"` or `--role "Merchant"` |
| `--melee` | Prefer melee combat style | `--melee` |
| `--ranged` | Prefer ranged combat style | `--ranged` |
| `-h, --help` | Display help information | `--help` |

## Example Output

```json
{
  "name": "Zaraquill Tanglefoot",
  "race": "Tabaxi",
  "class": "Rogue",
  "subclass": "Inquisitive",
  "level": 14,
  "background": "Guild Artisan (Brewer)",
  "alignment": "Chaotic Neutral",
  "ability_scores": {
    "strength": 10,
    "dexterity": 18,
    "constitution": 14,
    ...
  },
  ...
}
```

## How It Works

NPCForge uses:
- **Ollama** - Local AI runtime for privacy and control
- **Qwen2.5-32B-Instruct** - Powerful model with excellent JSON generation
- **Rust** - Fast, reliable, and memory-safe
- **Advanced Prompting** - Ensures varied, creative characters with complete D&D 2024 stats

The AI generates complete characters following D&D 2024 rules, including:
- Ability scores and modifiers
- Class features and proficiencies
- Spell selections for spellcasters
- Equipment appropriate to class and level
- Personality traits, bonds, ideals, and flaws
- Comprehensive backstory

## Project Documentation

See [Project.md](Project.md) for:
- Detailed project overview
- Technology stack
- Development timeline
- Lessons learned
- Future enhancements
- Known issues

## Example Characters

**Vesla Dawnstar** - Triton Ranger (Horizon Walker), Level 16
- Entertainer background with aquatic heritage
- Specialized in planar exploration

**Zaraquill Tanglefoot** - Tabaxi Rogue (Inquisitive), Level 14
- Guild Artisan (Brewer) with investigative focus
- Unique race/class combination

## Tech Stack

- **Rust** 1.90+
- **Ollama** with Qwen2.5-32B-Instruct
- **Dependencies**: tokio, reqwest, serde, anyhow, clap

## License

(To be determined)

## Contributing

Contributions welcome! This is an early-stage project - feel free to open issues or submit PRs.

## Acknowledgments

- **Ollama Team** - Local AI runtime
- **Qwen Team** - Amazing language model
- **Rust Community** - Fantastic ecosystem
- **D&D Community** - Inspiration

---

Built with ❤️ and 🤖 AI assistance
