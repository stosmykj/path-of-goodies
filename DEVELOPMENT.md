# Path of Goodies - Development Guide

**Status**: Phase 1 Core Systems - In Progress
**Last Updated**: 2025-10-21
**Bevy Version**: 0.15

---

## Current Implementation Status

### ✅ Completed (Phase 1 - Week 1)

#### Core Architecture
- [x] Modular ECS component structure
- [x] Game state management system
- [x] Resource management framework
- [x] System organization and separation

#### Components (`src/components.rs`)
- [x] Wagon component (durability, cargo, speed)
- [x] Horse component (health, stamina, exhaustion, morale)
- [x] PlayerInventory (gold, food, water)
- [x] TravelState (movement tracking)
- [x] PartyMember (for guards/companions)
- [x] Velocity and MainCamera markers

#### Resources (`src/resources.rs`)
- [x] GameState enum (MainMenu, Traveling, Combat, Town, etc.)
- [x] GameTime (day/night cycle, time progression)
- [x] TravelInfo (destination, distance, progress)
- [x] GameSettings (time scale, debug mode)

#### Systems

**Movement System** (`src/systems/movement.rs`):
- [x] WASD/Arrow key wagon movement
- [x] Horse stamina system (drains while moving)
- [x] Exhaustion mechanics (builds when stamina low)
- [x] Speed multiplier based on horse condition
- [x] Camera follow with smooth lerp

**UI System** (`src/systems/ui.rs`):
- [x] HUD root container
- [x] Resources display (gold, food, water)
- [x] Time display (day and hour)
- [x] Horse status display (health, stamina, speed)
- [x] Travel info display (destination, progress)
- [x] Real-time UI updates

**Time System** (`src/systems/time_system.rs`):
- [x] Game time progression (1 real second = 1 game minute)
- [x] Day/night cycle
- [x] Time of day states (Dawn, Day, Dusk, Night)
- [x] Ambient lighting changes based on time

**World System** (`src/systems/world.rs`):
- [x] Procedural world tile generation
- [x] Multiple tile types (Grass, Road, Forest, Water, Mountain)
- [x] 32x32 tile grid
- [x] Color-coded placeholder tiles

---

## How to Build and Run

### Prerequisites

- Rust 1.70+ (stable)
- Cargo

### Development Build

```bash
# Standard debug build
cargo run

# Fast compilation (for rapid iteration)
cargo run --profile dev-fast
```

### Release Build

```bash
# Optimized release build
cargo run --release
```

### Web (WASM) Build

```bash
# Install trunk if you haven't already
cargo install trunk

# Build and serve locally
trunk serve

# Build for production
trunk build --release
```

---

## Controls

**Movement**:
- `W` / `↑` - Move North
- `S` / `↓` - Move South
- `A` / `←` - Move West
- `D` / `→` - Move East

**Coming Soon**:
- `Space` - Whip horse (speed boost)
- `C` - Set up camp
- `I` - Inventory
- `M` - Map
- `ESC` - Pause menu

---

## Current Features

### Playable Features
1. **Wagon Movement**: Move around the world with WASD/arrows
2. **Horse Stamina**: Watch your horse's stamina drain as you travel
3. **Day/Night Cycle**: Time progresses (configurable speed)
4. **Resource Tracking**: Gold, food, water displayed in HUD
5. **Procedural World**: Random terrain generation

### Visual Features
- Placeholder sprites (colored squares)
- Camera follows wagon smoothly
- Dynamic background color based on time of day
- HUD with resource information

---

## Project Structure

```
src/
├── main.rs              # App initialization and setup
├── components.rs        # All ECS components
├── resources.rs         # Global game resources
└── systems/
    ├── mod.rs           # Systems module
    ├── movement.rs      # Wagon/horse movement logic
    ├── ui.rs            # HUD and UI systems
    ├── time_system.rs   # Game time and day/night
    └── world.rs         # World generation

docs/                    # Design documentation
├── GAME_DESIGN.md
├── ARCHITECTURE.md
├── TRAVEL_SYSTEM_DESIGN.md
├── COMBAT_SYSTEM_DESIGN.md
├── ENCOUNTERS_BRAINSTORM.md
├── ADDITIONAL_MECHANICS_BRAINSTORM.md
└── RESOURCE_CONSUMPTION.md

mods/                    # Modding system (data-driven)
└── example_mod/
    ├── mod.ron
    └── data/
        └── guards.ron

ROADMAP.md              # Development roadmap
ART_STYLE_DECISION.md   # Art specifications
MODDING_SYSTEM.md       # Modding architecture
```

---

## Next Steps (Phase 1 Completion)

### Week 2 Tasks
- [ ] Add RON data loading system
- [ ] Implement basic encounter system
- [ ] Add destination/city system
- [ ] Create simple contract/delivery system
- [ ] Add basic save/load functionality

### Week 3-4 Tasks
- [ ] Implement combat system (real-time)
- [ ] Add guard system (hiring, equipment)
- [ ] Create enemy AI (bandits, wildlife)
- [ ] Add camping mechanics
- [ ] Implement resource consumption

---

## Known Issues

1. **Network Issue**: Cannot currently download dependencies from crates.io (temporary)
2. **Placeholder Art**: All visuals are colored squares (by design for MVP)
3. **No Collisions**: Can move through all terrain types (planned feature)
4. **Game State**: Currently defaults to Traveling state on launch

---

## Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=info cargo run

# Run with debug mode
RUST_LOG=debug cargo run
```

---

## Performance

**Target FPS**: 60 FPS
**Current Performance**: Excellent (simple 2D rendering)

**Optimization Notes**:
- Dev builds optimize dependencies (faster runtime)
- WASM builds use size optimization
- Entity count currently low (<1000 tiles)

---

## Contributing

See design documents in `/docs` for implementation guidelines.

**Key Principles**:
1. Everything is data-driven (use RON files)
2. All content is moddable
3. ECS architecture (no large monolithic systems)
4. Cross-platform (native + WASM)

---

## Debug Mode

Debug mode is enabled by default in `GameSettings`. This provides:
- Extra logging
- Performance metrics
- Debug overlays (coming soon)

Disable in production by setting `debug_mode: false` in GameSettings.

---

## Resources

- [Bevy Book](https://bevyengine.org/learn/book/)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [RON Format](https://github.com/ron-rs/ron)
- [Our Game Design Doc](docs/GAME_DESIGN.md)

---

**Happy Coding!** 🎮🦀
