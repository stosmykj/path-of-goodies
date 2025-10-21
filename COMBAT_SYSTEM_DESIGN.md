# Path of Goodies - Combat System Design

**Version**: 0.2
**Last Updated**: 2025-10-21
**Status**: Detailed Design

---

## Table of Contents

1. [Combat Philosophy](#combat-philosophy)
2. [Real-Time Combat Flow](#real-time-combat-flow)
3. [Player Combat](#player-combat)
4. [Guard Combat](#guard-combat)
5. [Enemy Behavior](#enemy-behavior)
6. [Damage & Health](#damage--health)
7. [Tactical Positioning](#tactical-positioning)
8. [Combat UI](#combat-ui)
9. [Loot & Rewards](#loot--rewards)
10. [Technical Implementation](#technical-implementation)

---

## 1. Combat Philosophy

### Core Principles

**Reverse Tower Defense + Action**:
- You're moving through danger (not defending a static point)
- Enemies approach from all sides
- Real-time combat with pause-to-command option
- Player can fight OR command
- Positioning matters

**Strategic Depth**:
- Choose weapons based on situation
- Position guards effectively
- Know when to fight vs flee
- Resource management (arrows, stamina)

**Tension & Stakes**:
- Guards can die permanently
- Horse can be killed/stolen
- Cargo can be damaged/stolen
- Death means losing progress

---

## 2. Real-Time Combat Flow

### Combat Triggers

```
Traveling on path
    ↓
Event Trigger:
├─ Scripted encounter (tutorial)
├─ Random ambush (based on danger level)
├─ Chase sequence (fleeing bandits)
└─ Player chooses to fight (e.g., rescue NPC)
    ↓
COMBAT INITIATED
```

### Combat Sequence

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PHASE 1: PRE-COMBAT (2-5 seconds)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Visual warning:
├─ Enemies visible in distance (fog of war reveal)
├─ Warning indicator: ⚠️ "AMBUSH AHEAD!"
├─ Wagon slows to 50% speed
├─ Audio cue: Tension music starts
└─ Tutorial prompt (first time): "Prepare for combat!"

Player Actions Available:
├─ Draw weapon (sword/bow)
├─ Position guards (drag & drop)
├─ Use items (buff potions)
├─ Attempt to flee (if fast enough)
└─ Brace for impact (defensive stance)

    ↓

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PHASE 2: ENGAGEMENT (Variable duration)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Wagon STOPS (cannot move during combat)

Enemies approach from multiple directions:
├─ Front: 2 bandits charging
├─ Left: 1 bandit + 1 wolf
├─ Right: 1 archer (ranged)
└─ Rear: (safe, wagon rear guard)

Combat Loop:
┌─────────────────────────────────┐
│ While (enemies alive):          │
│                                 │
│  ├─ Player attacks (manual)     │
│  ├─ Guards attack (auto)        │
│  ├─ Enemies attack wagon/guards │
│  ├─ Check casualties            │
│  ├─ Update health bars          │
│  └─ Check victory/defeat        │
│                                 │
│ Real-time, ~60 FPS              │
└─────────────────────────────────┘

Player Options During Combat:
├─ Fight directly (WASD + mouse/click)
├─ Switch weapons (1=sword, 2=bow)
├─ Use consumables (H=health potion)
├─ Command guards (Right-click guard → command)
├─ Tactical retreat (if overwhelmed)
└─ Pause-to-command (TAB key, optional)

    ↓

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PHASE 3: RESOLUTION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Victory:
├─ All enemies defeated
├─ Victory fanfare
├─ Loot drops appear
├─ XP/reputation gained
├─ Casualties counted (guards died?)
├─ Damage assessed (wagon, horse)
├─ Loot collection UI
└─ Resume travel

Defeat:
├─ Player health → 0
├─ OR wagon destroyed
├─ Death screen
└─ Game over options

Retreat:
├─ Player chooses to flee
├─ Wagon speeds up (whip bonus)
├─ Enemies chase (distance-based)
├─ If escape: Combat ends, cargo may be lost
└─ If caught: Re-engage combat
```

---

## 3. Player Combat

### Weapon Types

```rust
enum WeaponType {
    Melee,      // Sword, axe, dagger
    Ranged,     // Bow, crossbow
    Magic,      // Staff, wand (future)
}

#[derive(Component)]
struct Weapon {
    weapon_type: WeaponType,
    damage: f32,
    attack_speed: f32,      // Attacks per second
    range: f32,             // Pixels
    stamina_cost: f32,      // Per attack

    // Ranged specific
    ammo_type: Option<String>,
    current_ammo: u32,
    reload_time: f32,

    // Melee specific
    knockback: f32,         // Push enemies back
    cleave: bool,           // Hit multiple enemies

    // Durability
    durability: f32,        // 0-100
}
```

### Melee Combat (Sword)

**Mechanics**:

```
Player equipped with sword
    ↓
Left-click or Attack key
    ↓
Attack animation plays
    ↓
Hitbox active during swing (0.2s window)
    ↓
Any enemy in hitbox takes damage
    ↓
Knockback applied
    ↓
Cooldown (based on attack speed)
```

**Attack Patterns**:

```rust
// Simple combo system
enum AttackCombo {
    Light,      // Fast, low damage, 1.5 attacks/sec
    Heavy,      // Slow, high damage, 0.8 attacks/sec
    Spin,       // AoE, hits all nearby, long cooldown
}

// Input: Click repeatedly for light attacks
// Input: Hold click for heavy attack (charge)
// Input: Double-tap direction + click for spin
```

**Melee Stats** (Starter Rusty Sword):

```ron
(
    id: "rusty_sword",
    name: "Rusty Sword",
    weapon_type: Melee,

    damage: 10.0,
    attack_speed: 1.0,      // 1 attack/second
    range: 30.0,            // 30 pixels (close range)
    stamina_cost: 5.0,

    knockback: 15.0,        // Pushes enemies back
    cleave: false,          // Single target

    durability: 50.0,       // Half worn
)
```

### Ranged Combat (Bow)

**Mechanics**:

```
Player equipped with bow
    ↓
Left-click or Attack key
    ↓
Aim indicator appears (mouse direction)
    ↓
Hold to charge (0-1s = 0-100% power)
    ↓
Release to fire arrow
    ↓
Arrow projectile spawns
    ↓
Travels in trajectory
    ↓
Hits enemy or terrain
    ↓
Apply damage
    ↓
Consume 1 arrow
```

**Aiming**:

```
╔════════════════════════════════╗
║     🏹                         ║
║      \                         ║
║       \  ← Aim line            ║
║        \                       ║
║         🎯 Enemy               ║
║                                ║
║  Charge: ▓▓▓░░░░░░ 30%       ║
║  Arrows: 17/20                 ║
╚════════════════════════════════╝

- Mouse aim or WASD directional aim
- Charge for power (longer draw = more damage)
- Lead moving targets (arrows have travel time)
- Gravity affects trajectory
```

**Ranged Stats** (Starter Short Bow):

```ron
(
    id: "short_bow",
    name: "Short Bow",
    weapon_type: Ranged,

    damage: 8.0,            // Base damage
    attack_speed: 1.2,      // Can fire every 0.8s
    range: 180.0,           // Max effective range
    stamina_cost: 3.0,

    ammo_type: Some("arrow"),
    current_ammo: 20,       // Start with 20 arrows
    reload_time: 0.0,       // Individual arrows (no reload)

    // Charged shot multiplier
    charge_damage_mult: 2.0,  // Fully charged = 2x damage
    charge_time: 1.0,         // 1 second to full charge
)
```

### Weapon Switching

**Quick Switch**:

```
Press [1] → Equip sword
Press [2] → Equip bow
Press [3] → Equip shield (if have)

Animation: 0.5s switch time
During switch: Vulnerable (cannot attack)
```

**Strategic Switching**:
- ✅ Sword for close enemies
- ✅ Bow for distant enemies
- ✅ Shield for defense
- ❌ Switching mid-attack cancels attack

---

## 4. Guard Combat

### Auto-Attack System

**Guards fight automatically**:

```rust
fn guard_combat_ai(
    mut guards: Query<(&Transform, &Guard, &mut AttackState)>,
    enemies: Query<(&Transform, &Enemy, &Health)>,
) {
    for (guard_transform, guard, mut attack_state) in guards.iter_mut() {
        // Find nearest enemy in range
        let target = find_nearest_enemy(
            guard_transform.translation,
            guard.attack_range,
            &enemies,
        );

        if let Some(target_entity) = target {
            // Check cooldown
            if attack_state.can_attack() {
                // Attack!
                perform_attack(guard, target_entity);
                attack_state.start_cooldown(1.0 / guard.attack_speed);

                // Animation
                play_animation("guard_attack");
            }
        }
    }
}
```

### Guard Types & Behaviors

**Archer Guard**:

```ron
(
    id: "archer_guard",
    name: "Archer",

    stats: (
        health: 60.0,
        damage: 12.0,
        attack_speed: 1.5,      // 1.5 shots/sec
        attack_range: 200.0,    // Long range
    ),

    behavior: (
        ai_type: Ranged,
        target_priority: Furthest,  // Shoots distant enemies first
        positioning: Rear,          // Prefers wagon rear/sides
    ),

    sprite: "sprites/guards/archer.png",
)
```

**Swordsman Guard**:

```ron
(
    id: "swordsman_guard",
    name: "Swordsman",

    stats: (
        health: 80.0,
        damage: 15.0,
        attack_speed: 1.0,
        attack_range: 40.0,     // Melee
    ),

    behavior: (
        ai_type: Melee,
        target_priority: Closest,   // Charges nearest enemy
        positioning: Front,         // Front line defender
    ),

    abilities: [
        (
            id: "shield_bash",
            cooldown: 8.0,
            effect: "Stuns enemy for 2s",
        ),
    ],
)
```

**Shield Bearer**:

```ron
(
    id: "shield_bearer",
    name: "Shield Bearer",

    stats: (
        health: 100.0,
        damage: 8.0,
        attack_speed: 0.8,
        attack_range: 35.0,
    ),

    behavior: (
        ai_type: Tank,
        target_priority: Strongest,  // Targets elite enemies
        positioning: Front,
    ),

    abilities: [
        (
            id: "defensive_stance",
            cooldown: 15.0,
            effect: "+50% damage reduction for 5s",
        ),
    ],

    armor: 20.0,  // High armor
)
```

### Guard Positioning

**Wagon Defense Grid**:

```
         [Front]
           ╔═╗
    [Left] ║▓║ [Right]
           ║▓║
         [Rear]

Positioning slots (max 4 guards):
- Front: Melee guards (swordsman, shield)
- Rear: Ranged guards (archers)
- Left/Right: Flexible (balanced defense)

Player can drag-drop guards in pre-combat
OR use quick-assign buttons
```

**AI Positioning Logic**:

```rust
fn auto_position_guards(guards: &[Guard]) {
    for guard in guards {
        let preferred_slot = match guard.behavior.positioning {
            Front => get_front_slot(),
            Rear => get_rear_slot(),
            Sides => get_best_side_slot(),
            Flexible => get_any_open_slot(),
        };

        assign_guard_to_slot(guard, preferred_slot);
    }
}
```

### Guard Commands (Advanced)

**Tactical Commands** (Right-click guard):

```
╔════════════════════════════════╗
║ GUARD: Archer                  ║
║ HP: ▓▓▓▓▓░░░░░ 50/60         ║
╠════════════════════════════════╣
║ [HOLD POSITION]  - Stay here  ║
║ [AGGRESSIVE]     - Chase       ║
║ [DEFENSIVE]      - Stay close  ║
║ [USE ABILITY]    - Special     ║
║ [FALL BACK]      - Retreat     ║
╚════════════════════════════════╝
```

**Stance Effects**:
- **Hold**: Guard doesn't move, focuses fire
- **Aggressive**: Charges enemies, high risk
- **Defensive**: Stays near wagon, safe
- **Ability**: Uses special skill if available

---

## 5. Enemy Behavior

### Enemy AI Types

```rust
enum EnemyAI {
    Aggressive,     // Charges directly, ignores damage
    Cautious,       // Retreats when low HP
    Flanker,        // Tries to attack from sides
    Coward,         // Flees if outnumbered
    Berserker,      // Gets stronger when damaged
    Ranged,         // Keeps distance, shoots
}
```

### Enemy Attack Patterns

**Melee Enemies** (Bandits):

```
Spawn at distance (150-200m)
    ↓
Move toward wagon
    ↓
Target priority:
├─ 1. Isolated guards (easy kill)
├─ 2. Player (if visible and close)
├─ 3. Horse (if undefended)
└─ 4. Wagon (last resort)
    ↓
Attack when in range (30-40px)
    ↓
Deal damage to target
    ↓
Repeat until dead or victory
```

**Ranged Enemies** (Bandit Archers):

```
Spawn at distance
    ↓
Move to optimal range (120-150px)
    ↓
Stop and aim
    ↓
Fire arrow at:
├─ Closest guard
└─ Player (if exposed)
    ↓
Retreat if approached by melee
    ↓
Kite and shoot
```

**Elite Enemies** (Bandit Leaders):

```
Higher stats:
├─ More HP (2-3x normal)
├─ More damage (1.5x)
└─ Special abilities

AI:
├─ Targets player preferentially
├─ Uses abilities strategically
├─ Calls for backup (spawns reinforcements)
└─ Doesn't flee (fights to death)

Abilities:
├─ "Rally" - Buffs nearby bandits (+20% damage)
├─ "Charge" - Dash attack with knockback
└─ "Execute" - High damage to low HP targets
```

### Enemy Stats (Examples)

```ron
(
    enemies: [
        // Basic bandit
        (
            id: "bandit_basic",
            name: "Bandit",

            stats: (
                health: 40.0,
                damage: 10.0,
                attack_speed: 1.0,
                movement_speed: 80.0,
            ),

            behavior: (
                ai_type: Aggressive,
                chase_range: 300.0,
                attack_range: 35.0,
                flee_health: 10.0,
            ),

            loot_table: [
                (item: "gold", amount: (3, 8), chance: 1.0),
                (item: "rusty_dagger", amount: (1, 1), chance: 0.3),
            ],
        ),

        // Wolf
        (
            id: "wolf",
            name: "Wolf",

            stats: (
                health: 30.0,
                damage: 12.0,
                attack_speed: 1.5,  // Fast attacks
                movement_speed: 120.0,  // Very fast
            ),

            behavior: (
                ai_type: Flanker,       // Circles around
                chase_range: 250.0,
                attack_range: 25.0,
                flee_health: 8.0,
            ),

            loot_table: [
                (item: "wolf_pelt", amount: (1, 1), chance: 0.8),
                (item: "meat", amount: (1, 2), chance: 0.5),
            ],
        ),

        // Elite bandit leader
        (
            id: "bandit_elite",
            name: "Bandit Leader",

            stats: (
                health: 120.0,
                damage: 20.0,
                attack_speed: 1.2,
                movement_speed: 90.0,
            ),

            behavior: (
                ai_type: Berserker,
                chase_range: 400.0,
                attack_range: 40.0,
                flee_health: 0.0,   // Never flees
            ),

            abilities: [
                (
                    id: "rally_cry",
                    cooldown: 20.0,
                    effect: "Buffs nearby bandits +20% damage for 10s",
                ),
                (
                    id: "power_strike",
                    cooldown: 8.0,
                    effect: "Heavy attack for 2x damage",
                ),
            ],

            loot_table: [
                (item: "gold", amount: (20, 40), chance: 1.0),
                (item: "bandit_key", amount: (1, 1), chance: 1.0),
                (item: "iron_sword", amount: (1, 1), chance: 0.6),
            ],
        ),
    ],
)
```

---

## 6. Damage & Health

### Damage Calculation

```rust
fn calculate_damage(
    attacker: &CombatStats,
    defender: &CombatStats,
    weapon: &Weapon,
    is_critical: bool,
) -> f32 {
    // Base damage
    let base = attacker.damage + weapon.damage;

    // Armor reduction
    let armor_reduction = defender.armor * 0.5; // 50% effectiveness
    let after_armor = (base - armor_reduction).max(1.0); // Min 1 damage

    // Critical hit (10% chance, 2x damage)
    let critical_mult = if is_critical { 2.0 } else { 1.0 };

    // Random variance (±10%)
    let variance = 1.0 + (rand::random::<f32>() - 0.5) * 0.2;

    // Final damage
    after_armor * critical_mult * variance
}
```

### Health & Death

**Health System**:

```rust
#[derive(Component)]
struct Health {
    current: f32,
    max: f32,
    regeneration: f32,  // HP per second (out of combat)
}

fn apply_damage(health: &mut Health, damage: f32) {
    health.current -= damage;
    health.current = health.current.max(0.0);

    // Visual feedback
    show_damage_number(damage);
    play_hit_sound();
    flash_sprite_red();

    // Check death
    if health.current <= 0.0 {
        trigger_death_event();
    }
}
```

**Death States**:

```rust
enum DeathState {
    Alive,
    Dying(f32),     // Animation duration
    Dead,
    Corpse(f32),    // Time until despawn
}

// Player death
fn on_player_death() {
    play_death_animation();
    play_death_sound();
    drop_loot();
    show_game_over_screen();
}

// Guard death (permanent)
fn on_guard_death(guard: Entity) {
    play_death_animation();
    remove_from_wagon_slots(guard);
    show_notification("Guard has fallen!");
    // Guard is gone forever
}

// Enemy death
fn on_enemy_death(enemy: Entity) {
    play_death_animation();
    drop_loot(enemy);
    despawn_after_animation(enemy, 2.0); // Corpse stays 2s
}
```

### Healing

**Healing Sources**:

```ron
(
    consumables: [
        (
            id: "health_potion",
            name: "Health Potion",
            effect: RestoreHealth(50.0),
            duration: Instant,
            cooldown: 5.0,  // Can't spam
        ),
        (
            id: "bandage",
            name: "Bandage",
            effect: RestoreHealth(20.0),
            duration: OverTime(10.0), // Heal 20 over 10 seconds
            cooldown: 0.0,
        ),
        (
            id: "camp_rest",
            name: "Rest at Camp",
            effect: RestoreHealth(999.0), // Full heal
            duration: Instant,
            cooldown: 0.0,
        ),
    ],
)
```

---

## 7. Tactical Positioning

### Positioning Matters

**Attack Directions**:

```
Enemies can approach from 4 directions:

       N (Front)
         ↓
    W ← 🛒 → E
         ↑
       S (Rear)

Undefended sides = vulnerable!
```

**Flanking Bonus**:

```rust
fn calculate_flanking_bonus(
    attacker_pos: Vec2,
    defender_pos: Vec2,
    defender_facing: Vec2,
) -> f32 {
    let angle = calculate_angle(attacker_pos, defender_pos, defender_facing);

    if angle < 45.0 {
        1.0  // Front attack (no bonus)
    } else if angle < 135.0 {
        1.2  // Side attack (+20%)
    } else {
        1.5  // Rear attack (+50% - backstab!)
    }
}
```

**Terrain Use**:

```
Wagon near rocks/trees:
├─ Blocks some attack angles
├─ Enemies must path around
├─ Gives defensive advantage
└─ Strategic positioning!

Wagon in open field:
├─ Enemies attack from all sides
├─ No cover
└─ Very dangerous!
```

### Formation Tactics

**Defensive Formation** (Recommended):

```
        [Archer]
           ║
    [Sword]║[Sword]
           ║
        [Shield]

- Shield front (tanks damage)
- Swords left/right (protect flanks)
- Archer rear (safe, high damage)
```

**Offensive Formation** (Risky):

```
    [Sword][Shield]
           ║
        [Archer]
           ║
        [Player]

- Aggressive front line
- Player fights directly
- High damage but vulnerable
```

**Balanced Formation**:

```
      [Shield]
           ║
    [Archer]║[Sword]
           ║
        [Player]

- Mix of offense and defense
- Player can switch between roles
- Flexible adaptation
```

---

## 8. Combat UI

### HUD During Combat

```
╔════════════════════════════════════════════════════════════╗
║ HP: ▓▓▓▓▓▓░░░░ 60/100    Stamina: ▓▓▓▓▓▓▓▓░░ 80/100    ║
║                                                            ║
║ Weapon: [🗡️ Rusty Sword]  Ammo: 17 🏹                    ║
║                                                            ║
║ ┌──────────────────────────────────────────────────────┐ ║
║ │  ⚠️ COMBAT ⚠️                                        │ ║
║ │                                                       │ ║
║ │  Enemies: 3 🗡️  1 🏹                               │ ║
║ │                                                       │ ║
║ │  Guards:                                             │ ║
║ │  ├─ Archer    HP: ▓▓▓▓▓░░░░░ 50/60                │ ║
║ │  ├─ Swordsman HP: ▓▓▓▓▓▓▓▓░░ 80/100              │ ║
║ │  └─ Shield    HP: ▓▓▓▓▓▓▓▓▓▓ 100/100             │ ║
║ │                                                       │ ║
║ └──────────────────────────────────────────────────────┘ ║
║                                                            ║
║ [1] Sword  [2] Bow  [H] Heal  [TAB] Pause  [R] Retreat   ║
╚════════════════════════════════════════════════════════════╝
```

### Enemy Health Bars

```
Above each enemy:

    [Bandit]
    ▓▓▓▓▓░░░░░ 25/50

Color coding:
- Green: > 60% HP
- Yellow: 30-60% HP
- Red: < 30% HP
- Flashing Red: < 10% HP (execute!)
```

### Damage Numbers

```
    💥 -15      ← Damage dealt to enemy

    🩸 -8       ← Damage taken by player

    ⚡ CRITICAL! -30  ← Critical hit

    🛡️ BLOCKED  ← Shield blocked attack
```

---

## 9. Loot & Rewards

### Loot Drops

**On Enemy Death**:

```rust
fn drop_loot(enemy: &Enemy, position: Vec2) {
    for loot_entry in &enemy.loot_table {
        if rand::random::<f32>() < loot_entry.chance {
            let amount = rand::range(loot_entry.amount.0, loot_entry.amount.1);

            spawn_loot_item(
                loot_entry.item_id,
                amount,
                position + random_offset(),
            );
        }
    }
}
```

**Loot Collection**:

```
After combat ends:

╔════════════════════════════════╗
║ VICTORY!                       ║
╠════════════════════════════════╣
║                                ║
║ Loot collected:                ║
║ ├─ 💰 Gold: 45                ║
║ ├─ 🗡️ Iron Dagger (1)        ║
║ ├─ 🧪 Health Potion (2)       ║
║ └─ 🔑 Bandit Key (1)          ║
║                                ║
║ Experience: +50 XP             ║
║ Reputation: +5                 ║
║                                ║
║      [COLLECT ALL]             ║
╚════════════════════════════════╝

Auto-collected OR manual pickup (player choice in settings)
```

### Post-Combat Summary

```
╔══════════════════════════════════════════════════════╗
║              COMBAT SUMMARY                          ║
╠══════════════════════════════════════════════════════╣
║                                                      ║
║ Enemies Defeated: 5                                 ║
║ ├─ Bandits: 4                                       ║
║ └─ Elite: 1                                         ║
║                                                      ║
║ Casualties:                                          ║
║ ├─ Guards Lost: 0                                   ║
║ ├─ Guards Wounded: 1 (Archer at 40% HP)            ║
║ └─ Horse: Healthy                                   ║
║                                                      ║
║ Rewards:                                             ║
║ ├─ Gold: 45                                         ║
║ ├─ Items: 4                                         ║
║ ├─ Experience: +50 XP                               ║
║ └─ Reputation: +5 (Merchant Guild)                  ║
║                                                      ║
║ Damage Taken:                                        ║
║ ├─ Player: 40 HP lost                               ║
║ ├─ Wagon: 10 durability lost                       ║
║ └─ Cargo: Intact                                    ║
║                                                      ║
║              [CONTINUE JOURNEY]                      ║
╚══════════════════════════════════════════════════════╝
```

---

## 10. Technical Implementation

### Combat State Machine

```rust
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum CombatState {
    #[default]
    NoCombat,
    PreCombat,      // Warning phase
    InCombat,       // Active fighting
    PostCombat,     // Loot collection
    Fleeing,        // Attempting escape
}

fn setup_combat_systems(app: &mut App) {
    app
        .add_systems(OnEnter(CombatState::PreCombat), (
            slow_wagon,
            show_warning_ui,
            spawn_enemies,
        ))
        .add_systems(Update, (
            player_combat_system,
            guard_combat_system,
            enemy_ai_system,
            damage_system,
            health_system,
            loot_drop_system,
        ).run_if(in_state(CombatState::InCombat)))
        .add_systems(OnExit(CombatState::InCombat), (
            collect_loot,
            show_summary,
            resume_travel,
        ));
}
```

### Attack System

```rust
fn player_attack_system(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&Transform, &mut Player, &mut Weapon)>,
    enemies: Query<(Entity, &Transform, &Health), With<Enemy>>,
) {
    if let Ok((player_transform, mut player, mut weapon)) = player.get_single_mut() {
        // Check if attack key pressed
        if mouse_input.just_pressed(MouseButton::Left) ||
           keyboard_input.just_pressed(KeyCode::Space) {

            // Check cooldown
            if !player.can_attack() {
                return;
            }

            match weapon.weapon_type {
                WeaponType::Melee => {
                    // Melee attack - hit nearby enemies
                    for (enemy_entity, enemy_transform, enemy_health) in enemies.iter() {
                        let distance = player_transform.translation.distance(
                            enemy_transform.translation
                        );

                        if distance < weapon.range {
                            // Hit!
                            let damage = calculate_damage(&player, &weapon);
                            commands.trigger_targets(
                                DamageEvent { amount: damage },
                                enemy_entity,
                            );

                            // Play effects
                            spawn_hit_effect(enemy_transform.translation);
                            play_sound("sword_hit");
                        }
                    }

                    // Start cooldown
                    player.start_attack_cooldown(1.0 / weapon.attack_speed);
                },

                WeaponType::Ranged => {
                    // Ranged attack - spawn projectile
                    if weapon.current_ammo > 0 {
                        spawn_arrow_projectile(
                            player_transform.translation,
                            get_mouse_direction(),
                            weapon.damage,
                        );

                        weapon.current_ammo -= 1;
                        player.start_attack_cooldown(1.0 / weapon.attack_speed);

                        play_sound("bow_shoot");
                    } else {
                        show_message("Out of arrows!");
                    }
                },
            }
        }
    }
}
```

### Guard Auto-Combat

```rust
fn guard_auto_combat_system(
    mut guards: Query<(&Transform, &Guard, &mut AttackCooldown)>,
    mut enemies: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (guard_transform, guard, mut cooldown) in guards.iter_mut() {
        // Update cooldown
        cooldown.tick(time.delta());

        if !cooldown.ready() {
            continue;
        }

        // Find target
        let target = find_nearest_enemy_in_range(
            guard_transform.translation,
            guard.attack_range,
            &enemies,
        );

        if let Some((target_entity, target_transform, _)) = target {
            // Attack!
            let damage = guard.damage;

            commands.trigger_targets(
                DamageEvent { amount: damage },
                target_entity,
            );

            // Visual feedback
            spawn_attack_effect(
                guard_transform.translation,
                target_transform.translation,
                guard.weapon_type,
            );

            // Start cooldown
            cooldown.reset(1.0 / guard.attack_speed);
        }
    }
}
```

---

## Summary

This real-time combat system provides:

✅ **Dynamic Combat**: Real-time action with tactical depth
✅ **Player Choice**: Fight directly or command guards
✅ **Strategic Positioning**: Flanking, formations matter
✅ **Weapon Variety**: Melee vs ranged, each unique
✅ **AI Behaviors**: Diverse enemy types with different tactics
✅ **Stakes**: Guards can die, cargo can be lost
✅ **Rewards**: Loot, XP, reputation from victories
✅ **Moddable**: All stats and behaviors in RON files

**Next**: Resource consumption and encounter brainstorming!

---

**Status**: Ready for implementation
**Dependencies**: Movement system, sprite animation
**Priority**: Phase 1 (Week 6 - Basic Combat)
