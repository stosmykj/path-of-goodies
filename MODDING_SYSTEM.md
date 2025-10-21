# Path of Goodies - Modding System Architecture

**Version**: 0.1
**Last Updated**: 2025-10-21
**Status**: Design Document

---

## Vision

**Path of Goodies will be fully moddable from day one.**

Everything from contracts, items, enemies, guards, biomes, to UI themes will be data-driven and moddable. The community will be able to create and share custom content easily.

---

## Table of Contents

1. [Core Philosophy](#core-philosophy)
2. [Mod Structure](#mod-structure)
3. [Data Formats](#data-formats)
4. [Asset Override System](#asset-override-system)
5. [Hot-Reload System](#hot-reload-system)
6. [Mod Loading & Priority](#mod-loading--priority)
7. [Validation & Safety](#validation--safety)
8. [Scripting (Future)](#scripting-future)
9. [Distribution & Workshop](#distribution--workshop)
10. [Modding API](#modding-api)

---

## 1. Core Philosophy

### Design Principles

1. **Data-Driven Everything** - No hardcoded game content
2. **Human-Readable Formats** - RON (Rusty Object Notation) for configs
3. **Hot-Reload in Dev** - Changes appear instantly while testing
4. **Asset Override** - Mods can replace any asset
5. **Additive by Default** - Multiple mods can coexist
6. **Safe Sandboxing** - No arbitrary code execution (initially)
7. **Easy Discovery** - Built-in mod browser
8. **One-Click Install** - Simple mod management

### What Can Be Modded?

✅ **Gameplay Content**:
- Contracts (delivery missions)
- Items (goods, consumables, equipment)
- Guards (types, stats, abilities)
- Enemies (types, behaviors, stats)
- NPCs and companions
- Events and encounters

✅ **World Content**:
- Biomes (terrain types)
- Settlements (cities, villages)
- Points of interest (shrines, ruins)
- Random encounters
- World generation rules

✅ **Visual Content**:
- Sprites (characters, tiles, objects)
- UI themes and layouts
- Particle effects
- Color palettes
- Fonts

✅ **Audio Content**:
- Music tracks
- Sound effects
- Ambient sounds

✅ **Balance & Tuning**:
- Stats and formulas
- Economy (prices, rewards)
- Difficulty curves
- Progression rates

✅ **Text & Localization**:
- All game text
- Translations
- Item descriptions
- Story text

❌ **Cannot Be Modded (Initially)**:
- Core game logic (ECS systems)
- Netcode and multiplayer
- Anti-cheat systems
- Executable code

---

## 2. Mod Structure

### Directory Layout

```
path-of-goodies/
├── assets/                 # Base game assets
│   ├── sprites/
│   ├── audio/
│   └── data/
├── mods/                   # User mods folder
│   ├── example_mod/
│   │   ├── mod.ron        # Mod metadata
│   │   ├── data/          # Data overrides/additions
│   │   │   ├── contracts.ron
│   │   │   ├── items.ron
│   │   │   ├── guards.ron
│   │   │   └── enemies.ron
│   │   ├── assets/        # Asset overrides/additions
│   │   │   ├── sprites/
│   │   │   ├── audio/
│   │   │   └── ui/
│   │   └── README.md      # Mod description
│   │
│   ├── medieval_overhaul/
│   └── fantasy_expansion/
│
└── mods_config.ron         # Mod load order and settings
```

### Mod Metadata File

**mods/example_mod/mod.ron**:

```ron
(
    // Mod identity
    id: "example_mod",
    name: "Example Mod",
    version: "1.0.0",
    author: "YourName",
    description: "Adds new guards and enemies to the game",

    // Compatibility
    game_version: "0.1.0",      // Minimum game version
    game_version_max: "0.2.0",  // Maximum (optional)

    // Dependencies
    dependencies: [
        (id: "base_game", version: "0.1.0"),
        (id: "optional_mod", version: "1.0.0", optional: true),
    ],

    // What this mod changes
    modifies: [
        "guards",
        "enemies",
        "sprites",
    ],

    // Load order (higher = later, can override)
    priority: 100,

    // Tags for discovery
    tags: ["gameplay", "guards", "enemies", "medieval"],

    // URLs
    homepage: "https://example.com/my-mod",
    repository: "https://github.com/user/example-mod",

    // License
    license: "MIT",
)
```

---

## 3. Data Formats

### RON (Rusty Object Notation)

All game data uses RON format for human readability and Rust integration.

#### Example: Contract Definition

**mods/example_mod/data/contracts.ron**:

```ron
// contracts.ron
(
    contracts: [
        (
            id: "grain_delivery_north",
            name: "Grain Delivery to Northport",
            description: "Deliver 100 bushels of grain to Northport before winter.",

            cargo: (
                type: Grain,
                quantity: 100,
                weight: 500.0,  // kg
                value: 200,     // gold
            ),

            origin: "capital_city",
            destination: "northport",

            payment: (
                base: 200,
                time_bonus: 50,     // If delivered early
                condition_bonus: 25, // If cargo undamaged
            ),

            time_limit: Some(900.0),  // seconds (15 min)
            difficulty: Medium,

            unlock_requirements: (
                min_reputation: 0,
                completed_contracts: 0,
            ),
        ),

        // Add more contracts...
    ],
)
```

#### Example: Guard Definition

**mods/example_mod/data/guards.ron**:

```ron
(
    guards: [
        (
            id: "archer_elite",
            name: "Elite Archer",
            description: "A skilled bowman with deadly accuracy.",

            stats: (
                health: 80.0,
                damage: 15.0,
                attack_speed: 1.5,   // attacks per second
                attack_range: 200.0, // pixels
                movement_speed: 0.0, // guards don't move
            ),

            cost: (
                hire: 100,     // One-time hire cost
                wages: 25,     // Per delivery
            ),

            sprite: "sprites/guards/archer_elite.png",

            abilities: [
                (
                    id: "piercing_shot",
                    cooldown: 5.0,
                    description: "Arrow pierces through enemies",
                ),
            ],

            unlock_requirements: (
                min_reputation: 50,
                level: 5,
            ),
        ),
    ],
)
```

#### Example: Enemy Definition

**mods/example_mod/data/enemies.ron**:

```ron
(
    enemies: [
        (
            id: "bandit_veteran",
            name: "Veteran Bandit",
            description: "An experienced outlaw, hardened by years of crime.",

            stats: (
                health: 60.0,
                damage: 12.0,
                attack_speed: 1.0,
                movement_speed: 80.0,  // pixels per second
            ),

            behavior: (
                ai_type: Aggressive,
                chase_range: 300.0,
                attack_range: 30.0,
                flee_health: 15.0,  // Flees when health below this
            ),

            loot_table: [
                (item: "gold", amount: (15, 30), chance: 1.0),
                (item: "rusty_sword", amount: (1, 1), chance: 0.3),
                (item: "bandage", amount: (1, 2), chance: 0.5),
            ],

            sprite: "sprites/enemies/bandit_veteran.png",

            spawn_conditions: (
                biomes: [Grassland, Forest],
                danger_level: (3, 10),
                time_of_day: Any,
            ),
        ),
    ],
)
```

#### Example: Biome Definition

**mods/example_mod/data/biomes.ron**:

```ron
(
    biomes: [
        (
            id: "dark_forest",
            name: "Dark Forest",

            terrain: (
                tiles: "tiles/dark_forest.png",
                movement_speed_multiplier: 0.6,  // 40% slower
            ),

            encounter_rates: (
                base_rate: 0.15,        // 15% per minute
                danger_multiplier: 1.5,
            ),

            possible_encounters: [
                (enemy: "wolf", weight: 40),
                (enemy: "bear", weight: 20),
                (enemy: "bandit", weight: 30),
                (event: "abandoned_camp", weight: 10),
            ],

            resource_costs: (
                food_multiplier: 1.0,
                water_multiplier: 0.8,  // Streams available
            ),

            atmosphere: (
                music: "audio/music/dark_forest.ogg",
                ambient: "audio/ambient/forest_sounds.ogg",
                particle_effects: ["falling_leaves", "fog"],
            ),
        ),
    ],
)
```

#### Example: Item Definition

**mods/example_mod/data/items.ron**:

```ron
(
    items: [
        (
            id: "health_potion",
            name: "Health Potion",
            description: "Restores 50 health instantly.",

            item_type: Consumable,

            stats: (
                weight: 0.5,
                value: 25,
                stack_size: 10,
            ),

            effects: [
                (
                    effect: RestoreHealth,
                    amount: 50.0,
                    duration: Instant,
                ),
            ],

            sprite: "sprites/items/health_potion.png",

            crafting: Some((
                ingredients: [
                    (item: "herb", quantity: 3),
                    (item: "water", quantity: 1),
                ],
                skill_required: None,
            )),
        ),
    ],
)
```

---

## 4. Asset Override System

### Override Priority

```
1. User mods (highest priority)
   ↓
2. Base game assets
   ↓
3. Fallback/default assets
```

### How It Works

```rust
// Pseudo-code for asset loading
fn load_sprite(path: &str) -> Handle<Image> {
    // Check enabled mods in priority order
    for mod in enabled_mods.iter().rev() {
        if mod.has_asset(path) {
            return load_from_mod(mod, path);
        }
    }

    // Fallback to base game
    load_from_base_game(path)
}
```

### Example: Replacing Wagon Sprite

```
Base game:
  assets/sprites/player/wagon.png

Mod override:
  mods/fancy_wagons/assets/sprites/player/wagon.png

Result:
  Game uses mod version when "fancy_wagons" mod is enabled
```

### Example: Adding New Content

```
Mod adds new guard type:
  mods/new_guards/data/guards.ron
  (contains new "crossbowman" guard)

  mods/new_guards/assets/sprites/guards/crossbowman.png

Result:
  New guard type appears in hire menu
```

---

## 5. Hot-Reload System

### Development Mode

```toml
# config.ron
(
    dev_mode: true,
    hot_reload: (
        enabled: true,
        watch_paths: [
            "assets/",
            "mods/",
        ],
        debounce_ms: 500,
    ),
)
```

### File Watcher

```rust
// Hot-reload implementation (pseudo-code)
fn setup_hot_reload(app: &mut App) {
    #[cfg(debug_assertions)]
    {
        app.add_systems(Update, (
            watch_for_file_changes,
            reload_changed_assets,
            reload_changed_data,
        ));
    }
}

fn watch_for_file_changes(
    mut file_events: EventReader<FileChange>,
    mut reload_queue: ResMut<ReloadQueue>,
) {
    for event in file_events.read() {
        match event.path.extension() {
            Some("ron") => reload_queue.add_data(event.path),
            Some("png") => reload_queue.add_sprite(event.path),
            Some("ogg") => reload_queue.add_audio(event.path),
            _ => {},
        }
    }
}
```

### Developer Experience

1. Edit `guards.ron` file
2. Save
3. Game detects change within 500ms
4. Reloads guard definitions
5. See changes immediately in-game
6. No restart required!

---

## 6. Mod Loading & Priority

### Load Order Configuration

**mods_config.ron**:

```ron
(
    enabled_mods: [
        (id: "base_game", enabled: true, priority: 0),
        (id: "balance_mod", enabled: true, priority: 100),
        (id: "new_guards", enabled: true, priority: 200),
        (id: "visual_overhaul", enabled: false, priority: 300),
    ],

    auto_enable_dependencies: true,
    warn_on_conflicts: true,
)
```

### Priority System

- **Lower number** = loads first, lower priority
- **Higher number** = loads last, can override
- **Base game** = priority 0
- **Mods** = typically 100-1000

### Conflict Resolution

```
Scenario: Two mods both modify "archer" guard

Mod A (priority 100):
  archer.damage = 15

Mod B (priority 200):
  archer.damage = 20

Result:
  archer.damage = 20 (Mod B wins)

Warning shown:
  "Mod 'B' overrides 'archer.damage' from Mod 'A'"
```

### Merging Strategies

```ron
// additive_merge.ron example
(
    merge_strategy: Additive,  // vs Override

    // Original base game has 3 guards
    // Mod adds 2 more guards
    // Result: 5 guards total
)
```

---

## 7. Validation & Safety

### Schema Validation

```rust
// Data validation on load
#[derive(Deserialize)]
struct GuardDefinition {
    #[serde(validate = "non_empty")]
    id: String,

    #[serde(validate = "non_empty")]
    name: String,

    stats: GuardStats,

    #[serde(validate = "exists")]
    sprite: String,  // Must point to real file
}

impl Validate for GuardStats {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.health <= 0.0 {
            return Err("health must be positive");
        }
        if self.damage < 0.0 {
            return Err("damage cannot be negative");
        }
        Ok(())
    }
}
```

### Safe Sandboxing

**Phase 1 (MVP)**: Data-only mods
- No code execution
- Only RON data files
- Safe by design

**Phase 2 (Future)**: Scripting support
- Lua or Rhai scripting
- Sandboxed environment
- Limited API surface
- No file system access
- No network access
- CPU/memory limits

### Mod Verification

```rust
fn verify_mod(mod_path: &Path) -> Result<ModManifest, ModError> {
    // 1. Check mod.ron exists and is valid
    let manifest = load_manifest(mod_path)?;

    // 2. Validate version compatibility
    check_version_compatibility(&manifest)?;

    // 3. Check dependencies are installed
    check_dependencies(&manifest)?;

    // 4. Validate all referenced assets exist
    check_asset_references(&manifest)?;

    // 5. Validate data schemas
    validate_data_files(mod_path)?;

    Ok(manifest)
}
```

---

## 8. Scripting (Future)

### Phase 1: No Scripting (MVP)

For initial release, keep it simple:
- Data-driven only
- No custom logic
- Safe and stable

### Phase 2: Lua Scripting (Post-Launch)

Add scripting for advanced mods:

```lua
-- mods/example_mod/scripts/custom_encounter.lua

function on_encounter_start(context)
    -- Custom encounter logic
    local player = context.player
    local difficulty = context.difficulty

    if difficulty > 5 then
        spawn_enemy("elite_bandit", 3)
    else
        spawn_enemy("bandit", 2)
    end

    show_dialog("Bandits ahead! Prepare for battle!")
end

function on_enemy_killed(enemy)
    if enemy.type == "elite_bandit" then
        drop_item("rare_sword", 1)
        player.reputation += 10
    end
end
```

### API Surface

```lua
-- Available to mod scripts (safe subset)

-- Entity spawning
spawn_enemy(id, count)
spawn_item(id, count)

-- UI
show_dialog(text)
show_notification(text, duration)

-- Player state
player.health
player.gold
player.inventory

-- Events
on_encounter_start(callback)
on_enemy_killed(callback)
on_contract_complete(callback)

-- Utilities
random_int(min, max)
random_float(min, max)
get_current_biome()
```

### Security Model

```rust
// Lua sandbox configuration
let lua = Lua::new();

// Limit memory
lua.set_memory_limit(Some(10 * 1024 * 1024));  // 10MB

// Limit CPU
lua.set_hook(HookTriggers::every_nth_instruction(10000), |_| {
    // Check if script has run too long
});

// Disable dangerous functions
lua.globals().set("io", Nil)?;
lua.globals().set("os", Nil)?;
lua.globals().set("require", Nil)?;
```

---

## 9. Distribution & Workshop

### Manual Installation

```
1. Download mod .zip file
2. Extract to mods/ folder
3. Enable in game settings
4. Restart or hot-reload
```

### Steam Workshop Integration (Future)

```rust
// Steamworks integration
fn subscribe_to_mod(workshop_id: u64) {
    steamworks::ugc::subscribe(workshop_id);
}

fn download_subscribed_mods() {
    let subscribed = steamworks::ugc::get_subscribed_items();
    for item in subscribed {
        if item.needs_update() {
            item.download();
        }
    }
}
```

### In-Game Mod Browser

```
╔══════════════════════════════════════════════════╗
║             MOD BROWSER                          ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  [Search: ________]  [Sort: Popular ▼]          ║
║                                                  ║
║  ┌────────────────────────────────────────────┐ ║
║  │ 🎨 Medieval Overhaul                       │ ║
║  │ by UserName  ⭐⭐⭐⭐⭐ (1.2k)           │ ║
║  │ Complete visual overhaul with hand-drawn   │ ║
║  │ sprites. Adds 50+ new assets.              │ ║
║  │ [Subscribe]                  v1.5.2        │ ║
║  └────────────────────────────────────────────┘ ║
║                                                  ║
║  ┌────────────────────────────────────────────┐ ║
║  │ ⚔️ New Guards Pack                         │ ║
║  │ by ArtistName  ⭐⭐⭐⭐☆ (856)            │ ║
║  │ Adds 10 new guard types with unique        │ ║
║  │ abilities and animations.                  │ ║
║  │ [Subscribed ✓]               v2.0.1        │ ║
║  └────────────────────────────────────────────┘ ║
║                                                  ║
╚══════════════════════════════════════════════════╝
```

### Mod Repository API

```json
// Example mod listing from repository
{
  "id": "medieval_overhaul",
  "name": "Medieval Overhaul",
  "author": "UserName",
  "version": "1.5.2",
  "game_version": "0.1.0",
  "downloads": 15234,
  "rating": 4.8,
  "tags": ["visual", "overhaul", "sprites"],
  "thumbnail": "https://cdn.example.com/mods/medieval_overhaul/thumb.png",
  "download_url": "https://cdn.example.com/mods/medieval_overhaul/v1.5.2.zip",
  "file_size": 15728640,
  "created_at": "2025-01-15",
  "updated_at": "2025-10-10"
}
```

---

## 10. Modding API

### Bevy Resource Access

```rust
// Expose game state to mods (read-only)
#[derive(Resource)]
pub struct ModdingAPI {
    pub player_state: Arc<RwLock<PlayerState>>,
    pub world_state: Arc<RwLock<WorldState>>,
    pub economy: Arc<RwLock<Economy>>,
}

impl ModdingAPI {
    pub fn get_player_gold(&self) -> i32 {
        self.player_state.read().unwrap().gold
    }

    pub fn get_current_biome(&self) -> BiomeId {
        self.world_state.read().unwrap().current_biome
    }
}
```

### Event Hooks

```rust
// Mod event system
#[derive(Event)]
pub enum ModEvent {
    ContractAccepted { contract_id: String },
    ContractCompleted { contract_id: String, reward: i32 },
    EnemyKilled { enemy_id: String },
    GuardHired { guard_id: String },
    PlayerDied,
    // ... more events
}

// Mods can register callbacks
pub fn register_mod_callback(event_type: ModEventType, callback: fn(ModEvent)) {
    // Store and trigger on event
}
```

### Mod Creation Tools

**CLI Tool**: `pog-mod-tool`

```bash
# Create new mod
pog-mod-tool create my_mod

# Validate mod
pog-mod-tool validate mods/my_mod

# Package mod for distribution
pog-mod-tool package mods/my_mod --output my_mod_v1.0.0.zip

# Test mod
pog-mod-tool test mods/my_mod
```

---

## Implementation Phases

### Phase 1 (MVP - Weeks 3-6)
- [x] Data-driven design for contracts, items, guards
- [ ] RON file loading
- [ ] Basic asset override system
- [ ] Mod folder structure

### Phase 2 (Alpha - Weeks 7-10)
- [ ] Hot-reload system (dev mode)
- [ ] Mod priority system
- [ ] Validation and error handling
- [ ] In-game mod menu (enable/disable)

### Phase 3 (Beta - Weeks 11-14)
- [ ] Mod browser UI
- [ ] Mod repository integration
- [ ] One-click install/uninstall
- [ ] Conflict detection and warnings

### Phase 4 (Post-Launch - Month 4+)
- [ ] Lua/Rhai scripting support
- [ ] Steam Workshop integration
- [ ] Mod creation tools (CLI)
- [ ] Community showcase

---

## Community Resources

### Modding Documentation

**Provide to community**:
- Modding tutorial (step-by-step)
- Data format reference
- Asset specifications
- Example mods (source code)
- Mod template generator
- Best practices guide

### Example Starter Mod

**mods/starter_template/** (distributed with game):

```
starter_template/
├── mod.ron
├── README.md (tutorial)
├── data/
│   └── example_guard.ron
└── assets/
    └── sprites/
        └── example_guard.png
```

### Modding Discord Channel

- #modding-help
- #mod-showcase
- #mod-development
- #mod-releases

---

## Success Metrics

**Launch Goals**:
- 5+ example mods available
- Modding documentation complete
- Active modding community (50+ members)

**3 Months Post-Launch**:
- 50+ community mods
- 1,000+ mod downloads
- Featured mod showcases

**6 Months Post-Launch**:
- 200+ community mods
- Workshop integration (if Steam)
- Mod of the month contests
- Professional quality mod packs

---

## Conclusion

By making Path of Goodies **fully moddable from day one**, we:
1. ✅ Empower the community to create content
2. ✅ Extend game lifespan infinitely
3. ✅ Reduce development burden (community helps)
4. ✅ Build passionate fanbase
5. ✅ Create unique selling point

**Modding is not a feature - it's a core pillar of the game.**

---

**Status**: Design Complete
**Next**: Implement in Phase 1
**Owner**: Development Team
**Review**: Monthly during active development
