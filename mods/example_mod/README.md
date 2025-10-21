# Example Mod - New Guards

This is an example mod for **Path of Goodies** that demonstrates how to create custom content.

## What This Mod Does

Adds two new guard types to the game:
- **Elite Archer** - High-tier archer with piercing shots
- **Heavy Crossbowman** - Slow but powerful ranged guard

## Installation

### Automatic (In-Game)
1. Open Path of Goodies
2. Go to Settings → Mods
3. Click "Browse Mods"
4. Search for "Example Mod"
5. Click "Subscribe" or "Install"

### Manual
1. Download this mod
2. Extract to `path-of-goodies/mods/example_mod/`
3. Launch the game
4. Go to Settings → Mods
5. Enable "Example Mod"
6. Restart or hot-reload

## File Structure

```
example_mod/
├── mod.ron                 # Mod metadata (required)
├── README.md               # This file
├── data/                   # Game data files
│   └── guards.ron         # New guard definitions
└── assets/                 # Asset files
    └── sprites/
        └── guards/
            ├── elite_archer.png
            └── crossbowman.png
```

## How It Works

### 1. Mod Metadata (`mod.ron`)

This file defines your mod's identity and requirements:

```ron
(
    id: "example_mod",              // Unique mod ID
    name: "Example Mod - New Guards",
    version: "1.0.0",
    author: "Your Name",

    game_version: "0.1.0",          // Compatible game versions

    modifies: ["guards", "sprites"], // What you're changing
    priority: 100,                   // Load order
)
```

### 2. Data Files (`data/guards.ron`)

Defines new game content using RON (Rusty Object Notation):

```ron
(
    guards: [
        (
            id: "elite_archer",
            name: "Elite Archer",
            stats: (
                health: 100.0,
                damage: 18.0,
                // ...
            ),
            // ...
        ),
    ],
)
```

### 3. Asset Files

Place sprites in `assets/sprites/guards/`:
- `elite_archer.png` (32x32 pixels, pixel art)
- `crossbowman.png` (32x32 pixels, pixel art)

## Creating Your Own Mod

### Step 1: Create Folder Structure

```bash
cd path-of-goodies/mods/
mkdir my_mod
cd my_mod
mkdir -p data assets/sprites
```

### Step 2: Create `mod.ron`

```ron
(
    id: "my_mod",
    name: "My Awesome Mod",
    version: "1.0.0",
    author: "YourName",
    description: "Description of what your mod does",
    game_version: "0.1.0",
    dependencies: [],
    modifies: ["guards"],  // What you're modifying
    priority: 100,
    tags: ["gameplay"],
    license: "MIT",
)
```

### Step 3: Add Content

Create data files in `data/`:
- `guards.ron` - New guard types
- `enemies.ron` - New enemy types
- `items.ron` - New items
- `contracts.ron` - New delivery missions
- `biomes.ron` - New terrain types

### Step 4: Add Assets

Place your sprites/audio in `assets/`:
```
assets/
├── sprites/
│   ├── guards/
│   ├── enemies/
│   └── items/
└── audio/
    ├── music/
    └── sfx/
```

### Step 5: Test Your Mod

1. Launch game in dev mode: `cargo run --features dev`
2. Hot-reload is enabled - changes appear instantly
3. Check console for errors
4. Test in-game

## Data Format Reference

### Guard Definition

```ron
(
    id: "unique_guard_id",          // Required: Unique identifier
    name: "Display Name",            // Required: Name shown in UI
    description: "Description text", // Required: Tooltip text

    stats: (
        health: 100.0,               // Hit points
        damage: 15.0,                // Damage per attack
        attack_speed: 1.5,           // Attacks per second
        attack_range: 200.0,         // Range in pixels
        movement_speed: 0.0,         // (Guards don't move)
    ),

    cost: (
        hire: 100,                   // Gold to hire
        wages: 20,                   // Gold per mission
    ),

    sprite: "sprites/guards/name.png",  // Path to sprite
    sprite_size: (32, 32),           // Width, height

    abilities: [                     // Optional special abilities
        (
            id: "ability_id",
            name: "Ability Name",
            description: "What it does",
            cooldown: 5.0,           // Seconds (0 = passive)
        ),
    ],

    unlock_requirements: (
        min_reputation: 0,           // Minimum reputation
        min_level: 1,                // Minimum player level
        completed_contracts: 0,      // Contracts needed
    ),

    morale: (
        base: 80.0,
        decay_rate: 2.0,
        boost_on_victory: 10.0,
    ),
)
```

### Enemy Definition

```ron
(
    id: "enemy_id",
    name: "Enemy Name",
    description: "Description",

    stats: (
        health: 50.0,
        damage: 10.0,
        attack_speed: 1.0,
        movement_speed: 80.0,        // Pixels per second
    ),

    behavior: (
        ai_type: Aggressive,         // Aggressive, Defensive, Flee
        chase_range: 300.0,
        attack_range: 30.0,
        flee_health: 15.0,           // Flees when health drops below
    ),

    loot_table: [
        (item: "gold", amount: (10, 25), chance: 1.0),
        (item: "sword", amount: (1, 1), chance: 0.2),
    ],

    sprite: "sprites/enemies/name.png",

    spawn_conditions: (
        biomes: [Grassland, Forest],
        danger_level: (1, 10),       // Min, max difficulty
        time_of_day: Any,            // Any, Day, Night
    ),
)
```

### Item Definition

```ron
(
    id: "item_id",
    name: "Item Name",
    description: "What it does",

    item_type: Consumable,           // Consumable, Equipment, Cargo

    stats: (
        weight: 1.0,                 // Kg
        value: 25,                   // Gold value
        stack_size: 10,              // Max stack
    ),

    effects: [
        (
            effect: RestoreHealth,   // Effect type
            amount: 50.0,
            duration: Instant,       // Instant or duration in seconds
        ),
    ],

    sprite: "sprites/items/name.png",

    crafting: Some((              // Optional crafting recipe
        ingredients: [
            (item: "herb", quantity: 2),
            (item: "water", quantity: 1),
        ],
        skill_required: None,
    )),
)
```

## Asset Specifications

### Sprites

**Guards/Characters:**
- Size: 32x32 pixels
- Format: PNG with transparency
- Style: Pixel art, isometric view
- Color palette: 48-64 colors
- Animation: Optional (idle, attack frames)

**Items:**
- Size: 16x16 pixels
- Format: PNG with transparency
- Style: Match game aesthetic

### Audio

**Music:**
- Format: OGG Vorbis
- Bitrate: 128-192 kbps
- Loop: Seamless looping if needed

**Sound Effects:**
- Format: OGG Vorbis
- Bitrate: 96-128 kbps
- Length: Usually < 2 seconds

## Testing Your Mod

### Hot-Reload (Development)

```bash
# Run game in dev mode
cargo run --features dev

# Edit your .ron files
# Changes appear instantly in-game (no restart needed)
```

### Validation

```bash
# Validate mod structure
pog-mod-tool validate mods/my_mod

# Test load order
pog-mod-tool test mods/my_mod
```

### Common Errors

**"Mod failed to load"**
- Check `mod.ron` syntax (use RON validator)
- Ensure all referenced files exist
- Check game version compatibility

**"Asset not found"**
- Verify sprite paths are correct
- Use forward slashes: `sprites/guards/name.png`
- Check file extensions match

**"Conflicting mod"**
- Check load priority in `mod.ron`
- Higher priority = loads later = overrides others

## Publishing Your Mod

### 1. Package Mod

```bash
pog-mod-tool package mods/my_mod --output my_mod_v1.0.0.zip
```

### 2. Upload

**Itch.io:**
1. Create project page
2. Upload .zip file
3. Tag with "path-of-goodies" and "mod"

**Steam Workshop:**
1. Use in-game "Publish Mod" button
2. Fill out details
3. Upload

**GitHub:**
1. Create repository
2. Include installation instructions
3. Tag releases

### 3. Share

- Post in #mod-showcase on Discord
- Submit to mod directory
- Share on Reddit/social media

## Getting Help

- **Documentation**: See `MODDING_SYSTEM.md` in game root
- **Discord**: #modding-help channel
- **Examples**: Check other mods in `mods/` folder
- **Issues**: Report bugs on GitHub

## License

This example mod is released under the MIT License. You can use it as a template for your own mods.

## Credits

- Mod by: Your Name
- Game by: Path of Goodies Team
- Sprites by: [Artist name if applicable]

---

**Happy Modding!** 🎮

For more information, see the full modding documentation in the game's root folder.
