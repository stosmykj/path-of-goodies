# Path of Goodies - Technical Architecture

**Version**: 0.1
**Last Updated**: 2025-10-21

This document outlines the technical architecture for Path of Goodies, with special focus on the multiplayer and networking systems.

---

## Table of Contents

1. [System Overview](#system-overview)
2. [ECS Architecture](#ecs-architecture)
3. [Networking & Multiplayer](#networking--multiplayer)
4. [Procedural Generation](#procedural-generation)
5. [Save/Load System](#saveload-system)
6. [Performance Considerations](#performance-considerations)
7. [Platform-Specific Implementations](#platform-specific-implementations)

---

## 1. System Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Presentation Layer                │
│  ┌─────────────┐  ┌─────────────┐  ┌──────────────┐│
│  │  Rendering  │  │     UI      │  │    Audio     ││
│  └─────────────┘  └─────────────┘  └──────────────┘│
└─────────────────────────────────────────────────────┘
                        │
┌─────────────────────────────────────────────────────┐
│                   Game Logic Layer                  │
│  ┌─────────────┐  ┌─────────────┐  ┌──────────────┐│
│  │   Systems   │  │ Components  │  │  Resources   ││
│  │  (Bevy ECS) │  │ (Bevy ECS)  │  │  (Bevy ECS)  ││
│  └─────────────┘  └─────────────┘  └──────────────┘│
└─────────────────────────────────────────────────────┘
                        │
┌─────────────────────────────────────────────────────┐
│                   Data Layer                        │
│  ┌─────────────┐  ┌─────────────┐  ┌──────────────┐│
│  │  World Gen  │  │  Save/Load  │  │  Networking  ││
│  │  (Noise)    │  │   (RON)     │  │  (Replicon)  ││
│  └─────────────┘  └─────────────┘  └──────────────┘│
└─────────────────────────────────────────────────────┘
```

### Technology Stack

- **Engine**: Bevy 0.15+ (ECS game engine)
- **Language**: Rust 2021
- **Rendering**: Bevy's 2D renderer (wgpu-based)
- **Networking**: bevy_replicon + renet (Phase 4+)
- **Procedural Generation**: noise-rs (Perlin/Simplex)
- **Serialization**: serde + RON format
- **Build**: cargo + trunk (WASM)

---

## 2. ECS Architecture

### Entity Types

#### Player Caravan
```rust
// Main player entity
Entity {
    Wagon,              // Core caravan component
    Transform,          // Position in world
    Velocity,           // Movement
    Health,             // Caravan durability
    Inventory,          // Cargo and supplies
    Resources,          // Food, water, gold
    PlayerControlled,   // Marks as player entity
    Replicated,         // (Multiplayer) Synced to other clients
}
```

#### Guard/Companion
```rust
Entity {
    Guard,              // Guard-specific data
    Transform,          // Position relative to wagon
    Health,
    Damage,
    AttackRange,
    Morale,
    SlotPosition,       // Where on wagon (front/back/left/right)
    Parent(wagon_id),   // Attached to wagon entity
    Replicated,         // (Multiplayer)
}
```

#### Enemy
```rust
Entity {
    Enemy,
    EnemyType,          // Bandit, Wolf, Bear, etc.
    Transform,
    Velocity,
    Health,
    Damage,
    AIState,            // Chase, Attack, Flee
    Target(entity_id),  // What they're attacking
    Replicated,         // (Multiplayer) Server-authoritative
}
```

#### World Objects
```rust
// Cities, settlements, POIs
Entity {
    Settlement,         // City, town, outpost
    Transform,
    SettlementType,
    Economy,            // Prices, contracts available
    Sprite,
    Replicated,         // (Multiplayer) Static, sync once
}
```

### Component Definitions

```rust
// Core gameplay components
#[derive(Component)]
struct Wagon {
    speed: f32,
    max_speed: f32,
    weight_capacity: f32,
    armor: f32,
    guard_slots: u8,
}

#[derive(Component)]
struct Resources {
    food: f32,
    water: f32,
    gold: i32,
}

#[derive(Component)]
struct Inventory {
    items: Vec<Item>,
    cargo: Option<Cargo>,
    max_weight: f32,
}

#[derive(Component)]
struct Guard {
    guard_type: GuardType,  // Archer, Swordsman, etc.
    attack_damage: f32,
    attack_speed: f32,
    attack_range: f32,
    morale: f32,
    wages: i32,  // Cost per delivery
}

#[derive(Component)]
struct Contract {
    id: u32,
    cargo_type: CargoType,
    origin: Entity,  // Settlement entity
    destination: Entity,
    payment: i32,
    time_limit: Option<f32>,
    difficulty: Difficulty,
}

#[derive(Component)]
struct Health {
    current: f32,
    max: f32,
}

// Multiplayer components
#[derive(Component)]
struct PlayerControlled {
    player_id: u64,
}

#[derive(Component)]
struct Guild {
    guild_id: u64,
    name: String,
    members: Vec<u64>,
    warehouse: Inventory,
}
```

### Resource Definitions

```rust
// Global game state
#[derive(Resource)]
struct WorldState {
    seed: u64,
    biome_map: HashMap<IVec2, Biome>,
    settlements: Vec<Entity>,
    discovered_areas: HashSet<IVec2>,
}

#[derive(Resource)]
struct GameTime {
    elapsed: f32,
    day_night_cycle: f32,  // 0.0 = midnight, 0.5 = noon
}

#[derive(Resource)]
struct PlayerProgress {
    completed_contracts: u32,
    total_earnings: i32,
    reputation: HashMap<Faction, i32>,
    unlocks: HashSet<Unlock>,
}

#[derive(Resource)]
struct ContractPool {
    available_contracts: Vec<Contract>,
    refresh_timer: f32,
}

// Multiplayer resources
#[derive(Resource)]
struct NetworkMode {
    mode: Mode,  // Offline, LAN, Online
    server_url: Option<String>,
}

#[derive(Resource)]
struct GuildRegistry {
    guilds: HashMap<u64, Guild>,
}
```

### System Execution Order

```rust
// Bevy system sets for proper ordering
App::new()
    .add_systems(Update, (
        // Phase 1: Input
        input_system,

        // Phase 2: Game Logic
        (
            resource_consumption_system,
            contract_system,
            encounter_spawn_system,
        ).chain(),

        // Phase 3: Movement & Physics
        (
            movement_system,
            pathfinding_system,
            collision_system,
        ).chain(),

        // Phase 4: Combat
        (
            targeting_system,
            combat_system,
            damage_system,
            death_system,
        ).chain(),

        // Phase 5: Networking (if multiplayer)
        (
            send_client_updates,
            receive_server_updates,
        ).chain(),

        // Phase 6: Rendering & UI
        (
            camera_follow_system,
            sprite_animation_system,
            ui_update_system,
        ),
    ))
```

---

## 3. Networking & Multiplayer

### Architecture Choice: Client-Server

**Authoritative Server Model**
- Server owns game state (world, entities, events)
- Clients send inputs only
- Server validates and broadcasts state changes
- Prevents cheating, ensures consistency

### Network Stack

```
Client (WASM/Native)
    ↓ (WebSocket/UDP)
bevy_replicon (ECS replication)
    ↓
renet (Transport layer)
    ↓
Server (Native only)
```

### Replication Strategy

#### What Gets Replicated?

**Full Replication (all clients see)**:
- Player caravans (position, health, cargo)
- Settlements and static world objects
- Active combat encounters
- Guild information

**Partial Replication (nearby only)**:
- Enemies (only if in same chunk)
- Random events (only affect local players)
- Loot drops

**Server-Only (not replicated)**:
- Player inventories (sent only to owner)
- Contract pools (sent on request)
- World generation data (deterministic, clients generate locally)

#### Replication Configuration

```rust
use bevy_replicon::prelude::*;

// Mark components for replication
app.replicate::<Transform>()
   .replicate::<Health>()
   .replicate::<Wagon>()
   .replicate::<Guard>()
   .replicate::<Enemy>();

// Configure replication rules
app.add_systems(Update, (
    replicate_in_range::<Enemy>(REPLICATION_RANGE),
    replicate_to_owner::<Inventory>(),
    replicate_to_guild::<GuildWarehouse>(),
));
```

### Client-Server Communication

#### Client → Server Messages

```rust
#[derive(Serialize, Deserialize)]
enum ClientMessage {
    // Input
    Move(Vec2),                    // Movement direction
    Interact(Entity),              // Click on entity/UI

    // Actions
    AcceptContract(u32),
    HireGuard(GuardType, u8),      // Type and slot
    PositionGuard(u8, SlotPos),

    // Social
    TradeRequest(u64),             // Player ID
    GuildInvite(u64),
    ChatMessage(String),
}
```

#### Server → Client Messages

```rust
#[derive(Serialize, Deserialize)]
enum ServerMessage {
    // State updates (automatic via bevy_replicon)
    // Manual events:

    ContractComplete(u32, i32),    // Contract ID, reward
    EncounterStart(EncounterType),
    ResourceUpdate(Resources),

    // Multiplayer events
    PlayerJoined(u64, String),     // ID, name
    PlayerLeft(u64),
    GuildUpdate(Guild),
    ChatBroadcast(u64, String),    // Sender ID, message
}
```

### Synchronization Strategies

#### Position Sync (Player Caravans)

**Client-Side Prediction**:
1. Client sends input to server
2. Client immediately applies input locally (predict)
3. Server validates and broadcasts true position
4. Client reconciles if mismatch (snap or interpolate)

```rust
// Client prediction
fn client_movement_system(
    mut query: Query<(&mut Transform, &Velocity), With<PlayerControlled>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        // Apply movement immediately
        transform.translation += velocity.0 * time.delta_secs();
    }
}

// Server reconciliation
fn server_reconciliation_system(
    mut events: EventReader<ReplicationUpdate<Transform>>,
    mut query: Query<&mut Transform>,
) {
    for event in events.read() {
        if let Ok(mut transform) = query.get_mut(event.entity) {
            // Server says we're actually here
            *transform = event.component.clone();
        }
    }
}
```

#### Combat Sync (Server-Authoritative)

- All combat calculations on server
- Server broadcasts damage events
- Clients play animations/effects on event receipt
- No client-side prediction for combat (prevent cheating)

```rust
// Server only
fn server_combat_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Damage, &AttackRange, &Target)>,
    mut targets: Query<&mut Health>,
) {
    for (entity, transform, damage, range, target) in query.iter() {
        if let Ok(mut health) = targets.get_mut(target.0) {
            // Calculate hit
            health.current -= damage.0;

            // Send damage event to all clients
            commands.trigger(DamageEvent {
                attacker: entity,
                target: target.0,
                amount: damage.0,
            });
        }
    }
}
```

### World State Synchronization

#### Deterministic World Generation

- Server generates world with seed
- Server sends seed to clients on join
- Clients generate identical world locally
- Only deltas (changes) are synchronized

**Benefits**:
- Minimal network bandwidth
- Fast client join (no huge world download)
- Procedural content scales infinitely

```rust
// Server sends seed
#[derive(Serialize, Deserialize)]
struct JoinResponse {
    player_id: u64,
    world_seed: u64,
    current_settlements: Vec<SettlementData>,
}

// Client generates world
fn client_world_gen(seed: u64) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    // Generate identical world to server
    generate_terrain(&mut rng);
    generate_settlements(&mut rng);
}
```

#### Persistent World State

- Server saves world state periodically
- Dead caravans, guild warehouses, economy data
- Restore on server restart

```rust
#[derive(Serialize, Deserialize)]
struct ServerWorldState {
    seed: u64,
    time_elapsed: f32,
    settlements: Vec<SettlementState>,
    dead_caravans: Vec<DeadCaravanMarker>,
    guilds: HashMap<u64, Guild>,
    economy: EconomyState,
}
```

### Guild System (Multiplayer)

#### Guild Data Structure

```rust
#[derive(Component, Serialize, Deserialize)]
struct Guild {
    id: u64,
    name: String,
    leader: u64,                    // Player ID
    members: Vec<u64>,              // Player IDs
    warehouse: Inventory,           // Shared storage
    funds: i32,                     // Shared gold
    reputation: i32,
    perks: Vec<GuildPerk>,
}

#[derive(Serialize, Deserialize)]
enum GuildPerk {
    SharedMapKnowledge,  // All members see discovered areas
    BulkDiscounts,       // Lower prices at shops
    ExclusiveContracts,  // Access to high-tier deliveries
    SafeHouse,           // Guild hall in major city
}
```

#### Guild Operations

**Creation**:
1. Client requests guild creation
2. Server validates (name unique, cost paid)
3. Server creates guild entity
4. Server broadcasts to all clients

**Joining**:
1. Guild leader sends invite
2. Target player accepts
3. Server adds to member list
4. Server replicates update to guild members

**Warehouse Access**:
1. Member requests item withdrawal
2. Server checks permissions
3. Server updates inventory
4. Replicate to all guild members

### PvP & Anti-Griefing

#### Reputation System

```rust
#[derive(Component)]
struct Reputation {
    value: i32,  // -1000 to +1000
    kills: u32,
    thefts: u32,
    bounty: i32,
}

// Reputation affects:
// - NPC guard aggro (low rep = attacked on sight)
// - Shop prices (higher for criminals)
// - Bounty hunters spawn (if bounty > 0)
```

#### PvP Zones

- **Safe Zones** (cities): PvP disabled, guards protect
- **Trade Routes**: PvP enabled but penalized (reputation loss)
- **Wilderness**: PvP enabled, lower penalties
- **Outlaw Zones**: PvP encouraged, no penalties

#### Theft Mechanics

```rust
// Steal cargo from another player
fn pvp_theft_system(
    mut commands: Commands,
    attacker: Query<(&Transform, &PlayerControlled), With<StealAttempt>>,
    mut victim: Query<(&Transform, &mut Inventory), Without<StealAttempt>>,
) {
    // 1. Attacker must be in range
    // 2. Victim must be defeated (health < 20%)
    // 3. Cargo transferred to attacker
    // 4. Reputation penalty applied
    // 5. Bounty added

    // Server-side only, prevents cheating
}
```

---

## 4. Procedural Generation

### World Generation Pipeline

```
1. Generate Base Terrain (Noise)
    ↓
2. Place Major Settlements (Fixed positions)
    ↓
3. Generate Road Network (Pathfinding)
    ↓
4. Distribute Biomes (Moisture + Temperature)
    ↓
5. Place Minor POIs (Random scatter)
    ↓
6. Populate Encounters (Dynamic spawning)
```

### Terrain Generation

```rust
use noise::{NoiseFn, Perlin, Seedable};

fn generate_terrain(seed: u64) -> HashMap<IVec2, TerrainType> {
    let perlin = Perlin::new().set_seed(seed as u32);
    let mut terrain = HashMap::new();

    for x in -WORLD_SIZE..WORLD_SIZE {
        for y in -WORLD_SIZE..WORLD_SIZE {
            let nx = x as f64 / WORLD_SIZE as f64;
            let ny = y as f64 / WORLD_SIZE as f64;

            let elevation = perlin.get([nx, ny]);
            let moisture = perlin.get([nx + 100.0, ny + 100.0]);

            let terrain_type = match (elevation, moisture) {
                (e, _) if e < -0.2 => TerrainType::Water,
                (e, m) if e < 0.2 && m > 0.3 => TerrainType::Swamp,
                (e, m) if e > 0.5 => TerrainType::Mountain,
                (_, m) if m < -0.3 => TerrainType::Desert,
                (_, m) if m > 0.4 => TerrainType::Forest,
                _ => TerrainType::Grassland,
            };

            terrain.insert(IVec2::new(x, y), terrain_type);
        }
    }

    terrain
}
```

### Settlement Placement

```rust
// Fixed major cities (same on all servers/saves)
const MAJOR_CITIES: &[(i32, i32, &str)] = &[
    (0, 0, "Capital"),
    (100, 50, "Northport"),
    (-80, 60, "Westholm"),
    (50, -90, "Southaven"),
    (-70, -70, "Eastmarch"),
];

// Random minor settlements (seeded)
fn place_minor_settlements(seed: u64, count: u32) -> Vec<Settlement> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut settlements = Vec::new();

    for _ in 0..count {
        let x = rng.gen_range(-WORLD_SIZE..WORLD_SIZE);
        let y = rng.gen_range(-WORLD_SIZE..WORLD_SIZE);
        // Check not too close to others, on valid terrain, etc.
        settlements.push(Settlement { x, y, type: SettlementType::Village });
    }

    settlements
}
```

### Dynamic Encounter Spawning

```rust
fn spawn_encounters(
    mut commands: Commands,
    query: Query<&Transform, With<PlayerControlled>>,
    time: Res<Time>,
    mut spawn_timer: ResMut<EncounterSpawnTimer>,
    world: Res<WorldState>,
) {
    spawn_timer.tick(time.delta());

    if !spawn_timer.finished() {
        return;
    }

    for player_transform in query.iter() {
        let chunk = world_to_chunk(player_transform.translation);
        let danger = world.get_danger_level(chunk);

        // Spawn chance based on biome danger
        if rand::random::<f32>() < danger * 0.1 {
            let encounter_type = choose_encounter(danger);
            spawn_encounter(&mut commands, encounter_type, chunk);
        }
    }

    spawn_timer.reset();
}
```

---

## 5. Save/Load System

### Save File Format (RON)

```ron
// save_data.ron
(
    version: "0.1",
    player: (
        gold: 500,
        food: 50.0,
        water: 30.0,
        reputation: {
            "Merchants": 20,
            "Guards": 10,
            "Outlaws": -30,
        },
        unlocks: ["FastWagon", "GuardSlot3"],
    ),
    wagon: (
        speed: 120.0,
        armor: 50.0,
        capacity: 500.0,
        guard_slots: 4,
    ),
    world: (
        seed: 12345678,
        discovered_chunks: [(0, 0), (1, 0), (1, 1)],
    ),
    active_contract: Some((
        id: 42,
        cargo: "Grain",
        destination: "Northport",
        payment: 200,
    )),
    // Multiplayer data not saved (server-side)
)
```

### Save System Implementation

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SaveData {
    version: String,
    player: PlayerData,
    wagon: WagonData,
    world: WorldData,
    active_contract: Option<ContractData>,
}

fn save_game(world: &World) -> Result<(), SaveError> {
    let save_data = extract_save_data(world);
    let ron_string = ron::ser::to_string_pretty(&save_data, Default::default())?;

    #[cfg(not(target_arch = "wasm32"))]
    std::fs::write("save_data.ron", ron_string)?;

    #[cfg(target_arch = "wasm32")]
    web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .set_item("path_of_goodies_save", &ron_string)?;

    Ok(())
}

fn load_game(world: &mut World) -> Result<(), SaveError> {
    let ron_string = {
        #cfg(not(target_arch = "wasm32"))
        std::fs::read_to_string("save_data.ron")?;

        #cfg(target_arch = "wasm32"))
        web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("path_of_goodies_save")?
            .ok_or(SaveError::NoSaveFound)?
    };

    let save_data: SaveData = ron::de::from_str(&ron_string)?;
    apply_save_data(world, save_data);

    Ok(())
}
```

---

## 6. Performance Considerations

### WASM Optimization

```toml
# Cargo.toml
[profile.wasm-release]
inherits = "release"
opt-level = "z"        # Optimize for size
lto = "fat"            # Link-time optimization
codegen-units = 1      # Single codegen unit
strip = true           # Strip debug symbols
panic = "abort"        # Smaller panic handler
```

### Chunk-Based Rendering

```rust
const CHUNK_SIZE: i32 = 32;  // Tiles per chunk
const RENDER_DISTANCE: i32 = 3;  // Chunks around player

fn culling_system(
    player: Query<&Transform, With<PlayerControlled>>,
    mut chunks: Query<(&ChunkPosition, &mut Visibility)>,
) {
    if let Ok(player_transform) = player.get_single() {
        let player_chunk = world_to_chunk(player_transform.translation);

        for (chunk_pos, mut visibility) in chunks.iter_mut() {
            let distance = player_chunk.distance(chunk_pos.0);

            if distance <= RENDER_DISTANCE {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
```

### Entity Pooling

```rust
// Reuse entities instead of spawning/despawning
#[derive(Resource)]
struct EnemyPool {
    inactive: Vec<Entity>,
}

fn spawn_or_reuse_enemy(
    commands: &mut Commands,
    pool: &mut ResMut<EnemyPool>,
    enemy_type: EnemyType,
) -> Entity {
    if let Some(entity) = pool.inactive.pop() {
        // Reuse existing entity
        commands.entity(entity).insert((
            enemy_type,
            Visibility::Visible,
        ));
        entity
    } else {
        // Spawn new
        commands.spawn((
            Enemy,
            enemy_type,
            Transform::default(),
            // ...
        )).id()
    }
}
```

---

## 7. Platform-Specific Implementations

### WASM vs Native Differences

```rust
// Conditional compilation
#[cfg(target_arch = "wasm32")]
fn platform_specific_setup(app: &mut App) {
    // Web-specific
    app.add_systems(Update, handle_touch_input);
    app.insert_resource(NetworkMode::WebSocket);
}

#[cfg(not(target_arch = "wasm32"))]
fn platform_specific_setup(app: &mut App) {
    // Desktop-specific
    app.add_systems(Update, handle_gamepad_input);
    app.insert_resource(NetworkMode::UDP);
}
```

### Network Transport

```rust
// WASM: WebSocket only
#[cfg(target_arch = "wasm32")]
fn create_client() -> RenetClient {
    RenetClient::new(WebSocketClientTransport::new(server_url))
}

// Native: UDP for low latency
#[cfg(not(target_arch = "wasm32"))]
fn create_client() -> RenetClient {
    RenetClient::new(UdpClientTransport::new(server_addr))
}
```

---

## Summary

This architecture prioritizes:
1. **Scalability** - ECS design scales to 1000s of entities
2. **Cross-platform** - Same code runs on web and desktop
3. **Multiplayer-ready** - Client-server architecture prevents cheating
4. **Performance** - Chunk-based rendering, entity pooling
5. **Deterministic** - Seeded world generation reduces network load

**Next Steps**:
1. Implement Phase 1 systems (movement, resources, contracts)
2. Test WASM performance early
3. Prototype multiplayer in Phase 4
4. Iterate based on playtesting

---

*Last Updated*: 2025-10-21
*Review Cycle*: Monthly during active development
