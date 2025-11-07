use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

/// NPCForge - D&D 2024 NPC Generator using local AI
#[derive(Parser, Debug)]
#[command(name = "npcforge")]
#[command(about = "Generate D&D 2024 NPCs using Ollama AI", long_about = None)]
struct Args {
    /// Number of NPCs to generate (max 25)
    #[arg(short = 'n', long, default_value_t = 1)]
    count: u8,

    /// Specific name for the NPC (sets count to 1)
    #[arg(long)]
    name: Option<String>,

    /// Race for the NPC (e.g., "Dwarf", "Elf", "Dragonborn")
    #[arg(short, long)]
    race: Option<String>,

    /// Class for the NPC - can specify multiple comma-separated for multiclass (e.g., "Ranger,Druid")
    /// Maximum 3 classes allowed. If more are specified, only first 3 are used.
    #[arg(short, long)]
    class: Option<String>,

    /// Total level for the NPC (1-20, distributed across all classes in multiclass)
    #[arg(short, long)]
    level: Option<u8>,

    /// Levels in first class (for multiclass distribution)
    #[arg(long)]
    lvl1: Option<u8>,

    /// Levels in second class (for multiclass distribution)
    #[arg(long)]
    lvl2: Option<u8>,

    /// Levels in third class (for multiclass distribution)
    #[arg(long)]
    lvl3: Option<u8>,

    /// Minimum level for random generation (1-20)
    #[arg(long, default_value_t = 1)]
    low: u8,

    /// Maximum level for random generation (1-20)
    #[arg(long, default_value_t = 10)]
    high: u8,

    /// Alignment for the NPC (e.g., "CG", "LN", "CE")
    #[arg(short, long)]
    alignment: Option<String>,

    /// Role/occupation for the NPC (e.g., "Mercenary", "Scholar", "Pirate", or "random")
    #[arg(long, default_value = "Mercenary")]
    role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NPC {
    name: String,
    race: String,
    #[serde(rename = "class")]
    class_name: String,
    subclass: Option<String>,
    level: u8,
    background: String,
    alignment: String,

    // Ability Scores
    ability_scores: AbilityScores,

    // Combat Stats
    hit_points: HitPoints,
    armor_class: u8,
    initiative: i8,
    speed: u8,
    proficiency_bonus: i8,

    // Skills and Proficiencies
    #[serde(default)]
    skills: Vec<Skill>,
    #[serde(default)]
    saving_throws: Vec<String>,
    #[serde(default)]
    languages: Vec<String>,
    #[serde(default)]
    tool_proficiencies: Vec<String>,

    // Combat Abilities
    #[serde(default)]
    attacks: Vec<Attack>,
    spells: Option<Spellcasting>,

    // Equipment
    equipment: Equipment,

    // Character Details
    personality: Personality,
    backstory: String,
    appearance: Appearance,

    // Additional Features
    #[serde(default)]
    features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AbilityScores {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct HitPoints {
    max: u16,
    current: u16,
    temporary: u16,
    hit_dice: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Skill {
    name: String,
    modifier: i8,
    proficient: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Attack {
    name: String,
    attack_bonus: i8,
    damage: String,
    damage_type: String,
    range: Option<String>,
    #[serde(default)]
    properties: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Spellcasting {
    spellcasting_ability: String,
    spell_save_dc: u8,
    spell_attack_bonus: i8,
    spell_slots: Option<SpellSlots>,
    spells_known: SpellsByLevel,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpellSlots {
    #[serde(default)]
    level_1: u8,
    #[serde(default)]
    level_2: u8,
    #[serde(default)]
    level_3: u8,
    #[serde(default)]
    level_4: u8,
    #[serde(default)]
    level_5: u8,
    #[serde(default)]
    level_6: u8,
    #[serde(default)]
    level_7: u8,
    #[serde(default)]
    level_8: u8,
    #[serde(default)]
    level_9: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpellsByLevel {
    #[serde(default)]
    cantrips: Vec<String>,
    #[serde(default)]
    level_1: Vec<String>,
    #[serde(default)]
    level_2: Vec<String>,
    #[serde(default)]
    level_3: Vec<String>,
    #[serde(default)]
    level_4: Vec<String>,
    #[serde(default)]
    level_5: Vec<String>,
    #[serde(default)]
    level_6: Vec<String>,
    #[serde(default)]
    level_7: Vec<String>,
    #[serde(default)]
    level_8: Vec<String>,
    #[serde(default)]
    level_9: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Equipment {
    #[serde(default)]
    armor: Vec<String>,
    #[serde(default)]
    weapons: Vec<String>,
    #[serde(default)]
    gear: Vec<String>,
    treasure: Treasure,
}

#[derive(Debug, Serialize, Deserialize)]
struct Treasure {
    #[serde(default)]
    gold: u32,
    #[serde(default)]
    items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Personality {
    #[serde(default)]
    traits: Vec<String>,
    ideals: String,
    bonds: String,
    flaws: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Appearance {
    age: u16,
    height: String,
    weight: String,
    eyes: String,
    hair: String,
    skin: String,
    #[serde(default)]
    distinguishing_features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Feature {
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaOptions {
    temperature: f32,
    top_p: f32,
    top_k: i32,
    num_predict: i32,  // Maximum number of tokens to generate
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

async fn generate_npc_with_ollama(
    name: Option<&str>,
    race: Option<&str>,
    class: Option<&str>,
    level: Option<u8>,
    level_distribution: Option<Vec<u8>>, // e.g., [3, 5, 2] for Fighter 3/Wizard 5/Cleric 2
    level_range: Option<(u8, u8)>,
    alignment: Option<&str>,
    role: &str,
) -> Result<NPC> {
    // Create client with extended timeout for AI generation
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(600)) // 10 minute timeout
        .build()
        .context("Failed to create HTTP client")?;

    let prompt = create_npc_generation_prompt(name, race, class, level, level_distribution, level_range, alignment, role);

    let request = OllamaRequest {
        model: "qwen2.5:32b-instruct".to_string(),
        prompt,
        stream: false,
        format: "json".to_string(),
        options: Some(OllamaOptions {
            temperature: 1.2,  // Higher temperature for more randomness (default is 0.8)
            top_p: 0.95,       // Nucleus sampling
            top_k: 50,         // Top-k sampling for variety
            num_predict: 4096, // Increase max tokens to prevent truncation
        }),
    };

    println!("Generating NPC with Ollama...");
    println!("This may take a minute or two...\n");

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&request)
        .send()
        .await
        .context("Failed to connect to Ollama. Is it running?")?;

    let response_text = response
        .text()
        .await
        .context("Failed to read response text")?;

    let ollama_response: OllamaResponse = serde_json::from_str(&response_text)
        .context(format!("Failed to parse Ollama response. Response was: {}",
            &response_text[..response_text.len().min(500)]))?;

    let npc: NPC = serde_json::from_str(&ollama_response.response)
        .context(format!("Failed to parse NPC JSON. Response was: {}",
            &ollama_response.response[..ollama_response.response.len().min(1000)]))?;

    Ok(npc)
}

fn create_npc_generation_prompt(
    name: Option<&str>,
    race: Option<&str>,
    class: Option<&str>,
    level: Option<u8>,
    level_distribution: Option<Vec<u8>>,
    level_range: Option<(u8, u8)>,
    alignment: Option<&str>,
    role: &str,
) -> String {
    let mut prompt = String::from(
        "You are a D&D 2024 character generator. Generate a complete, TRULY RANDOM D&D character with maximum variety and creativity.\n\n"
    );

    // Parse classes and limit to 3
    let classes: Vec<&str> = class.map(|c| c.split(',').take(3).collect()).unwrap_or_default();
    let is_multiclass = classes.len() > 1;

    // Add user constraints if provided
    let has_constraints = name.is_some() || race.is_some() || class.is_some() || level.is_some() || level_distribution.is_some() || level_range.is_some() || alignment.is_some() || role != "Mercenary";

    if has_constraints {
        prompt.push_str("USER CONSTRAINTS (MUST follow these exactly):\n");
        if let Some(n) = name {
            prompt.push_str(&format!("- Name MUST be: {}\n", n));
        }
        if let Some(r) = race {
            prompt.push_str(&format!("- Race MUST be: {}\n", r));
        }
        if let Some(_c) = class {
            if is_multiclass {
                let class_names = classes.join(", ");
                prompt.push_str(&format!("- Classes MUST be (multiclass): {}\n", class_names));
                prompt.push_str("  - This is a MULTICLASS character with multiple classes\n");

                if let Some(dist) = &level_distribution {
                    // Manual level distribution specified
                    prompt.push_str("  - Level distribution:\n");
                    for (i, (class_name, class_level)) in classes.iter().zip(dist.iter()).enumerate() {
                        if i < classes.len() {
                            prompt.push_str(&format!("    * {} {} levels\n", class_name.trim(), class_level));
                        }
                    }
                    let total: u8 = dist.iter().sum();
                    prompt.push_str(&format!("  - Total character level: {}\n", total));
                } else if let Some(l) = level {
                    // Total level specified, distribute randomly
                    prompt.push_str(&format!("  - TOTAL level is {} - distribute these {} levels across the {} classes\n", l, l, classes.len()));
                    prompt.push_str("  - Example distributions you could use:\n");
                    if classes.len() == 2 && l >= 2 {
                        prompt.push_str(&format!("    * {}: {}, {}: {} OR {}: {}, {}: {}, etc.\n",
                            classes[0].trim(), l-1,
                            classes[1].trim(), 1,
                            classes[0].trim(), l/2,
                            classes[1].trim(), l - l/2));
                    } else if classes.len() == 3 && l >= 3 {
                        prompt.push_str(&format!("    * {}: {}, {}: {}, {}: {}, etc.\n",
                            classes[0].trim(), l/3,
                            classes[1].trim(), l/3,
                            classes[2].trim(), l - 2*(l/3)));
                    }
                } else {
                    prompt.push_str("  - Choose a total level and distribute it randomly across classes\n");
                }
            } else {
                prompt.push_str(&format!("- Class MUST be: {}\n", classes[0]));
            }
        }

        // Level specification (only if not already specified above in multiclass distribution)
        if level_distribution.is_none() {
            if let Some(l) = level {
                if !is_multiclass {
                    prompt.push_str(&format!("- Level MUST be: {}\n", l));
                }
            } else if let Some((low, high)) = level_range {
                prompt.push_str(&format!("- Level MUST be between {} and {} (inclusive)\n", low, high));
            }
        }
        if let Some(a) = alignment {
            prompt.push_str(&format!("- Alignment MUST be: {}\n", a));
        }
        if role.to_lowercase() != "mercenary" {
            if role.to_lowercase() == "random" {
                prompt.push_str("- Role/Occupation MUST be randomly selected\n");
            } else {
                prompt.push_str(&format!("- Role/Occupation MUST be: {}\n", role));
            }
        }
        prompt.push_str("\n");
    }

    prompt.push_str(r#"IMPORTANT: Be EXTREMELY VARIED in your choices! Avoid patterns and defaults!
- DO NOT default to elves, wizards, or common combinations
- Mix unusual race/class combinations (Dragonborn Bard, Half-Orc Wizard, Tiefling Paladin, etc.)
- Vary genders, alignments, backgrounds, and personality types significantly
- Create diverse and unique characters each time

Requirements:
"#);

    // Add race requirement (random or constrained)
    if race.is_some() {
        prompt.push_str(&format!("- Use the specified race: {}\n", race.unwrap()));
    } else {
        prompt.push_str("- Choose a COMPLETELY RANDOM race from ALL official D&D races (Human, Elf, Dwarf, Halfling, Dragonborn, Gnome, Half-Elf, Half-Orc, Tiefling, Aasimar, Firbolg, Goliath, Kenku, Tabaxi, Triton, Genasi, Bugbear, Goblin, Hobgoblin, Kobold, Orc, Yuan-ti, Lizardfolk, etc.)\n");
    }

    // Add class requirement (random or constrained)
    if let Some(c) = class {
        if is_multiclass {
            prompt.push_str(&format!("- Use the specified MULTICLASS: {} and choose appropriate subclasses for each class\n", c));
            prompt.push_str("  - This is a multiclass character - split levels appropriately\n");
            prompt.push_str("  - Choose compatible ability scores and features for both classes\n");
        } else {
            prompt.push_str(&format!("- Use the specified class: {} and choose an appropriate subclass\n", c));
        }
    } else {
        prompt.push_str("- Choose a COMPLETELY RANDOM class from ALL official D&D classes (Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue, Sorcerer, Warlock, Wizard, Artificer) and appropriate subclass\n");
        prompt.push_str("  - 10% chance: Make this a MULTICLASS character with 2 classes (e.g., Fighter/Rogue, Paladin/Warlock, etc.)\n");
    }

    // Add level requirement (random or constrained)
    if let Some(l) = level {
        prompt.push_str(&format!("- Use the specified level: {}\n", l));
    } else if let Some((low, high)) = level_range {
        prompt.push_str(&format!("- Choose a random level between {} and {} (inclusive)\n", low, high));
    } else {
        prompt.push_str("- Choose a random level between 1-20\n");
    }

    // Add alignment requirement (random or constrained)
    if let Some(a) = alignment {
        prompt.push_str(&format!("- Use the specified alignment: {}\n", a));
    } else {
        prompt.push_str("- Choose a RANDOM alignment (Lawful Good, Neutral Good, Chaotic Good, Lawful Neutral, True Neutral, Chaotic Neutral, Lawful Evil, Neutral Evil, Chaotic Evil)\n");
    }

    // Add role requirement
    if role.to_lowercase() == "random" {
        prompt.push_str("- Choose a RANDOM role/occupation from: Soldier, Mercenary, Pirate, Merchant, Scholar, Academic, Builder, Craftsman, Farmer, Laborer, Vagrant, Beggar, Noble, Aristocrat, Statesman, Diplomat, Musician, Bard, Traveling Performer, Entertainer, Sailor, Guard, Adventurer, Explorer, Hermit, Mystic, etc.\n");
    } else if role.to_lowercase() != "mercenary" {
        prompt.push_str(&format!("- Role/Occupation MUST be: {}\n", role));
        prompt.push_str("  - Integrate this role into the backstory and personality\n");
    } else {
        // Default: Mercenary
        prompt.push_str("- Role/Occupation: Mercenary (fighter for hire)\n");
        prompt.push_str("  - This character makes their living through combat and protection services\n");
    }

    prompt.push_str("- Choose a RANDOM background (Acolyte, Charlatan, Criminal, Entertainer, Folk Hero, Guild Artisan, Hermit, Noble, Outlander, Sage, Sailor, Soldier, Urchin, etc.)\n");
    prompt.push_str("- Generate appropriate ability scores (use standard array or point buy)\n");
    prompt.push_str("- Calculate all derived stats correctly (AC, HP, initiative, proficiency bonus, etc.)\n");
    prompt.push_str("- Include all relevant skills, proficiencies, and saving throws\n");
    prompt.push_str("- For spellcasters, include appropriate spells based on class and level\n");
    prompt.push_str("- Include attacks and combat abilities\n");
    prompt.push_str("- Generate realistic equipment based on class and level\n");
    prompt.push_str("- Create a DETAILED and COMPREHENSIVE backstory (3-5 paragraphs) that includes:\n");
    prompt.push_str("  * Childhood: Family, upbringing, early life experiences\n");
    prompt.push_str("  * Education: Training, mentors, how they learned their skills\n");
    prompt.push_str("  * Life events: Major events, adventures, tragedies, triumphs\n");
    prompt.push_str("  * Relationships: Important people (family, friends, rivals, mentors, lovers)\n");
    prompt.push_str("  * Personality: Likes, dislikes, hobbies, quirks\n");
    prompt.push_str("  * Current situation: What brought them to where they are now in life\n");
    prompt.push_str("- Include personality traits, ideals, bonds, and flaws\n");
    prompt.push_str("- Create a vivid physical appearance\n\n");

    prompt.push_str(r#"Output ONLY valid JSON matching this exact structure (no additional text):

{
  "name": "Full character name",
  "race": "Character race",
  "class": "Character class",
  "subclass": "Character subclass or null",
  "level": 10,
  "background": "Background name",
  "alignment": "Alignment",
  "ability_scores": {
    "strength": 10,
    "dexterity": 14,
    "constitution": 12,
    "intelligence": 16,
    "wisdom": 13,
    "charisma": 8
  },
  "hit_points": {
    "max": 65,
    "current": 65,
    "temporary": 0,
    "hit_dice": "10d8"
  },
  "armor_class": 15,
  "initiative": 2,
  "speed": 30,
  "proficiency_bonus": 4,
  "skills": [
    {"name": "Arcana", "modifier": 7, "proficient": true},
    {"name": "Investigation", "modifier": 7, "proficient": true}
  ],
  "saving_throws": ["Intelligence", "Wisdom"],
  "languages": ["Common", "Elvish"],
  "tool_proficiencies": ["Alchemist's Supplies"],
  "attacks": [
    {
      "name": "Quarterstaff",
      "attack_bonus": 4,
      "damage": "1d6+0",
      "damage_type": "bludgeoning",
      "range": "Melee",
      "properties": ["Versatile"]
    }
  ],
  "spells": {
    "spellcasting_ability": "Intelligence",
    "spell_save_dc": 15,
    "spell_attack_bonus": 7,
    "spell_slots": {
      "level_1": 4,
      "level_2": 3,
      "level_3": 3,
      "level_4": 3,
      "level_5": 2,
      "level_6": 0,
      "level_7": 0,
      "level_8": 0,
      "level_9": 0
    },
    "spells_known": {
      "cantrips": ["Fire Bolt", "Mage Hand", "Prestidigitation"],
      "level_1": ["Magic Missile", "Shield", "Detect Magic"],
      "level_2": ["Misty Step", "Scorching Ray"],
      "level_3": ["Fireball", "Counterspell"],
      "level_4": ["Greater Invisibility"],
      "level_5": ["Wall of Force"],
      "level_6": [],
      "level_7": [],
      "level_8": [],
      "level_9": []
    }
  },
  "equipment": {
    "armor": ["Studded Leather Armor"],
    "weapons": ["Quarterstaff", "Dagger"],
    "gear": ["Spellbook", "Component Pouch", "Backpack", "Bedroll", "Rations"],
    "treasure": {
      "gold": 250,
      "items": ["Potion of Healing", "Spell Scroll (Identify)"]
    }
  },
  "personality": {
    "traits": ["Curious about everything", "Speaks in elaborate metaphors"],
    "ideals": "Knowledge is the path to power and domination",
    "bonds": "I seek to preserve ancient magical texts",
    "flaws": "I am easily distracted by the promise of new knowledge"
  },
  "backstory": "A comprehensive backstory covering childhood (family, upbringing, formative experiences), education (training, mentors, how they developed their skills), major life events (adventures, tragedies, triumphs), important relationships (family, friends, rivals, mentors, romantic interests), personality details (likes, dislikes, hobbies, quirks), and their current situation (what brought them to where they are now). This should be 3-5 detailed paragraphs that paint a vivid picture of their entire life journey.",
  "appearance": {
    "age": 127,
    "height": "5'8\"",
    "weight": "140 lbs",
    "eyes": "Amber",
    "hair": "Silver",
    "skin": "Pale",
    "distinguishing_features": ["Arcane tattoos on arms", "Singed eyebrows"]
  },
  "features": [
    {
      "name": "Arcane Recovery",
      "description": "Once per day during a short rest, you can recover expended spell slots..."
    }
  ]
}

Generate a completely random character now:"#);

    prompt
}

fn save_npc_to_file(npc: &NPC, index: Option<usize>) -> Result<String> {
    let base_name = npc.name.replace(" ", "_");
    let filename = if let Some(idx) = index {
        format!("{}_{}.json", base_name, idx)
    } else {
        format!("{}.json", base_name)
    };

    let json = serde_json::to_string_pretty(npc)
        .context("Failed to serialize NPC to JSON")?;

    fs::write(&filename, json)
        .context(format!("Failed to write to file: {}", filename))?;

    Ok(filename)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = Args::parse();

    println!("=== NPCForge - D&D 2024 NPC Generator ===\n");

    // If name is specified, set count to 1
    if args.name.is_some() {
        args.count = 1;
    }

    // Limit count to 25
    if args.count > 25 {
        eprintln!("✗ Error: Count limited to 25 NPCs maximum");
        args.count = 25;
        println!("  Setting count to 25\n");
    }

    // Parse and validate classes (limit to 3)
    let classes: Vec<&str> = args.class.as_ref()
        .map(|c| c.split(',').take(3).collect())
        .unwrap_or_default();
    let num_classes = classes.len();

    if num_classes > 3 {
        println!("⚠ Warning: More than 3 classes specified. Using only first 3: {}", classes.join(", "));
    }

    // Build level distribution vector if lvl1/lvl2/lvl3 are specified
    let level_distribution = if args.lvl1.is_some() || args.lvl2.is_some() || args.lvl3.is_some() {
        let mut dist = Vec::new();
        if let Some(l1) = args.lvl1 {
            if l1 < 1 || l1 > 20 {
                eprintln!("✗ Error: --lvl1 must be between 1 and 20");
                return Ok(());
            }
            dist.push(l1);
        }
        if let Some(l2) = args.lvl2 {
            if l2 < 1 || l2 > 20 {
                eprintln!("✗ Error: --lvl2 must be between 1 and 20");
                return Ok(());
            }
            dist.push(l2);
        }
        if let Some(l3) = args.lvl3 {
            if l3 < 1 || l3 > 20 {
                eprintln!("✗ Error: --lvl3 must be between 1 and 20");
                return Ok(());
            }
            dist.push(l3);
        }

        // Validate that distribution matches number of classes
        if num_classes > 0 && dist.len() != num_classes {
            eprintln!("✗ Error: Level distribution count ({}) doesn't match class count ({})",
                dist.len(), num_classes);
            eprintln!("  Classes: {}", classes.join(", "));
            return Ok(());
        }

        // Validate total level
        let total_level: u8 = dist.iter().sum();
        if total_level < 1 || total_level > 20 {
            eprintln!("✗ Error: Total level from distribution ({}) must be between 1 and 20", total_level);
            return Ok(());
        }

        // If explicit level was specified, validate it matches
        if let Some(level) = args.level {
            if total_level != level {
                eprintln!("✗ Error: Level distribution sum ({}) doesn't match specified level ({})",
                    total_level, level);
                return Ok(());
            }
        }

        Some(dist)
    } else {
        None
    };

    // Validate level if specified
    if let Some(level) = args.level {
        if level < 1 || level > 20 {
            eprintln!("✗ Error: Level must be between 1 and 20");
            return Ok(());
        }
    }

    // Validate level range
    if args.low < 1 || args.low > 20 {
        eprintln!("✗ Error: --low must be between 1 and 20");
        return Ok(());
    }
    if args.high < 1 || args.high > 20 {
        eprintln!("✗ Error: --high must be between 1 and 20");
        return Ok(());
    }
    if args.low > args.high {
        eprintln!("✗ Error: --low ({}) cannot be greater than --high ({})", args.low, args.high);
        return Ok(());
    }

    // Calculate level range (only if level is not explicitly specified)
    let level_range = if args.level.is_none() && level_distribution.is_none() && (args.low != 1 || args.high != 10) {
        Some((args.low, args.high))
    } else {
        None
    };

    println!("Generating {} NPC(s)...\n", args.count);

    let mut success_count = 0;
    let mut failed_count = 0;

    for i in 1..=args.count {
        if args.count > 1 {
            println!("--- Generating NPC {}/{} ---", i, args.count);
        }

        let result = generate_npc_with_ollama(
            args.name.as_deref(),
            args.race.as_deref(),
            args.class.as_deref(),
            args.level,
            level_distribution.clone(),
            level_range,
            args.alignment.as_deref(),
            &args.role,
        )
        .await;

        match result {
            Ok(npc) => {
                println!("✓ Successfully generated NPC: {}", npc.name);
                println!("  Race: {}", npc.race);
                println!("  Class: {} (Level {})", npc.class_name, npc.level);
                if let Some(subclass) = &npc.subclass {
                    println!("  Subclass: {}", subclass);
                }
                println!("  Background: {}", npc.background);
                println!("  Alignment: {}", npc.alignment);

                let index = if args.count > 1 { Some(i as usize) } else { None };
                match save_npc_to_file(&npc, index) {
                    Ok(filename) => {
                        println!("✓ Saved to: {}", filename);
                        success_count += 1;
                    }
                    Err(e) => {
                        eprintln!("✗ Error saving file: {}", e);
                        failed_count += 1;
                    }
                }
                println!();
            }
            Err(e) => {
                eprintln!("✗ Error generating NPC {}: {:#}", i, e);
                if i == 1 {
                    eprintln!("\nMake sure Ollama is running and you have the model installed:");
                    eprintln!("  ollama pull qwen2.5:32b-instruct");
                }
                failed_count += 1;
                println!();
            }
        }

        // Small delay between requests to prevent overwhelming Ollama
        if i < args.count {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }

    println!("=== Summary ===");
    println!("Successfully generated: {}", success_count);
    if failed_count > 0 {
        println!("Failed: {}", failed_count);
    }

    Ok(())
}
