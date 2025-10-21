# Path of Goodies - Travel System Design

**Version**: 0.2
**Last Updated**: 2025-10-21
**Status**: Detailed Design

---

## Table of Contents

1. [Overview](#overview)
2. [Journey Start & Prologue](#journey-start--prologue)
3. [Horse Mechanics](#horse-mechanics)
4. [Wagon Movement](#wagon-movement)
5. [Path & Navigation](#path--navigation)
6. [Camping & Save System](#camping--save-system)
7. [First Encounter (Tutorial)](#first-encounter-tutorial)
8. [Equipment System](#equipment-system)
9. [Wild Horse Taming](#wild-horse-taming)
10. [Technical Implementation](#technical-implementation)

---

## 1. Overview

The **Travel System** is the core gameplay loop where the player navigates their wagon from origin to destination, managing horses, resources, and encounters along the way.

### Core Loop

```
Accept Contract
    ↓
[PROLOGUE] First time: Animated introduction
    ↓
Select Destination on Map
    ↓
──────── TRAVEL PHASE ────────
│                            │
│  ┌─ Manage Horse Stamina  │
│  ├─ Navigate Path          │
│  ├─ Handle Encounters      │
│  ├─ Camp to Save/Rest      │
│  └─ React to Events        │
│                            │
──────────────────────────────
    ↓
Reach Destination
    ↓
Deliver Goods / Trade
    ↓
[Repeat or End]
```

---

## 2. Journey Start & Prologue

### First Time Player Experience

**Prologue Sequence** (First game launch):

```
1. Opening Cutscene (30-60 seconds)
   ├─ Camera pans over medieval kingdom map
   ├─ Narrator voiceover:
   │  "In the kingdom of [Name], traders brave dangerous roads
   │   to deliver goods between settlements. You are one such trader,
   │   seeking fortune on the Path of Goodies..."
   └─ Fade to player wagon on road

2. Character Introduction
   ├─ Player character standing by wagon
   ├─ Simple dialog box:
   │  "You are [Name], a skilled trader with a rusty sword
   │   and a short bow. Your first contract awaits..."
   └─ Show basic stats (health, skills)

3. Contract Assignment
   ├─ Contract board UI appears
   ├─ Single highlighted contract (tutorial)
   │  "Deliver grain to Northport Village"
   │  Payment: 50 gold
   │  Distance: Short (tutorial path)
   └─ Auto-accept or click to accept

4. Horse & Wagon Introduction
   ├─ Camera focuses on wagon and horse
   ├─ Tooltip: "Your horse pulls the wagon. Keep it healthy!"
   ├─ Show stamina bar above horse
   └─ Tutorial prompt: "Click the road to begin your journey"

5. Spawn on Path
   ├─ Player wagon appears at start of path
   ├─ Path clearly visible ahead
   ├─ HUD appears (resources, horse stamina)
   └─ Movement tutorial begins
```

### Subsequent Journeys

**Standard Start** (After prologue):

```
1. At Settlement
   ├─ Player in city/town
   ├─ Can visit: Contract board, Shop, Inn
   └─ Accept contract to trigger journey

2. Map View
   ├─ Shows kingdom overview
   ├─ Highlight: Origin (current location)
   ├─ Highlight: Destination (contract location)
   ├─ Path options shown (if multiple routes)
   └─ Click path to begin

3. Transition
   ├─ Brief loading screen (optional)
   ├─ Fade in to wagon on selected path
   ├─ Journey begins immediately
   └─ HUD shows active contract
```

---

## 3. Horse Mechanics

### Horse Stats

```rust
#[derive(Component)]
struct Horse {
    // Identity
    name: String,              // e.g., "Brown Mare"
    horse_type: HorseType,     // Draft, Riding, Wild

    // Core Stats
    health: f32,               // 0-100
    stamina: f32,              // 0-100
    max_stamina: f32,          // Base 100, increases with care

    // Condition
    exhaustion: f32,           // 0-100 (higher = more tired)
    morale: f32,               // 0-100 (affects performance)
    hunger: f32,               // 0-100 (needs feeding)

    // Performance
    speed_multiplier: f32,     // 1.0 = normal, 1.5 = fast horse
    pulling_power: f32,        // How much weight it can pull
    recovery_rate: f32,        // How fast stamina recovers

    // Equipment (not in demo)
    armor: Option<HorseArmor>,
    saddle: Option<Saddle>,
}

enum HorseType {
    Draft,      // Strong, slow, high pulling power
    Riding,     // Fast, weak pulling
    Wild,       // Balanced, needs taming
}
```

### Stamina System

**Stamina Consumption**:

```
Base Consumption Rate:
- Walking: -2 stamina/second
- Trotting (normal): -5 stamina/second
- Galloping (whipped): -15 stamina/second

Modifiers:
- Heavy wagon: +50% consumption
- Uphill: +30% consumption
- Rough terrain: +20% consumption
- Downhill: -20% consumption
- Well-fed horse: -10% consumption
```

**Stamina Recovery**:

```
Recovery Methods:
1. Idle (wagon stopped): +10 stamina/second
2. Walking slowly: +2 stamina/second (if stamina > 50)
3. Camping (rest): +20 stamina/second
4. Feeding oats: +30 stamina instant

Recovery Blocked When:
- In combat
- Galloping (whipped)
- Exhaustion >= 80
```

**Exhaustion Mechanic**:

```rust
fn update_exhaustion(horse: &mut Horse, delta: f32) {
    // Exhaustion builds when stamina is low
    if horse.stamina < 30.0 {
        horse.exhaustion += delta * 5.0; // Rapid exhaustion
    } else if horse.stamina < 50.0 {
        horse.exhaustion += delta * 2.0; // Slow exhaustion
    }

    // Exhaustion only recovers during rest
    if horse.is_resting && !in_combat {
        horse.exhaustion -= delta * 3.0;
    }

    // Clamp
    horse.exhaustion = horse.exhaustion.clamp(0.0, 100.0);
}

fn get_speed_penalty(horse: &Horse) -> f32 {
    if horse.exhaustion > 80.0 {
        0.3 // 70% speed penalty
    } else if horse.exhaustion > 60.0 {
        0.6 // 40% penalty
    } else if horse.exhaustion > 40.0 {
        0.8 // 20% penalty
    } else {
        1.0 // No penalty
    }
}
```

**Consequences of Exhaustion**:

| Exhaustion | Effect | Visual Indicator |
|-----------|--------|------------------|
| 0-40 | Normal performance | Horse sprite normal |
| 40-60 | -20% speed, occasional stumble | Horse breathing heavily |
| 60-80 | -40% speed, frequent stumbles | Horse sweating, head down |
| 80-100 | -70% speed, may collapse | Horse stumbling, critical warning |
| 100 | **Horse Collapse** - Cannot move until rested | Horse lying down, flashing warning |

### Whip Mechanic

**Whip Boost**:

```rust
#[derive(Component)]
struct WhipBoost {
    active: bool,
    duration: f32,           // Max 5 seconds
    cooldown: f32,           // 15 seconds before next whip
    speed_multiplier: f32,   // 2.0x speed when active
}

fn use_whip(horse: &mut Horse, boost: &mut WhipBoost) {
    if boost.cooldown > 0.0 {
        // Still on cooldown
        show_message("Horse needs rest before whipping again!");
        return;
    }

    if horse.exhaustion > 70.0 {
        // Too exhausted to whip
        show_message("Horse is too exhausted!");
        return;
    }

    // Activate boost
    boost.active = true;
    boost.duration = 5.0;
    boost.speed_multiplier = 2.0;

    // Immediate cost
    horse.stamina -= 20.0;
    horse.exhaustion += 15.0;
    horse.morale -= 10.0;

    // Play whip sound and animation
    play_sound("whip_crack");
    show_effect("whip_effect");
}
```

**UI Feedback**:
- **Whip Button** in HUD (spacebar or UI button)
- Shows cooldown timer when used
- Disabled (grayed out) when horse exhausted
- Tooltip: "Whip horse for speed boost (5s, 15s cooldown)"

**Strategic Use**:
- ✅ **Good**: Whip to escape ambush
- ✅ **Good**: Speed boost to reach camp before nightfall
- ❌ **Bad**: Whipping exhausted horse (damages morale)
- ❌ **Bad**: Constant whipping (horse collapses)

### Horse Death/Loss

**Horse Can Die From**:
1. **Health reaches 0** (attacked by enemies/wolves)
2. **Extreme neglect** (starved for extended time)
3. **Stolen by bandits** (special encounter)

**What Happens When Horse Dies**:

```
1. Horse Death Event
   ├─ Horse falls, death animation
   ├─ Wagon stops immediately
   ├─ Dialog: "Your horse has died! You must pull the wagon yourself."
   └─ Remove horse entity

2. Wagon Mode Changes
   ├─ Player character attaches to wagon (new sprite)
   ├─ Movement speed: -70% (very slow)
   ├─ Stamina system transfers to player
   └─ Player gets exhausted instead of horse

3. Options to Recover
   ├─ Find wild horse on meadow (see section 9)
   ├─ Buy new horse at next settlement
   ├─ Abandon wagon (if multiplayer, others can loot)
   └─ Continue struggle to destination
```

**Stolen Horse Encounter**:

```
Special Random Event: Horse Thieves
- Bandits attempt to steal horse
- Player must defend or horse is taken
- If stolen: Bandits flee with horse
- Player can chase but wagon left behind
- Choice: Abandon wagon or pull it yourself
```

---

## 4. Wagon Movement

### Movement States

```rust
enum WagonState {
    Idle,               // Stopped, not moving
    Walking,            // Normal travel speed
    Galloping,          // Whipped, 2x speed
    PlayerPulling,      // No horse, player pulls
    Camping,            // Setup camp, saved game
    InCombat,           // Cannot move freely
}

#[derive(Component)]
struct Wagon {
    // Movement
    speed: f32,                 // Current speed (pixels/sec)
    base_speed: f32,            // Base: 60 px/sec
    direction: Vec2,            // Current movement direction
    state: WagonState,

    // Cargo
    cargo_weight: f32,          // Total weight (kg)
    max_capacity: f32,          // Max weight before penalty
    goods: Vec<CargoItem>,

    // Condition
    durability: f32,            // 0-100
    wheel_damage: f32,          // Affects speed

    // Attached entities
    horse: Option<Entity>,      // Horse entity pulling wagon
    guards: Vec<Entity>,        // Guards riding/walking with wagon
    player: Entity,             // Player character
}
```

### Movement Speed Calculation

```rust
fn calculate_wagon_speed(
    wagon: &Wagon,
    horse: Option<&Horse>,
    terrain: &TerrainType,
    boost: &WhipBoost,
) -> f32 {
    let base = wagon.base_speed; // 60 px/sec

    // Horse modifier
    let horse_mult = if let Some(h) = horse {
        h.speed_multiplier * get_speed_penalty(h)
    } else {
        0.3 // Player pulling = 30% of base speed
    };

    // Cargo weight penalty
    let weight_mult = if wagon.cargo_weight > wagon.max_capacity {
        0.7 // Overloaded = 30% slower
    } else {
        1.0 - (wagon.cargo_weight / wagon.max_capacity) * 0.2
    };

    // Terrain modifier
    let terrain_mult = match terrain {
        TerrainType::Road => 1.0,
        TerrainType::Grass => 0.8,
        TerrainType::Forest => 0.6,
        TerrainType::Mountain => 0.4,
        TerrainType::Swamp => 0.3,
    };

    // Whip boost
    let boost_mult = if boost.active { 2.0 } else { 1.0 };

    // Durability penalty
    let durability_mult = if wagon.durability < 30.0 {
        0.7 // Damaged wagon = slower
    } else {
        1.0
    };

    // Final calculation
    base * horse_mult * weight_mult * terrain_mult * boost_mult * durability_mult
}
```

### Player Pulling Wagon (No Horse)

**Mechanics**:

```rust
#[derive(Component)]
struct PlayerPulling {
    active: bool,
    stamina: f32,           // Player stamina (0-100)
    exhaustion: f32,        // Player exhaustion
    speed_multiplier: f32,  // 0.3 (very slow)
}

// When player pulls wagon
fn update_player_pulling(
    player: &mut PlayerPulling,
    wagon: &Wagon,
    delta: f32,
) {
    // Stamina drain
    let base_drain = 3.0 * delta;
    let weight_mult = 1.0 + (wagon.cargo_weight / 100.0);
    player.stamina -= base_drain * weight_mult;

    // Exhaustion builds
    if player.stamina < 30.0 {
        player.exhaustion += delta * 4.0;
    }

    // Cannot pull if too exhausted
    if player.exhaustion > 90.0 {
        wagon.state = WagonState::Idle;
        show_message("You're too exhausted to pull the wagon!");
    }
}
```

**Visual Changes**:
- Player character sprite changes to "pulling wagon" animation
- Wagon has ropes/harness attached to player
- Significantly slower movement (walking pace)
- Frequent rest stops required
- Struggle animations when uphill

**Consequences**:
- ⚠️ **Very Slow**: 70% speed reduction
- ⚠️ **Exhausting**: Player stamina drains fast
- ⚠️ **Vulnerable**: Harder to escape ambushes
- ⚠️ **Costly**: Takes much longer to reach destination
- ✅ **Possible**: Can still complete delivery (hardcore mode!)

---

## 5. Path & Navigation

### Path Structure

**Path System**:

```rust
#[derive(Component)]
struct TravelPath {
    id: String,
    name: String,

    // Route definition
    origin: Entity,         // Settlement entity
    destination: Entity,
    waypoints: Vec<Vec2>,   // Path coordinates
    total_distance: f32,    // Meters

    // Properties
    terrain_types: Vec<(f32, TerrainType)>,  // (distance, terrain)
    danger_level: u8,       // 1-10
    encounter_rate: f32,    // Events per minute

    // Splits
    branches: Vec<PathBranch>,
}

struct PathBranch {
    position: f32,          // Distance where split occurs
    target: Entity,         // Destination settlement
    risk_level: RiskLevel,  // Safe, Risky, Dangerous
    distance_modifier: f32, // Shortcut vs detour
}

enum RiskLevel {
    Safe,       // Guards patrol, low encounters
    Moderate,   // Mixed, some patrols
    Risky,      // Few patrols, more bandits
    Dangerous,  // No patrols, high danger
}
```

### Path Selection UI

**Map View**:

```
╔═══════════════════════════════════════════════════════════╗
║                    KINGDOM MAP                            ║
╠═══════════════════════════════════════════════════════════╣
║                                                           ║
║     [Capital City]                                        ║
║          ║                                                ║
║          ║ King's Road (Safe, 500m, 8 min)              ║
║          ║                                                ║
║          ╠═══════════════╗                               ║
║          ║               ║                                ║
║      Forest Path    Mountain Pass                        ║
║      (Risky, 350m)  (Dangerous, 400m)                   ║
║          ║               ║                                ║
║          ║               ║                                ║
║          ╚═══════╦═══════╝                               ║
║                  ║                                        ║
║            [Northport Village]                           ║
║                                                           ║
║  ┌──────────────────────────────────────────────┐       ║
║  │ PATH OPTIONS:                                 │       ║
║  │                                               │       ║
║  │ 1. King's Road                               │       ║
║  │    Distance: 500m | Time: ~8min              │       ║
║  │    Risk: ⚠️ Safe | Guards patrol            │       ║
║  │    Toll: 10 gold                             │       ║
║  │                                               │       ║
║  │ 2. Forest Path (SHORTCUT)                    │       ║
║  │    Distance: 350m | Time: ~7min              │       ║
║  │    Risk: ⚠️⚠️ Risky | Bandit territory       │       ║
║  │    Toll: None                                 │       ║
║  │                                               │       ║
║  │ 3. Mountain Pass                              │       ║
║  │    Distance: 400m | Time: ~10min (slow)      │       ║
║  │    Risk: ⚠️⚠️⚠️ Dangerous | Harsh terrain    │       ║
║  │    Reward: Rare herbs found                  │       ║
║  │                                               │       ║
║  │              [ SELECT PATH ]                  │       ║
║  └──────────────────────────────────────────────┘       ║
╚═══════════════════════════════════════════════════════════╝
```

### Path Splits (Dynamic Choice)

**Mid-Journey Branch**:

```
Scenario: Player on King's Road
    ↓
━━━━━━━━━━━━━━━━━━━━━ (traveling)
    ↓
    ╔═══════════════════════════════╗
    ║  PATH SPLIT AHEAD!            ║
    ╠═══════════════════════════════╣
    ║                               ║
    ║  Continue on King's Road?     ║
    ║  Safe but longer (3min)       ║
    ║                               ║
    ║  OR                           ║
    ║                               ║
    ║  Take Forest Shortcut?        ║
    ║  Dangerous but faster (1min)  ║
    ║                               ║
    ║  [King's Road] [Forest]       ║
    ╚═══════════════════════════════╝
    ↓
Choice made, wagon turns onto selected path
```

**Implementation**:

```rust
fn check_path_branch(
    wagon_position: f32,
    current_path: &TravelPath,
) -> Option<PathBranch> {
    for branch in &current_path.branches {
        if (wagon_position - branch.position).abs() < 10.0 {
            return Some(branch.clone());
        }
    }
    None
}

fn show_branch_choice_ui(branch: &PathBranch) {
    // Pause wagon movement
    // Show modal dialog
    // Player chooses: continue or take branch
    // Resume with chosen path
}
```

---

## 6. Camping & Save System

### Camping Mechanic

**Setting Up Camp**:

```
Player Action: Press 'C' or click "Camp" button
    ↓
Requirements Check:
├─ Not in combat
├─ Not too close to last camp (min 100m apart)
├─ On safe terrain (not in water, cliff)
└─ Have camping supplies (optional: fire, tent)
    ↓
Camp Setup Sequence:
1. Wagon stops
2. Animation: Character unpacks tent/bedroll
3. Campfire appears (if have supplies)
4. UI changes to "Camp Menu"
5. Game auto-saves
```

**Camp Menu**:

```
╔═══════════════════════════════════════════════╗
║          CAMP MENU                            ║
╠═══════════════════════════════════════════════╣
║                                               ║
║  🔥 Campfire burning                         ║
║  🐴 Horse: Resting (stamina recovering)      ║
║  😴 Player: Can sleep to restore health      ║
║                                               ║
║  ┌──────────────────────────────────────────┐║
║  │ REST                                     │║
║  │ Sleep for 8 hours                        │║
║  │ Restores: Health, Horse Stamina         │║
║  │ Time passes: Morning → Evening           │║
║  │                  [REST]                  │║
║  └──────────────────────────────────────────┘║
║                                               ║
║  ┌──────────────────────────────────────────┐║
║  │ MANAGE INVENTORY                         │║
║  │ Sort cargo, eat food, check equipment    │║
║  │                [INVENTORY]               │║
║  └──────────────────────────────────────────┘║
║                                               ║
║  ┌──────────────────────────────────────────┐║
║  │ SAVE & QUIT                              │║
║  │ Game saved. You can resume later.        │║
║  │             [SAVE & QUIT]                │║
║  └──────────────────────────────────────────┘║
║                                               ║
║           [PACK UP AND CONTINUE]             ║
╚═══════════════════════════════════════════════╝
```

### Save System Design

**Save Philosophy**:
- ✅ **Save = Pause** - Resume exactly where you left off
- ❌ **NOT a checkpoint** - Death does NOT reload save
- ✅ **Auto-save** at camps
- ✅ **One save per run** - Cannot savescum

**Save Data Structure**:

```rust
#[derive(Serialize, Deserialize)]
struct GameSave {
    // Meta
    version: String,
    timestamp: DateTime,
    playtime: f32,

    // Journey State
    active_journey: bool,
    current_path: Option<PathId>,
    position_on_path: f32,    // Distance traveled
    time_of_day: f32,         // 0-24 hours

    // Player State
    player: PlayerSaveData,
    wagon: WagonSaveData,
    horse: Option<HorseSaveData>,
    guards: Vec<GuardSaveData>,

    // Resources
    food: f32,
    water: f32,
    gold: i32,
    inventory: Vec<ItemStack>,

    // Active Contract
    contract: Option<ContractSaveData>,

    // World State
    discovered_areas: HashSet<ChunkId>,
    completed_encounters: HashSet<EncounterId>,
    reputation: HashMap<Faction, i32>,

    // Multiplayer (if applicable)
    world_seed: Option<u64>,
    server_url: Option<String>,
}
```

**Save Trigger Points**:

```rust
fn should_trigger_save() -> bool {
    // Auto-save when:
    // 1. Setting up camp
    // 2. Completing delivery
    // 3. Major events (recruit companion, etc.)
    // 4. Player clicks "Save & Quit"

    // NOT when:
    // - In combat
    // - Horse just died (prevents abuse)
    // - Currently being chased
}
```

**Death and Save**:

```
Scenario: Player dies in combat
    ↓
Death Screen appears
    ↓
Options:
├─ "Continue as New Trader" (same world)
│  └─ Your old wagon remains where you died
│      (Can be looted if you find it)
│
└─ "New World" (fresh start)
    └─ Generates new world seed

Save file is NOT deleted
Previous camp save remains
BUT you cannot "reload" to before death
```

**Resume from Save**:

```
Main Menu → "Continue"
    ↓
Load save file
    ↓
Fade in to exact camp location
    ↓
Player at camp, wagon parked
    ↓
Horse rested (stamina recovered)
    ↓
Camp menu appears OR auto-pack if player chooses
    ↓
Continue journey
```

---

## 7. First Encounter (Tutorial)

### Tutorial Encounter Design

**Setup**:

```
Location: 200m into first journey
Trigger: Automatic (scripted)
Enemies: 4 Bandits + 1 Elite Bandit
Objective: Defeat bandits, rescue hostage
Reward: First companion (permanent)
```

**Encounter Sequence**:

```
1. PRE-ENCOUNTER (Visual cues)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Player traveling on road
       ↓
   Camera zooms out slightly
       ↓
   Visual: Smoke in forest (left side of road)
       ↓
   Audio: Distant shouting
       ↓
   Tutorial prompt: "⚠️ Danger ahead! Prepare for combat!"
       ↓
   Wagon slows automatically
       ↓
   Enemies visible in distance (forest edge)


2. ENCOUNTER START
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Wagon stops
       ↓
   Cutscene: Camera pans to forest
       ↓
   Show: 4 bandits surrounding a hostage
       ↓
   Elite bandit (leader): "Hand over your goods, trader!"
       ↓
   Tutorial prompt:
   "Draw your weapon! Click [Sword] or [Bow]"
       ↓
   Combat begins


3. COMBAT PHASE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   ┌────────────────────────────────────┐
   │  ENEMIES:                          │
   │                                    │
   │  [🗡️] Bandit 1    HP: ▓▓▓▓░     │
   │  [🗡️] Bandit 2    HP: ▓▓▓▓░     │
   │  [🏹] Bandit 3    HP: ▓▓▓░░░     │
   │  [🗡️] Bandit 4    HP: ▓▓▓▓░     │
   │  [⚔️] Elite        HP: ▓▓▓▓▓▓▓  │
   │                                    │
   │  YOU:                              │
   │  [🗡️] Rusty Sword (equipped)      │
   │  [🏹] Short Bow (in inventory)    │
   │  HP: ▓▓▓▓▓▓▓▓▓▓ 100/100          │
   └────────────────────────────────────┘

   Tutorial prompts during combat:
   ├─ "Use [1] for sword, [2] for bow"
   ├─ "Bandits 1-2 charging! Defend yourself!"
   ├─ "Switch to bow for distant enemies"
   ├─ "Watch your health! Use [H] for health potion"
   └─ "Defeat the Elite to win!"


4. VICTORY
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   All enemies defeated
       ↓
   Victory fanfare
       ↓
   Loot drops (gold, items)
       ↓
   Hostage freed
       ↓
   Dialog:
   "Thank you! I'm [Name], a skilled archer.
    Let me join your caravan as thanks!"
       ↓
   Companion joins party
       ↓
   Tutorial complete!
```

### Enemy Stats (Tutorial)

```ron
// First encounter enemies (easy, tutorial)
(
    enemies: [
        // Regular bandits
        (
            id: "tutorial_bandit",
            name: "Bandit",
            count: 4,

            stats: (
                health: 40.0,           // Low HP (tutorial)
                damage: 8.0,            // Low damage
                attack_speed: 0.8,
                movement_speed: 70.0,
            ),

            behavior: (
                ai_type: Aggressive,
                chase_range: 150.0,
                attack_range: 30.0,     // Melee
                flee_health: 10.0,      // Flees when low
            ),

            loot_table: [
                (item: "gold", amount: (3, 8), chance: 1.0),
                (item: "rusty_dagger", amount: (1, 1), chance: 0.3),
            ],

            sprite: "sprites/enemies/bandit_basic.png",
        ),

        // Elite bandit (leader)
        (
            id: "tutorial_elite",
            name: "Bandit Leader",
            count: 1,

            stats: (
                health: 80.0,           // Higher HP
                damage: 15.0,           // More damage
                attack_speed: 1.0,
                movement_speed: 80.0,
            ),

            behavior: (
                ai_type: Aggressive,
                chase_range: 200.0,
                attack_range: 30.0,
                flee_health: 20.0,
            ),

            loot_table: [
                (item: "gold", amount: (15, 25), chance: 1.0),
                (item: "bandit_key", amount: (1, 1), chance: 1.0),
                (item: "iron_sword", amount: (1, 1), chance: 0.5),
            ],

            sprite: "sprites/enemies/bandit_elite.png",
        ),
    ],
)
```

### First Companion

**Hostage → Companion**:

```rust
#[derive(Component)]
struct Companion {
    // Identity
    name: String,               // "Gareth the Archer"
    backstory: String,
    personality: Personality,

    // Combat
    weapon: Weapon,             // Bow (archer)
    stats: CombatStats,
    skills: Vec<Skill>,

    // Relationship
    loyalty: f32,               // 0-100
    friendship: f32,            // 0-100

    // Status
    health: f32,
    morale: f32,
    hunger: f32,

    // Can die permanently
    is_alive: bool,
    death_location: Option<Vec2>,
}

// First companion stats
let first_companion = Companion {
    name: "Gareth".to_string(),
    weapon: Weapon::ShortBow,

    stats: CombatStats {
        health: 60.0,
        damage: 12.0,
        attack_speed: 1.2,
        attack_range: 180.0,   // Archer
    },

    skills: vec![
        Skill::new("Aimed Shot", "High accuracy shot", 5.0),
    ],

    loyalty: 70.0,   // Grateful, but not fully loyal yet
    friendship: 50.0,
    // ...
};
```

**Companion Benefits**:
- ✅ Extra firepower in combat
- ✅ Can be positioned around wagon (like guards)
- ✅ Free (doesn't cost wages)
- ✅ Grows stronger over time
- ✅ Has personality and dialog
- ⚠️ **Can die permanently** (emotional investment)

---

## 8. Equipment System

### Equipment Slots

**Player & Companions**:

```rust
#[derive(Component)]
struct EquipmentSlots {
    head: Option<Equipment>,        // Helmet, hat, hood
    shoulders: Option<Equipment>,   // Pauldrons, cape clasp
    torso: Option<Equipment>,       // Armor, shirt, vest
    hands: Option<Equipment>,       // Gloves, gauntlets
    gloves: Option<Equipment>,      // (separate from hands)
    legs: Option<Equipment>,        // Pants, leg armor
    feet: Option<Equipment>,        // Boots, shoes
    back: Option<Equipment>,        // Backpack, quiver
    cape: Option<Equipment>,        // Cape, cloak

    // Weapons
    main_hand: Option<Weapon>,      // Sword, axe
    off_hand: Option<Weapon>,       // Shield, dagger
    ranged: Option<Weapon>,         // Bow, crossbow
}

#[derive(Clone)]
struct Equipment {
    id: String,
    name: String,
    slot: EquipmentSlot,

    // Stats
    armor: f32,                     // Damage reduction
    stat_bonuses: StatBonuses,

    // Visual
    sprite: String,                 // Sprite overlay
    sprite_layer: u8,               // Render order

    // Properties
    weight: f32,
    value: i32,
    durability: f32,                // 0-100
}

struct StatBonuses {
    health: f32,
    damage: f32,
    speed: f32,
    // ... more stats
}
```

**Example Equipment**:

```ron
// Equipment definitions
(
    equipment: [
        // Head
        (
            id: "leather_cap",
            name: "Leather Cap",
            slot: Head,

            armor: 2.0,
            stat_bonuses: (health: 5.0),

            sprite: "sprites/equipment/leather_cap.png",
            sprite_layer: 10,

            weight: 0.5,
            value: 15,
            durability: 100.0,
        ),

        // Torso
        (
            id: "chain_mail",
            name: "Chain Mail",
            slot: Torso,

            armor: 15.0,
            stat_bonuses: (health: 20.0, speed: -5.0), // Slower

            sprite: "sprites/equipment/chain_mail.png",
            sprite_layer: 5,

            weight: 10.0,
            value: 150,
            durability: 100.0,
        ),

        // Cape
        (
            id: "travelers_cloak",
            name: "Traveler's Cloak",
            slot: Cape,

            armor: 1.0,
            stat_bonuses: (
                cold_resistance: 10.0,  // Weather protection
            ),

            sprite: "sprites/equipment/cloak.png",
            sprite_layer: 1,  // Behind character

            weight: 1.0,
            value: 25,
            durability: 100.0,
        ),
    ],
)
```

### Starter Equipment (Tutorial)

**Player Starting Gear**:

```rust
// What player starts with (prologue)
let starter_equipment = EquipmentSlots {
    head: None,                                // No helmet
    shoulders: None,
    torso: Some(Equipment::load("worn_tunic")), // Basic shirt
    hands: Some(Equipment::load("leather_gloves")),
    gloves: None,
    legs: Some(Equipment::load("cloth_pants")),
    feet: Some(Equipment::load("old_boots")),
    back: Some(Equipment::load("small_backpack")),
    cape: None,

    main_hand: Some(Weapon::load("rusty_sword")),  // Tutorial weapon
    off_hand: None,
    ranged: Some(Weapon::load("short_bow")),       // Tutorial weapon
};
```

**Weapon Stats**:

```ron
(
    weapons: [
        // Rusty Sword (starter)
        (
            id: "rusty_sword",
            name: "Rusty Sword",
            weapon_type: Sword,

            damage: 10.0,           // Low damage (rusty)
            attack_speed: 1.0,
            range: 30.0,            // Melee range

            durability: 50.0,       // Half durability (rusty)
            value: 5,               // Nearly worthless

            sprite: "sprites/weapons/rusty_sword.png",
        ),

        // Short Bow (starter)
        (
            id: "short_bow",
            name: "Short Bow",
            weapon_type: Bow,

            damage: 8.0,
            attack_speed: 1.2,
            range: 180.0,           // Ranged

            ammo_type: Some("arrow"),
            ammo_capacity: 20,      // Starts with 20 arrows

            durability: 80.0,
            value: 25,

            sprite: "sprites/weapons/short_bow.png",
        ),
    ],
)
```

### Horse Equipment (Not in Demo)

**Future Feature**:

```rust
#[derive(Component)]
struct HorseEquipment {
    saddle: Option<Equipment>,      // +comfort, +speed
    armor: Option<Equipment>,       // +protection
    bags: Option<Equipment>,        // +carrying capacity
    horseshoes: Option<Equipment>,  // +speed on roads
}
```

**Not implemented in MVP**, but data structure ready for modding.

---

## 9. Wild Horse Taming

### Finding Wild Horses

**Spawn Conditions**:

```rust
// Wild horses spawn on meadows
fn spawn_wild_horse() -> bool {
    // Conditions:
    // 1. Player has no horse OR horse died
    // 2. On meadow terrain
    // 3. Random chance (10% per minute traveled)
    // 4. Not during combat

    let no_horse = !player.has_horse();
    let on_meadow = terrain == TerrainType::Meadow;
    let random_chance = rand::random::<f32>() < 0.1;

    no_horse && on_meadow && random_chance
}
```

**Visual Indicator**:

```
Player traveling on path
    ↓
Camera pans slightly to show meadow
    ↓
Wild horse visible in distance (grazing)
    ↓
🐴 Icon appears above horse
    ↓
Tutorial prompt: "Wild horse spotted! Approach to tame."
```

### Taming Mechanic

**Taming Mini-Game**:

```
1. APPROACH
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Click on wild horse
       ↓
   Player walks toward horse
       ↓
   If too fast: Horse startles and runs
   Must approach slowly (crouch mode)
       ↓
   Get within 5m range


2. TAMING ATTEMPT
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   ┌─────────────────────────────────────┐
   │  TAMING WILD HORSE                  │
   ├─────────────────────────────────────┤
   │                                     │
   │  [🐴]  ← Horse (nervous)            │
   │   │                                 │
   │   │  Trust: ▓░░░░░░░░░ 10%        │
   │   │                                 │
   │  Press [Space] at right time!      │
   │                                     │
   │  ●━━━━━━━━━━━━━━━━━━●             │
   │        ↑                            │
   │      cursor                         │
   │                                     │
   │  Too early = horse flees            │
   │  Too late = horse flees             │
   │  Perfect = trust increases          │
   └─────────────────────────────────────┘

   Timing mini-game (3-5 attempts needed)
       ↓
   Each success: +20% trust
   Each failure: -10% trust, horse may flee
       ↓
   If trust reaches 60%: Horse tamed!


3. SUCCESS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   "You've tamed the wild horse!"
       ↓
   Horse joins your caravan
       ↓
   Can now pull wagon
       ↓
   Stats: Wild Horse
   - Speed: 1.2x (faster than draft)
   - Stamina: 90 (lower than draft)
   - Morale: 60 (semi-wild, needs care)
       ↓
   Prompt: "Name your horse?"
   (Optional, player can name it)


4. FAILURE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Trust drops to 0
       ↓
   Horse neighs and runs away
       ↓
   "The horse escaped! Maybe you'll find another..."
       ↓
   Horse despawns
       ↓
   Try again at next meadow
```

**Taming Difficulty**:

```rust
fn calculate_taming_difficulty(player: &Player) -> f32 {
    // Base difficulty
    let mut difficulty = 0.5;

    // Modifiers
    if player.has_skill("Animal Handling") {
        difficulty -= 0.2; // Easier with skill
    }

    if player.has_item("carrot") {
        difficulty -= 0.15; // Easier with food
    }

    if player.is_injured() {
        difficulty += 0.1; // Harder when injured
    }

    if horse.is_scared() { // Nearby combat
        difficulty += 0.3; // Much harder
    }

    difficulty.clamp(0.1, 0.9)
}
```

### Wild Horse Types

```ron
(
    wild_horses: [
        // Common wild horse
        (
            id: "wild_brown",
            name: "Wild Brown Horse",
            spawn_chance: 0.7,

            stats: (
                health: 90.0,
                stamina: 90.0,
                speed_multiplier: 1.2,    // Faster
                pulling_power: 0.8,       // Weaker
            ),

            taming_difficulty: 0.5,       // Medium
            morale_starting: 60.0,        // Semi-wild
        ),

        // Rare wild horse (better stats)
        (
            id: "wild_stallion",
            name: "Wild Stallion",
            spawn_chance: 0.2,

            stats: (
                health: 110.0,
                stamina: 100.0,
                speed_multiplier: 1.4,
                pulling_power: 1.0,
            ),

            taming_difficulty: 0.7,       // Hard
            morale_starting: 50.0,
        ),

        // Very rare (exceptional)
        (
            id: "wild_black",
            name: "Wild Black Courser",
            spawn_chance: 0.1,

            stats: (
                health: 120.0,
                stamina: 110.0,
                speed_multiplier: 1.5,    // Very fast
                pulling_power: 1.1,
            ),

            taming_difficulty: 0.9,       // Very hard
            morale_starting: 40.0,        // Hard to trust
        ),
    ],
)
```

**Max Horses**: 2 (from start)

- **With 2 horses**: Faster pulling, shared stamina drain
- **Can swap**: Rest one horse while other pulls
- **Team bonus**: 2 horses = +30% speed, +40% capacity

---

## 10. Technical Implementation

### Core Systems

**1. Horse Stamina Update Loop**:

```rust
fn update_horse_stamina(
    mut horses: Query<&mut Horse>,
    wagon: Query<&Wagon>,
    terrain: Res<CurrentTerrain>,
    time: Res<Time>,
) {
    for mut horse in horses.iter_mut() {
        let wagon = wagon.single();
        let delta = time.delta_secs();

        // Calculate stamina drain
        let base_drain = match wagon.state {
            WagonState::Idle => 0.0,
            WagonState::Walking => -5.0,
            WagonState::Galloping => -15.0,
            _ => 0.0,
        };

        // Apply modifiers
        let terrain_mult = get_terrain_modifier(&terrain);
        let weight_mult = 1.0 + (wagon.cargo_weight / wagon.max_capacity);

        horse.stamina += base_drain * terrain_mult * weight_mult * delta;
        horse.stamina = horse.stamina.clamp(0.0, horse.max_stamina);

        // Update exhaustion
        update_exhaustion(&mut horse, delta);

        // Recovery when idle
        if wagon.state == WagonState::Idle {
            horse.stamina += 10.0 * delta;
        }
    }
}
```

**2. Path Following System**:

```rust
fn follow_path(
    mut wagon: Query<(&mut Transform, &mut Wagon)>,
    current_path: Res<CurrentPath>,
) {
    let (mut transform, mut wagon) = wagon.single_mut();

    // Get next waypoint
    let next_waypoint = current_path.get_next_waypoint(wagon.distance_traveled);

    if let Some(waypoint) = next_waypoint {
        // Calculate direction
        let direction = (waypoint - transform.translation.truncate()).normalize();
        wagon.direction = direction;

        // Move wagon
        let speed = calculate_wagon_speed(&wagon, ...);
        transform.translation += direction.extend(0.0) * speed * time.delta_secs();

        // Update distance
        wagon.distance_traveled += speed * time.delta_secs();
    } else {
        // Reached destination!
        trigger_arrival_event();
    }
}
```

**3. Camping System**:

```rust
fn setup_camp(
    mut commands: Commands,
    wagon: Query<&Transform, With<Wagon>>,
    mut game_state: ResMut<GameState>,
) {
    let wagon_transform = wagon.single();

    // Spawn camp entities
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/camp/tent.png")),
        Transform::from_translation(wagon_transform.translation),
        Camp {
            setup_time: 0.0,
            fire_burning: true,
        },
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/camp/campfire.png")),
        Transform::from_translation(wagon_transform.translation + Vec3::new(20.0, 0.0, 0.0)),
        Campfire { fuel: 100.0 },
    ));

    // Auto-save
    save_game(&game_state);

    // Change state
    *game_state = GameState::Camping;
}
```

**4. Wild Horse Taming**:

```rust
#[derive(Component)]
struct TamingMinigame {
    target_horse: Entity,
    trust: f32,
    attempts: u8,
    cursor_position: f32,
    target_zone: (f32, f32),  // Min, max for success
}

fn update_taming_minigame(
    mut minigame: Query<&mut TamingMinigame>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut taming) = minigame.get_single_mut() {
        // Move cursor
        taming.cursor_position += time.delta_secs() * 0.5;
        if taming.cursor_position > 1.0 {
            taming.cursor_position = 0.0;
        }

        // Check input
        if input.just_pressed(KeyCode::Space) {
            let (min, max) = taming.target_zone;
            if taming.cursor_position >= min && taming.cursor_position <= max {
                // SUCCESS
                taming.trust += 20.0;
                play_sound("horse_neigh_happy");
            } else {
                // FAILURE
                taming.trust -= 10.0;
                taming.attempts += 1;
                play_sound("horse_neigh_scared");
            }

            // Check win/lose
            if taming.trust >= 60.0 {
                taming_success();
            } else if taming.trust <= 0.0 || taming.attempts >= 5 {
                taming_failed();
            }
        }
    }
}
```

---

## Summary

This travel system creates a rich, strategic gameplay loop where:

✅ **Horse management** is critical (stamina, exhaustion, whipping)
✅ **Path choice** matters (safe vs risky, fast vs slow)
✅ **Camping** provides save points and rest
✅ **Tutorial encounter** introduces combat and companions
✅ **Equipment** offers progression and customization
✅ **Wild horses** provide recovery from loss
✅ **Player pulling wagon** creates consequence and challenge

### Key Design Principles

1. **Consequence**: Losing your horse is punishing but recoverable
2. **Strategy**: Whip usage and path choice require planning
3. **Tension**: Exhaustion system creates drama during chases
4. **Reward**: First companion is earned, not given
5. **Moddable**: All data in RON files for easy customization

---

**Next Steps**:
1. Implement horse stamina system
2. Create path following behavior
3. Build camping UI and save system
4. Script first encounter
5. Test and balance

**Status**: Ready for implementation
**Review**: After Phase 1 playtesting
