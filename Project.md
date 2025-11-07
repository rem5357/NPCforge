# NPCForge

A Rust CLI application that uses local AI (Ollama with Qwen2.5) to generate comprehensive D&D 2024 NPCs with full character sheets, stats, spells, equipment, and detailed backstories.

## Overview

NPCForge generates fully-fledged D&D characters complete with:
- Race, class, subclass, and level
- Ability scores and all derived stats (AC, HP, initiative, proficiency bonus)
- Skills, saving throws, languages, and tool proficiencies
- Attacks and combat abilities
- Spells (for spellcasting classes) with spell slots and known spells
- Equipment, weapons, armor, and treasure
- Personality traits, ideals, bonds, and flaws
- Detailed 3-5 paragraph backstory covering childhood, education, life events, relationships, personality, and current situation
- Physical appearance and distinguishing features
- Class features

## Technology Stack

### Rust Dependencies
- **tokio** (1.41) - Async runtime
- **reqwest** (0.12) - HTTP client for Ollama API
- **serde** (1.0) - JSON serialization/deserialization
- **serde_json** (1.0) - JSON processing
- **anyhow** (1.0) - Error handling
- **clap** (4.5) - Command-line argument parsing

### AI Model
- **Ollama** - Local AI runtime (https://ollama.com)
- **Qwen2.5:32b-instruct** - Primary model (19GB)
  - Alternative: qwen2.5:14b-instruct (9GB)
  - Excellent for structured JSON output and creative content generation
  - Strong tool/JSON discipline
  - Long context support

### AI Configuration
- **Temperature**: 1.2 (higher for more randomness)
- **Top-P**: 0.95 (nucleus sampling)
- **Top-K**: 50 (for variety)

## Setup Instructions

### Prerequisites
1. **Install Rust**
   - Download from https://rustup.rs/
   - Follow platform-specific instructions

2. **Install Ollama**
   - Windows: https://ollama.com/download/windows
   - Linux/Mac: https://ollama.com/download
   - Run the installer

3. **Pull the Qwen2.5 Model**
   ```bash
   ollama pull qwen2.5:32b-instruct
   # Or for the smaller version:
   ollama pull qwen2.5:14b-instruct
   ```

### Building the Project
```bash
cd NPCforge
cargo build --release
```

### Running the Application
```bash
# Generate one random NPC
cargo run

# Or use the release build
./target/release/npcforge
```

## Command-Line Parameters

### Options
- `-n, --count <COUNT>` - Number of NPCs to generate (max 25, default: 1)
- `--name <NAME>` - Specific name for the NPC (automatically sets count to 1)
- `-r, --race <RACE>` - Specify race (e.g., "Dwarf", "Elf", "Dragonborn", "Tabaxi")
- `-c, --class <CLASS>` - Specify class (e.g., "Ranger", "Wizard", "Barbarian", "Rogue")
- `-l, --level <LEVEL>` - Specify level (1-20)
- `-a, --alignment <ALIGNMENT>` - Specify alignment (e.g., "CG", "LN", "CE")
- `-h, --help` - Display help information

### Usage Examples
```bash
# Generate one random NPC
cargo run

# Generate 5 random NPCs
cargo run -- -n 5

# Generate a specific Dwarf Ranger, level 5, Chaotic Good
cargo run -- -r "Dwarf" -c "Ranger" -l 5 -a "CG"

# Generate an NPC with a specific name
cargo run -- --name "Mithroll"

# Generate 10 Wizards
cargo run -- -n 10 -c "Wizard"

# Get help
cargo run -- --help
```

## Output Format

NPCs are saved as JSON files in the current directory with filenames based on the character name:
- Single NPC: `Character_Name.json`
- Multiple NPCs: `Character_Name_1.json`, `Character_Name_2.json`, etc.

### JSON Structure
```json
{
  "name": "Character name",
  "race": "Race",
  "class": "Class",
  "subclass": "Subclass",
  "level": 10,
  "background": "Background",
  "alignment": "Alignment",
  "ability_scores": { ... },
  "hit_points": { ... },
  "armor_class": 15,
  "initiative": 2,
  "speed": 30,
  "proficiency_bonus": 4,
  "skills": [ ... ],
  "saving_throws": [ ... ],
  "languages": [ ... ],
  "tool_proficiencies": [ ... ],
  "attacks": [ ... ],
  "spells": { ... },
  "equipment": { ... },
  "personality": { ... },
  "backstory": "Detailed backstory...",
  "appearance": { ... },
  "features": [ ... ]
}
```

## What We've Built

### Phase 1: Initial Setup
- ✅ Created Rust project structure
- ✅ Added dependencies (tokio, reqwest, serde, anyhow)
- ✅ Designed comprehensive NPC data structures
- ✅ Implemented Ollama API client

### Phase 2: AI Integration
- ✅ Created detailed prompt for NPC generation
- ✅ Integrated with Ollama's Qwen2.5 model
- ✅ Implemented JSON parsing with flexible schema (using `#[serde(default)]`)
- ✅ Added error handling and user-friendly messages

### Phase 3: Randomization & Quality
- ✅ Added temperature/sampling parameters for true randomness
- ✅ Enhanced prompt to avoid patterns (e.g., too many elves/wizards)
- ✅ Expanded backstory requirements (childhood, education, relationships, etc.)
- ✅ Tested and verified varied output (Triton Ranger, Tabaxi Rogue, etc.)

### Phase 4: CLI Features
- ✅ Added clap for command-line argument parsing
- ✅ Implemented multi-NPC generation with `-n` flag
- ✅ Added constraint parameters (name, race, class, level, alignment)
- ✅ Created dynamic prompt generation based on user constraints
- ✅ Implemented filename conflict resolution (numbered suffixes)
- ✅ Added validation (count ≤ 25, level 1-20)
- ✅ Created summary output with success/failure counts

## Lessons Learned

### AI Integration
1. **Model Selection**: Qwen2.5 excels at structured JSON output and creative storytelling
2. **Temperature Matters**: Default temperature (0.8) was too low, causing repetitive outputs (all elves/wizards). Increasing to 1.2 dramatically improved variety.
3. **Prompt Engineering**: Being explicit about randomization requirements ("DO NOT default to elves/wizards") helped break patterns
4. **Flexible Schemas**: Using `#[serde(default)]` for optional fields prevents parsing failures when AI doesn't generate every field
5. **Constraint Handling**: AI needs clear "MUST follow" language when user specifies constraints

### Rust Development
1. **Async/Await**: Tokio makes HTTP requests clean and efficient
2. **Error Handling**: `anyhow` provides excellent error context chaining
3. **Clap**: Makes CLI argument parsing trivial with derive macros
4. **Serde**: Powerful serialization with attributes like `#[serde(rename)]` for reserved keywords ("class")

### Data Modeling
1. **Nested Structures**: Breaking down complex data (spells, equipment, appearance) into separate structs improves readability
2. **Option Types**: Using `Option<T>` for nullable fields (subclass, spells) handles non-spellcasters gracefully
3. **Type Safety**: Using `u8` for levels/counts provides compile-time validation

### User Experience
1. **Progress Indicators**: Showing "Generating NPC 1/5..." helps users track progress
2. **Summary Output**: Final success/failure counts provide clear feedback
3. **Helpful Errors**: Suggesting "ollama pull qwen2.5:32b-instruct" when Ollama fails

### Error Handling & Debugging
1. **Timeout Configuration**: Large AI models need extended timeouts (10 minutes vs default 30 seconds)
2. **Error Context**: Using anyhow's `context()` with format strings to include response snippets in errors
3. **Rate Limiting**: Adding small delays between batch requests prevents overwhelming the AI server
4. **Error Chain Display**: Using `{:#}` format specifier shows full error context to users

## Known Issues

1. **Batch Generation Reliability**: When generating multiple NPCs (5+), occasional JSON parsing failures or connection timeouts may occur
   - **Mitigation**: Extended HTTP timeout to 10 minutes, added 500ms delay between requests
   - **Debugging**: Enhanced error messages now show actual response content when parsing fails
2. **Constrained Generation Parsing**: Sometimes fails when multiple constraints are specified (needs investigation)
3. **Backstory Format**: AI sometimes condenses backstory into 1 paragraph instead of 3-5 (prompt could be refined)
4. **No Spell Validation**: Doesn't verify that selected spells are valid for the class/level
5. **Windows Path Handling**: Character names with special characters might cause file save issues

## Future Enhancements (TODOs)

### High Priority
- [ ] Fix constrained generation parsing errors
- [ ] Add `.gitignore` file (ignore `target/`, `*.json`, etc.)
- [ ] Create comprehensive README.md
- [ ] Add example generated NPCs to repository

### Medium Priority
- [ ] Add more CLI options:
  - [ ] Background selection (`-b, --background`)
  - [ ] Gender/pronoun specification
  - [ ] Specific ability score allocation method
  - [ ] Output directory specification
- [ ] Add configuration file support (TOML/YAML)
- [ ] Support for other models (llama, mistral, etc.)
- [ ] Progress bar for multi-NPC generation
- [ ] Validate spell selections against D&D rules
- [ ] Add character sheet PDF export

### Low Priority
- [ ] Blazor WASM frontend for web-based generation
- [ ] Interactive mode with prompts
- [ ] Character image generation (using Stable Diffusion)
- [ ] Party generation (balanced group composition)
- [ ] Character evolution/leveling system
- [ ] Database storage (SQLite) for character library
- [ ] Search/filter existing characters
- [ ] Character comparison tools
- [ ] Export to other formats (XML, YAML, CSV)

### Code Quality
- [ ] Add unit tests for data structures
- [ ] Add integration tests for Ollama API
- [ ] Create mock Ollama server for testing
- [ ] Add documentation comments (rustdoc)
- [ ] Set up CI/CD pipeline (GitHub Actions)
- [ ] Add logging (tracing/log crate)
- [ ] Performance profiling for large batch generation

### Documentation
- [ ] Create CONTRIBUTING.md
- [ ] Add code of conduct
- [ ] Create issue templates
- [ ] Add pull request template
- [ ] Document prompt engineering decisions
- [ ] Create troubleshooting guide

## Project Structure

```
NPCforge/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Rust dependencies
├── Cargo.lock           # Dependency lock file
├── Project.md           # This file
├── .claude/             # Claude Code configuration
├── target/              # Build artifacts (gitignored)
└── *.json               # Generated NPC files (gitignored)
```

## Development Timeline

**Session 1**: Initial setup and basic generation
- Project initialization
- Ollama integration
- Basic NPC generation
- Discovered randomization issues

**Session 2**: Improved randomization and CLI features
- Added temperature/sampling parameters
- Enhanced prompt for variety
- Implemented CLI argument parsing
- Multi-NPC generation
- Constraint-based generation

## Example Generated NPCs

### Vesla Dawnstar
- **Race**: Triton
- **Class**: Ranger (Horizon Walker), Level 16
- **Background**: Entertainer
- **Alignment**: Chaotic Neutral
- **Highlights**: Aquatic heritage, planar exploration focus, performer background

### Zaraquill Tanglefoot
- **Race**: Tabaxi
- **Class**: Rogue (Inquisitive), Level 14
- **Background**: Guild Artisan (Brewer)
- **Alignment**: Chaotic Neutral
- **Highlights**: Unique race/class combo, investigative focus

## Contributing

(Future section for when project is open-sourced)

## License

(To be determined)

## Acknowledgments

- **Ollama Team** - For creating an excellent local AI runtime
- **Qwen Team** - For the amazing Qwen2.5 model
- **Rust Community** - For fantastic libraries and documentation
- **D&D Community** - For inspiring this project

---

**Last Updated**: November 6, 2024
