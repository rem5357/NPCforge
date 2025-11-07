# NPCForge Usage Guide

Complete reference for all command-line parameters, available values, defaults, and usage examples.

**Version**: 0.1.0
**Last Updated**: November 7, 2025

---

## Table of Contents
- [Quick Start](#quick-start)
- [Command-Line Parameters](#command-line-parameters)
- [Parameter Details](#parameter-details)
- [Example Commands](#example-commands)
- [Tips & Best Practices](#tips--best-practices)

---

## Quick Start

```bash
# Generate one random NPC
cargo run

# Generate a specific character
cargo run -- --name "Thorin" -c "Fighter" -l 8 -a "CG" --melee
```

---

## Command-Line Parameters

| Parameter | Short | Type | Default | Description |
|-----------|-------|------|---------|-------------|
| `--name` | - | String | Random | Specific name for the NPC |
| `--count` | `-n` | Number | 1 | Number of NPCs to generate (max 25) |
| `--race` | `-r` | String | Random | Character race |
| `--class` | `-c` | String | Random | Character class(es) |
| `--level` | `-l` | Number | Random | Total character level (1-20) |
| `--lvl1` | - | Number | - | Levels in first class (multiclass) |
| `--lvl2` | - | Number | - | Levels in second class (multiclass) |
| `--lvl3` | - | Number | - | Levels in third class (multiclass) |
| `--low` | - | Number | 1 | Minimum level for random generation |
| `--high` | - | Number | 10 | Maximum level for random generation |
| `--alignment` | `-a` | String | Random | Character alignment |
| `--role` | - | String | Mercenary | Character occupation/role |
| `--melee` | - | Flag | Off | Prefer melee combat style |
| `--ranged` | - | Flag | Off | Prefer ranged combat style |

---

## Parameter Details

### `--name <NAME>`
**Purpose**: Set a specific name for the character
**Default**: Randomly generated
**Behavior**: Automatically sets `--count` to 1

**Examples**:
```bash
--name "Thorin Oakenshield"
--name "Aria"
--name "Grommash"
```

---

### `-n, --count <NUMBER>`
**Purpose**: Generate multiple NPCs at once
**Default**: 1
**Range**: 1-25
**Note**: Overridden to 1 if `--name` is specified

**Examples**:
```bash
-n 5          # Generate 5 NPCs
--count 10    # Generate 10 NPCs
```

---

### `-r, --race <RACE>`
**Purpose**: Specify character race
**Default**: Random
**Available Races** (non-exhaustive):
- Common: Human, Elf, Dwarf, Halfling, Gnome, Half-Elf, Half-Orc
- Exotic: Dragonborn, Tiefling, Aasimar, Genasi
- Uncommon: Goliath, Firbolg, Kenku, Tabaxi, Triton
- Monstrous: Goblin, Hobgoblin, Kobold, Orc, Yuan-ti, Lizardfolk, Bugbear

**Examples**:
```bash
-r "Dwarf"
-r "Tiefling"
-r "Tabaxi"
--race "Dragonborn"
```

---

### `-c, --class <CLASS>`
**Purpose**: Specify character class(es)
**Default**: Random
**Format**: Single class or comma-separated for multiclass (max 3)
**Available Classes**:
- Barbarian, Bard, Cleric, Druid, Fighter, Monk
- Paladin, Ranger, Rogue, Sorcerer, Warlock, Wizard, Artificer

**Single Class**:
```bash
-c "Fighter"
-c "Wizard"
--class "Ranger"
```

**Multiclass** (max 3 classes):
```bash
-c "Fighter,Wizard"           # 2 classes
-c "Fighter,Rogue,Ranger"     # 3 classes
-c "Paladin,Warlock"          # 2 classes
```

**Note**: Subclass is chosen automatically by AI based on class, level, and fighting style.

---

### `-l, --level <LEVEL>`
**Purpose**: Specify total character level
**Default**: Random (between `--low` and `--high`)
**Range**: 1-20
**Multiclass Note**: This is the TOTAL level distributed across all classes

**Examples**:
```bash
-l 5          # Level 5 character
-l 15         # Level 15 character
--level 20    # Max level character
```

---

### `--lvl1, --lvl2, --lvl3 <LEVEL>`
**Purpose**: Manual level distribution for multiclass characters
**Default**: AI distributes automatically
**Requirements**:
- Must match number of classes in `--class`
- Sum must equal total character level (if `-l` is also specified)
- Each value must be 1-20

**Examples**:
```bash
# Fighter 6 / Wizard 4 (total level 10)
-c "Fighter,Wizard" --lvl1 6 --lvl2 4

# Paladin 7 / Warlock 3 (total level 10)
-c "Paladin,Warlock" --lvl1 7 --lvl2 3

# Fighter 5 / Rogue 3 / Ranger 2 (total level 10)
-c "Fighter,Rogue,Ranger" --lvl1 5 --lvl2 3 --lvl3 2

# With explicit total level (must match)
-c "Fighter,Wizard" -l 10 --lvl1 6 --lvl2 4
```

---

### `--low <LEVEL>` and `--high <LEVEL>`
**Purpose**: Set level range for random generation
**Defaults**: `--low 1`, `--high 10`
**Range**: 1-20 for both
**Note**: Only used when `--level` is NOT specified

**Examples**:
```bash
# Generate NPCs between level 5-10
--low 5 --high 10

# Generate high-level NPCs (level 15-20)
--low 15 --high 20

# Generate low-level NPCs (level 1-3)
--low 1 --high 3
```

---

### `-a, --alignment <ALIGNMENT>`
**Purpose**: Specify character alignment
**Default**: Random
**Available Alignments**:
- **Lawful**: LG (Lawful Good), LN (Lawful Neutral), LE (Lawful Evil)
- **Neutral**: NG (Neutral Good), TN (True Neutral), NE (Neutral Evil)
- **Chaotic**: CG (Chaotic Good), CN (Chaotic Neutral), CE (Chaotic Evil)

**Examples**:
```bash
-a "CG"       # Chaotic Good
-a "LN"       # Lawful Neutral
-a "NE"       # Neutral Evil
--alignment "Lawful Good"  # Full name also works
```

---

### `--role <ROLE>`
**Purpose**: Specify character's occupation/role
**Default**: Mercenary
**Special Value**: "random" (picks randomly)
**Behavior**:
- **Combat roles** (Mercenary, Soldier, Guard, Pirate, Adventurer): Get adventurer-focused backstories
- **Non-combat roles** (Farmer, Scholar, Merchant, etc.): Get profession-focused NPC backstories

**Available Roles**:
- **Combat**: Mercenary, Soldier, Guard, Pirate, Bandit, Adventurer, Gladiator
- **Trade**: Merchant, Trader, Shopkeeper, Guild Artisan
- **Labor**: Farmer, Builder, Craftsman, Laborer, Blacksmith
- **Social**: Noble, Aristocrat, Statesman, Diplomat
- **Knowledge**: Scholar, Academic, Sage, Teacher
- **Arts**: Musician, Bard, Entertainer, Performer
- **Other**: Sailor, Hermit, Mystic, Vagrant, Beggar, Explorer

**Examples**:
```bash
--role "Farmer"
--role "Scholar"
--role "Pirate"
--role "random"       # Picks randomly
```

---

### `--melee` and `--ranged`
**Purpose**: Specify fighting style preference
**Default**: Neither (random based on class)
**Type**: Boolean flags (no value needed)
**Effects**:
- Weapon selection (melee vs ranged weapons)
- Spell selection (touch/close-range vs long-range)
- Subclass selection (melee-focused vs ranged-focused)
- Feat selection
- **Ability score optimization**

**Ability Score Optimization**:
- **`--melee`**: High STR (16+) or DEX (16+), high CON (14-16), dump CHA/INT
- **`--ranged`**: High DEX (16-18), good WIS (12-14), dump STR
- **Both flags**: Balanced STR/DEX (14-16), solid CON (14+)
- **Neither**: Random (90% ranged for casters, 50/50 for martial)

**Examples**:
```bash
--melee              # Melee-focused build
--ranged             # Ranged-focused build
--melee --ranged     # Versatile (both styles)
# (neither)          # Random fighting style
```

**Output**: Sets `"fighting_preference"` field in JSON to "Melee", "Ranged", or "Versatile"

---

## Example Commands

### Example 1: Simple Random NPC
```bash
cargo run
```
**Result**: One random NPC, level 1-10, random race/class/alignment

---

### Example 2: Named Character with Specific Stats
```bash
cargo run -- --name "Thorin Ironforge" -c "Fighter" -l 8 -a "LG" -r "Dwarf"
```
**Result**: Thorin Ironforge, Dwarf Fighter, level 8, Lawful Good

---

### Example 3: Melee-Focused Fighter
```bash
cargo run -- -c "Fighter" -l 10 --melee --name "Brutus"
```
**Result**: Brutus, Fighter level 10, melee weapons (Greatsword, Battleaxe), high STR/CON

---

### Example 4: Ranged Wizard Scholar
```bash
cargo run -- -c "Wizard" -l 12 --ranged --role "Scholar" --name "Elara"
```
**Result**: Elara, Wizard level 12, scholar (non-adventurer), long-range spells, high INT/DEX

---

### Example 5: Multiclass with Automatic Distribution
```bash
cargo run -- -c "Fighter,Wizard" -l 10 --melee --name "Gale"
```
**Result**: Gale, Fighter/Wizard level 10, AI distributes levels (e.g., Fighter 6/Wizard 4)

---

### Example 6: Multiclass with Manual Distribution
```bash
cargo run -- -c "Paladin,Warlock" --lvl1 7 --lvl2 3 --name "Zariel"
```
**Result**: Zariel, Paladin 7/Warlock 3, exact level split

---

### Example 7: Three-Class Multiclass
```bash
cargo run -- -c "Fighter,Rogue,Ranger" --lvl1 5 --lvl2 3 --lvl3 2 --ranged
```
**Result**: Fighter 5/Rogue 3/Ranger 2, ranged focus

---

### Example 8: Batch Generation with Level Range
```bash
cargo run -- -n 5 --low 5 --high 10 -c "Rogue"
```
**Result**: 5 Rogues, levels 5-10 (random within range)

---

### Example 9: Working NPC (Non-Adventurer)
```bash
cargo run -- --role "Farmer" -c "Druid" -l 5 --name "Milo"
```
**Result**: Milo, Druid level 5, farmer who uses magic to help crops (not an adventurer)

---

### Example 10: Versatile Ranger
```bash
cargo run -- -c "Ranger" -l 7 --melee --ranged --name "Strider"
```
**Result**: Strider, Ranger level 7, both melee (Shortsword) and ranged (Longbow) weapons

---

## Tips & Best Practices

### Naming
- Use quotes for names with spaces: `--name "Sir Galahad"`
- Single-word names don't need quotes: `--name Thorin`

### Multiclass
- **Automatic distribution**: Just specify classes and level, let AI distribute
  ```bash
  -c "Fighter,Wizard" -l 10
  ```
- **Manual distribution**: Use `--lvl1`, `--lvl2`, `--lvl3` for precise control
  ```bash
  -c "Fighter,Wizard" --lvl1 6 --lvl2 4
  ```
- **Limit**: Maximum 3 classes, extras are ignored

### Fighting Styles
- Use `--melee` for tanks, bruisers, heavy armor users
- Use `--ranged` for archers, artillery casters, snipers
- Use both flags for versatile characters (Rangers, Paladins)
- Omit both for random (good for variety)

### Ability Scores
- Fighting style directly affects ability score allocation
- **Melee builds**: Expect high STR/DEX and CON
- **Ranged builds**: Expect high DEX, dumped STR
- **Caster builds**: Casting stat prioritized, then optimized by fighting style

### Roles
- Use combat roles (Mercenary, Soldier) for traditional adventurers
- Use non-combat roles (Farmer, Scholar) for town NPCs
- Role affects backstory significantly
- Role appears in JSON output

### Batch Generation
- Use `-n` for multiple NPCs
- Combine with `--low`/`--high` for level variety
- Each NPC is saved as separate JSON file
- Files are numbered: `Name_1.json`, `Name_2.json`, etc.

### Level Ranges
- Default range (1-10) good for most campaigns
- Use `--low 5 --high 10` for experienced parties
- Use `--low 15 --high 20` for epic-level campaigns
- Use `--low 1 --high 3` for starting NPCs

### Output
- NPCs saved as JSON files in current directory
- Filename based on character name
- Use `-n` for batch generation with numbered files

---

## Common Use Cases

### Generate a Boss NPC
```bash
cargo run -- --name "Lord Vexus" -c "Fighter,Wizard" --lvl1 8 --lvl2 7 -a "LE" --melee
```

### Generate a Shopkeeper
```bash
cargo run -- --role "Merchant" -c "Bard" -l 5 --name "Pip Goodbarrel"
```

### Generate Party Members
```bash
# Tank
cargo run -- -c "Fighter" -l 5 --melee --name "Tank"

# Healer
cargo run -- -c "Cleric" -l 5 --ranged --name "Healer"

# DPS
cargo run -- -c "Rogue" -l 5 --ranged --name "DPS"

# Caster
cargo run -- -c "Wizard" -l 5 --ranged --name "Caster"
```

### Generate Random Encounters
```bash
# 5 random bandits, levels 3-5
cargo run -- -n 5 --low 3 --high 5 --role "Bandit"
```

---

## Need Help?

Run the built-in help command:
```bash
cargo run -- --help
```

For more information, see:
- [README.md](README.md) - Project overview
- [Project.md](Project.md) - Complete documentation
- [GitHub Issues](https://github.com/rem5357/NPCforge/issues) - Report bugs or request features

---

**Version**: 0.1.0
**Last Updated**: November 7, 2025
