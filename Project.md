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
- `-c, --class <CLASS>` - Specify class(es). Comma-separated for multiclass, max 3 (e.g., "Ranger" or "Fighter,Wizard,Cleric")
- `-l, --level <LEVEL>` - Specify total level (1-20). For multiclass, this is distributed across all classes
- `--lvl1 <LEVEL>` - Levels in first class (for manual multiclass distribution)
- `--lvl2 <LEVEL>` - Levels in second class (for manual multiclass distribution)
- `--lvl3 <LEVEL>` - Levels in third class (for manual multiclass distribution)
- `--low <LEVEL>` - Minimum level for random generation (1-20, default: 1)
- `--high <LEVEL>` - Maximum level for random generation (1-20, default: 10)
- `-a, --alignment <ALIGNMENT>` - Specify alignment (e.g., "CG", "LN", "CE")
- `--role <ROLE>` - Role/occupation (e.g., "Mercenary", "Scholar", "Pirate", "random", default: "Mercenary")
- `--melee` - Prefer melee combat style (affects weapons, spells, subclass, feats)
- `--ranged` - Prefer ranged combat style (affects weapons, spells, subclass, feats)
- `-h, --help` - Display help information

### Fighting Style Notes
- `--melee`: Prioritizes melee weapons, close-range/touch spells, melee-focused subclasses
- `--ranged`: Prioritizes ranged weapons, long-range spells, ranged-focused subclasses
- Both flags: Creates versatile character with mix of melee and ranged options
- Neither flag: Random selection (90% ranged for spellcasters, 50/50 for martial classes)

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

# Generate NPCs between level 5-10
cargo run -- -n 3 --low 5 --high 10

# Generate a multiclass Fighter/Wizard (level 10, AI distributes)
cargo run -- -c "Fighter,Wizard" -l 10

# Generate a multiclass with manual level distribution (7 Paladin / 3 Warlock)
cargo run -- -c "Paladin,Warlock" --lvl1 7 --lvl2 3

# Generate a 3-class multiclass (5 Fighter / 3 Rogue / 2 Ranger)
cargo run -- -c "Fighter,Rogue,Ranger" --lvl1 5 --lvl2 3 --lvl3 2

# Generate an NPC with a specific role
cargo run -- --role "Pirate" -c "Rogue"

# Generate a melee fighter
cargo run -- -c "Fighter" -l 8 --melee

# Generate a ranged wizard
cargo run -- -c "Wizard" -l 10 --ranged

# Generate a versatile ranger (both melee and ranged)
cargo run -- -c "Ranger" -l 7 --melee --ranged

# Generate a farmer NPC (non-adventurer backstory)
cargo run -- --role "Farmer" -c "Druid" -l 5

# Generate a merchant NPC
cargo run -- --role "Merchant" -c "Bard" -l 4

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
  "class": "Class (or Fighter/Wizard for multiclass)",
  "subclass": "Subclass (or Champion/Evocation for multiclass)",
  "level": 10,
  "class_levels": {"Fighter": 6, "Wizard": 4},  // Optional, multiclass only
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

### Phase 5: Multiclass System
- ✅ Implemented multiclass support (up to 3 classes)
- ✅ Added level range parameters (`--low`, `--high`)
- ✅ Added role/occupation system with 15+ role options
- ✅ Correct multiclass level distribution (total levels split across classes)
- ✅ Manual level distribution parameters (`--lvl1`, `--lvl2`, `--lvl3`)
- ✅ Fixed multiclass JSON format (class as "Fighter/Wizard" not array)
- ✅ Enhanced spellcasting flexibility for multiclass characters
- ✅ Comprehensive validation for level distribution

### Phase 6: Combat Styles & Role-Based Backstories
- ✅ Added `class_levels` field to JSON for multiclass transparency
- ✅ Implemented `--melee` and `--ranged` fighting style flags
- ✅ Fighting style affects weapons, spells, subclass, and feat selection
- ✅ Default fighting styles (90% ranged for casters, 50/50 for martial)
- ✅ Role-based backstory system (adventurer vs working NPC)
- ✅ Combat roles (Mercenary, Soldier) get adventurer-focused backstories
- ✅ Non-combat roles (Farmer, Merchant) get profession-focused NPC backstories
- ✅ NPCs use class abilities in their profession (Farmer Druid, Blacksmith Fighter)

## Lessons Learned

### AI Integration
1. **Model Selection**: Qwen2.5 excels at structured JSON output and creative storytelling
2. **Temperature Matters**: Default temperature (0.8) was too low, causing repetitive outputs (all elves/wizards). Increasing to 1.2 dramatically improved variety.
3. **Prompt Engineering**: Being explicit about randomization requirements ("DO NOT default to elves/wizards") helped break patterns
4. **Flexible Schemas**: Using `#[serde(default)]` for optional fields prevents parsing failures when AI doesn't generate every field
5. **Constraint Handling**: AI needs clear "MUST follow" language when user specifies constraints
6. **Multiclass JSON Format**: AI initially generated arrays for multiclass (["Fighter", "Wizard"]) but data structure expected strings. Explicit format examples in prompt ("Fighter/Wizard") fixed this immediately.
7. **Spellcasting Flexibility**: Making Spellcasting struct fields optional with `#[serde(default)]` prevents crashes when AI generates incomplete spell data for multiclass characters

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
2. **Backstory Format**: AI sometimes condenses backstory into 1 paragraph instead of 3-5 (prompt could be refined)
3. **No Spell Validation**: Doesn't verify that selected spells are valid for the class/level
4. **Windows Path Handling**: Character names with special characters might cause file save issues
5. **Hit Dice Format**: Multiclass characters sometimes show simplified hit dice (e.g., "10d8") instead of detailed breakdown (e.g., "7d10 + 3d8") - cosmetic issue only

## Future Enhancements (TODOs)

### High Priority
- [x] Fix constrained generation parsing errors (COMPLETED: Fixed multiclass JSON format)
- [x] Add `.gitignore` file (COMPLETED)
- [x] Create comprehensive README.md (COMPLETED)
- [ ] Add example generated NPCs to repository
- [ ] Improve hit dice format for multiclass in AI prompt

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

**Session 3**: Multiclass system and JSON format fixes
- Implemented multiclass support (up to 3 classes)
- Added level range and role parameters
- Fixed multiclass JSON format (arrays → strings with "/" separator)
- Enhanced spellcasting flexibility with #[serde(default)]
- Manual level distribution (--lvl1, --lvl2, --lvl3)
- Comprehensive validation and error handling
- Updated documentation with multiclass examples

**Session 4**: Fighting styles and role-based backstories
- Added `class_levels` field to JSON for multiclass transparency
- Implemented `--melee` and `--ranged` combat style preferences
- Fighting styles affect weapons, spells, subclasses, and feats
- Role-based backstory system (adventurer vs working NPC)
- Combat roles get adventurer-focused backstories
- Non-combat roles get profession-focused NPC backstories
- NPCs use class abilities in their profession (not for adventuring)

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

### Vex Shadowstep (Multiclass)
- **Race**: Tiefling
- **Class**: Fighter/Rogue/Ranger (Battle Master/Thief/Beast Master), Level 10
- **Level Distribution**: 5 Fighter / 3 Rogue / 2 Ranger
- **Background**: Urchin
- **Alignment**: Chaotic Neutral
- **Highlights**: 3-class multiclass, manual level distribution, versatile skill set

### TestFarmer (Working NPC)
- **Race**: Goliath
- **Class**: Druid (Land's Wrath), Level 5
- **Role**: Farmer (non-combat)
- **Background**: Folk Hero
- **Alignment**: Neutral Good
- **Highlights**: Uses Druid magic to help crops grow, lives in a farming community, not an adventurer

### TestMerchant (Working NPC)
- **Race**: Tabaxi
- **Class**: Bard (Glamour), Level 4
- **Role**: Merchant (non-combat)
- **Background**: Guild Artisan
- **Alignment**: Chaotic Neutral
- **Highlights**: Uses Glamour magic to charm customers and boost sales, runs a shop in a small town

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

**Last Updated**: November 7, 2025
