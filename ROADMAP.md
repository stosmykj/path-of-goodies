# Path of Goodies - Development Roadmap

**Game Type**: Reverse Tower Defense Trading Simulator
**Genre**: Strategy, Roguelike, Multiplayer
**Version**: 0.1 (Pre-Alpha)
**Last Updated**: 2025-10-21

> See [GAME_DESIGN.md](GAME_DESIGN.md) for complete game design documentation

---

## Project Overview

**Path of Goodies** is a reverse tower defense game set in a procedurally generated medieval fantasy kingdom. Players are traders traveling between cities completing delivery contracts while defending their cargo from bandits, wild animals, and rival traders. The game features resource management, roguelike permadeath mechanics, and multiplayer guild systems.

**Core Pillars:**
- Strategic route planning and risk management
- Real-time travel with defensive combat
- Resource management (food, water, gold)
- Persistent progression and permadeath
- Multiplayer guilds and PvP

**Platform Priority:**
1. Web (WASM) - Primary target for MVP
2. Desktop (Windows/Mac/Linux) - For multiplayer servers
3. Mobile - Future expansion

---

## Technology Stack

### Core Engine
- **Bevy 0.15+** - ECS game engine with WASM support
- **Rust 2021** - Performance and cross-platform compatibility

### Key Dependencies
```toml
bevy = "0.15"               # Core engine
bevy_ecs_tilemap = "*"      # Isometric tilemap rendering
rand = "*"                  # Procedural generation
serde = "*"                 # Save/load, networking
noise = "*"                 # Perlin/Simplex noise for terrain
pathfinding = "*"           # AI pathfinding

# Networking (Phase 3+)
bevy_replicon = "*"         # ECS replication for multiplayer
renet = "*"                 # UDP networking
```

### Build Tools
- `trunk` - WASM bundler
- `wasm-bindgen` - Web bindings
- `wasm-opt` - Bundle optimization

---

## Development Phases

### Phase 0: Foundation ✅ (Weeks 1-2)

**Status**: ✅ Complete (Basic setup done)

**Completed:**
- [x] Rust project initialization
- [x] Bevy setup with basic app
- [x] Web build configuration
- [x] Game design documentation
- [x] Project roadmap

**Remaining:**
- [ ] Test WASM build locally
- [ ] Verify controls work on web
- [ ] Create placeholder art assets
- [ ] Set up asset directory structure

---

### Phase 1: MVP Core Systems (Weeks 3-6)

**Goal**: Prove core gameplay loop is fun

#### 1.1 World & Movement (Week 3)
- [ ] **Isometric camera system**
  - Orthographic projection with isometric angle
  - Zoom controls
  - Follow player smoothly

- [ ] **Caravan entity and movement**
  - Replace placeholder square with wagon sprite
  - 8-directional movement (or free movement)
  - Speed affected by weight
  - Animation states (idle, moving)

- [ ] **Simple procedural world**
  - Noise-based terrain generation (one biome: grasslands)
  - Tile-based rendering (grass, dirt roads)
  - Two cities (start and destination)
  - Simple path between them

**Deliverable**: Move a wagon across procedurally generated terrain

#### 1.2 Resource Management (Week 4)
- [ ] **Resource system**
  - Food, water, gold components
  - Consumption over time
  - UI display (HUD showing resources)

- [ ] **Resource effects**
  - Low food/water = health loss
  - Zero gold = bankruptcy (game over)
  - Weight calculation (cargo + supplies)

- [ ] **Supply management**
  - Buy food/water at cities
  - Inventory system (simple list)

**Deliverable**: Player must manage food/water or die

#### 1.3 Basic Contract System (Week 5)
- [ ] **Contract board UI**
  - List of 3-5 delivery contracts
  - Show: destination, cargo, payment, difficulty

- [ ] **Cargo system**
  - Accept contract = load cargo
  - Cargo has weight (affects speed)
  - Deliver at destination = get paid

- [ ] **Win/loss conditions**
  - Success: Deliver cargo, earn gold
  - Failure: Death (zero health) or bankruptcy
  - Return to main menu on game over

**Deliverable**: Complete one delivery contract end-to-end

#### 1.4 Basic Combat/Encounters (Week 6)
- [ ] **Simple enemy encounters**
  - Bandits spawn on path (random chance)
  - Enemies chase wagon
  - Basic auto-combat (no tower defense yet)

- [ ] **Health and damage**
  - Player/wagon has health
  - Enemies deal damage on contact
  - Health bar UI

- [ ] **Loot and rewards**
  - Defeated enemies drop gold
  - Risk vs reward (fighting is profitable but dangerous)

**Deliverable**: Enemies attack, player can fight back or flee

---

### Phase 2: Core Gameplay Loop (Weeks 7-10)

**Goal**: Feature-complete single-player core gameplay

#### 2.1 Reverse Tower Defense (Week 7)
- [ ] **Guard system**
  - Hire guards at cities (cost gold)
  - Guard types: Archer, Swordsman, Spearman
  - Stats: health, damage, range, cost

- [ ] **Guard positioning**
  - Place guards around wagon (grid slots)
  - Front, back, left, right positions
  - Guards auto-attack nearby enemies

- [ ] **Combat mechanics**
  - Enemies spawn ahead and approach
  - Guards shoot/attack automatically
  - Tactical positioning matters
  - Guards can die (permanent loss)

**Deliverable**: Hire and position guards to defend against bandit waves

#### 2.2 Advanced Encounters (Week 8)
- [ ] **Encounter variety**
  - Multiple enemy types (bandits, wolves, bears)
  - Non-combat events (shrines, traders, abandoned houses)
  - Choices (bribe bandits, pray at shrine, search ruins)

- [ ] **Event system**
  - Random events trigger during travel
  - Modal UI for event choices
  - Consequences (gain/lose items, health, gold)

- [ ] **Loot and items**
  - Find artifacts, potions, equipment
  - Item inventory and usage
  - Consumables (healing potions)

**Deliverable**: Varied encounters make each journey unique

#### 2.3 Progression & Upgrades (Week 9)
- [ ] **Persistent progression**
  - Gold persists between deliveries
  - Unlock new wagon upgrades
  - Upgrade shop in cities

- [ ] **Wagon upgrades**
  - Capacity (carry more cargo/supplies)
  - Armor (reduce damage)
  - Speed (move faster)
  - Guard slots (hire more guards)

- [ ] **Player skills**
  - Combat, negotiation, survival, perception
  - Level up through use or training
  - Skill effects (better prices, spot ambushes)

**Deliverable**: Clear progression between runs

#### 2.4 World Expansion (Week 10)
- [ ] **Multiple biomes**
  - Forests (slow, ambushes)
  - Mountains (very slow, bandits)
  - Deserts (high water consumption)

- [ ] **More settlements**
  - 5+ cities and towns
  - Villages and outposts
  - Different prices and contracts at each

- [ ] **Route planning UI**
  - Map view showing settlements and paths
  - Choose route (safe, fast, or risky)
  - Fog of war (unexplored areas hidden)

**Deliverable**: Meaningful route choices with consequences

---

### Phase 3: Roguelike & Polish (Weeks 11-14)

**Goal**: Polished single-player experience with replayability

#### 3.1 Permadeath & Persistence (Week 11)
- [ ] **Death mechanics**
  - Wagon remains at death location
  - Spawn as new trader
  - Option to create new world

- [ ] **Legacy system**
  - Dead wagons marked on map
  - Loot your own corpse (if you find it)
  - Death markers visible to other players (multiplayer prep)

- [ ] **Save/load system**
  - Save game state (world, progress, contracts)
  - RON format for readability
  - Auto-save on major events

**Deliverable**: Permadeath with meaningful persistence

#### 3.2 Companions & AI (Week 12)
- [ ] **Permanent companions**
  - Special NPCs join your caravan
  - Named characters with personalities
  - Level up and gain abilities
  - Can die permanently

- [ ] **Improved AI**
  - Enemy pathfinding (chase wagon)
  - Guard target prioritization
  - Companion special abilities

- [ ] **Morale system**
  - Guards have morale (affects performance)
  - Low morale = desertion risk
  - Improve with rest, food, victory

**Deliverable**: Companions add emotional depth

#### 3.3 UI/UX Polish (Week 13)
- [ ] **Complete UI overhaul**
  - Contract board redesign
  - Inventory management UI
  - Character sheet / stats screen
  - Settings menu

- [ ] **HUD improvements**
  - Resource bars (food, water, health)
  - Minimap
  - Active contract display
  - Warning indicators (ambush ahead!)

- [ ] **Visual feedback**
  - Damage numbers
  - Hit effects / screen shake
  - Particle effects (dust, blood, magic)
  - Death animations

**Deliverable**: Professional, polished interface

#### 3.4 Audio & Juice (Week 14)
- [ ] **Sound effects**
  - Wagon sounds (wheels, creaking)
  - Combat (swords, arrows, impact)
  - UI sounds (clicks, coins)
  - Ambient (wind, birds, water)

- [ ] **Music**
  - Travel music (ambient, medieval)
  - Combat music (tense)
  - City music (bustling)

- [ ] **Game feel**
  - Camera shake on hits
  - Slow motion on kills
  - Satisfying feedback loops

**Deliverable**: Game feels great to play

---

### Phase 4: Multiplayer Foundation (Weeks 15-18)

**Goal**: Implement online multiplayer and guilds

#### 4.1 Networking Architecture (Week 15)
- [ ] **Client-server setup**
  - Authoritative server
  - Client prediction
  - Server reconciliation

- [ ] **Bevy networking integration**
  - `bevy_replicon` for ECS replication
  - `renet` for UDP transport
  - WebSocket support for WASM clients

- [ ] **Basic synchronization**
  - Player position sync
  - World state replication
  - Event synchronization

**Deliverable**: Two players can see each other moving

#### 4.2 Multiplayer Gameplay (Week 16)
- [ ] **Shared world**
  - Persistent server world
  - Multiple caravans visible
  - Other players' positions synced

- [ ] **Player interactions**
  - Trade UI (exchange items)
  - Chat system
  - Shared combat (help defend)

- [ ] **PvP mechanics**
  - Friendly fire option
  - Steal cargo mechanic
  - Reputation and consequences

**Deliverable**: Meaningful multiplayer interactions

#### 4.3 Guild System (Week 17)
- [ ] **Guild creation**
  - Create/join guilds
  - Guild roster and members
  - Guild warehouse (shared storage)

- [ ] **Guild features**
  - Guild contracts (higher rewards)
  - Shared map knowledge
  - Guild chat
  - Guild reputation

- [ ] **Guild economy**
  - Contribute resources
  - Withdraw with permissions
  - Guild-funded caravans

**Deliverable**: Functional guild system

#### 4.4 Server Infrastructure (Week 18)
- [ ] **Dedicated server**
  - Headless server build
  - Server configuration files
  - Admin commands

- [ ] **Persistence**
  - Save world state to disk
  - Player account system (simple)
  - Server restart recovery

- [ ] **Anti-cheat basics**
  - Server validation
  - Rate limiting
  - Basic exploit prevention

**Deliverable**: Stable multiplayer server

---

### Phase 5: Content & Balance (Weeks 19-22)

**Goal**: Rich content and balanced economy

#### 5.1 More Content (Week 19-20)
- [ ] **Expanded world**
  - 10+ cities and settlements
  - 20+ random encounter types
  - 5+ biomes with unique challenges

- [ ] **More contracts**
  - Illegal goods deliveries
  - Rescue missions
  - Time-sensitive contracts
  - Multi-stop deliveries

- [ ] **More enemies**
  - Elite enemies (bosses)
  - Supernatural threats
  - Organized bandit gangs

**Deliverable**: 20+ hours of content

#### 5.2 Economy Balancing (Week 21)
- [ ] **Dynamic pricing**
  - Supply and demand system
  - Prices vary by location
  - Market trends

- [ ] **Difficulty curve**
  - Tutorial for new players
  - Gradual difficulty increase
  - End-game challenges

- [ ] **Progression tuning**
  - Balanced upgrade costs
  - Meaningful choices
  - No grinding required

**Deliverable**: Balanced, fair economy

#### 5.3 Playtesting (Week 22)
- [ ] **Internal testing**
  - Bug hunting
  - Balance adjustments
  - UX improvements

- [ ] **External playtest**
  - Invite testers
  - Gather feedback
  - Iterate on pain points

**Deliverable**: Polished, tested game

---

### Phase 6: Release Preparation (Weeks 23-24)

**Goal**: Public release

#### 6.1 Optimization (Week 23)
- [ ] **Performance tuning**
  - WASM bundle optimization (wasm-opt)
  - Asset compression
  - Code profiling and optimization

- [ ] **Cross-platform testing**
  - Test on multiple browsers
  - Desktop builds (Windows, Mac, Linux)
  - Various screen sizes

- [ ] **Accessibility**
  - Colorblind modes
  - Rebindable keys
  - UI scaling

**Deliverable**: Runs smoothly everywhere

#### 6.2 Launch (Week 24)
- [ ] **Marketing materials**
  - Trailer video
  - Screenshots
  - Press kit

- [ ] **Distribution**
  - itch.io page
  - Steam page (if applicable)
  - Landing website

- [ ] **Community setup**
  - Discord server
  - Reddit community
  - Social media

- [ ] **Release**
  - Public launch
  - Monitor for critical bugs
  - Community management

**Deliverable**: Public release!

---

## Post-Launch Roadmap

### Mobile Port (Months 4-5)
- Touch controls
- UI optimization for small screens
- Performance tuning for mobile hardware
- App store submission

### Advanced Features (Months 6+)
- Seasonal events
- Leaderboards
- Steam Workshop (mod support)
- More biomes and content
- Quality-of-life improvements

---

## Technical Architecture

### Project Structure
```
path-of-goodies/
├── assets/
│   ├── sprites/
│   │   ├── wagons/
│   │   ├── characters/
│   │   ├── enemies/
│   │   └── items/
│   ├── tiles/          # Terrain tiles
│   ├── audio/
│   │   ├── music/
│   │   └── sfx/
│   ├── fonts/
│   └── data/
│       └── contracts.ron   # Contract definitions
├── src/
│   ├── main.rs
│   ├── game/
│   │   ├── mod.rs
│   │   ├── state.rs        # Game states (Menu, Playing, etc.)
│   │   └── resources.rs    # Global game resources
│   ├── world/
│   │   ├── mod.rs
│   │   ├── generation.rs   # Procedural world generation
│   │   ├── biomes.rs
│   │   └── pathfinding.rs
│   ├── entities/
│   │   ├── mod.rs
│   │   ├── player.rs       # Player/wagon components
│   │   ├── guards.rs
│   │   ├── enemies.rs
│   │   └── items.rs
│   ├── systems/
│   │   ├── mod.rs
│   │   ├── movement.rs
│   │   ├── combat.rs
│   │   ├── resources.rs    # Resource management
│   │   ├── contracts.rs
│   │   └── events.rs       # Random encounters
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── hud.rs
│   │   ├── menus.rs
│   │   ├── contracts.rs    # Contract board UI
│   │   └── inventory.rs
│   ├── multiplayer/        # Phase 4+
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   ├── server.rs
│   │   └── guilds.rs
│   └── utils/
│       ├── mod.rs
│       └── save_load.rs
├── server/                 # Dedicated server (Phase 4+)
│   └── main.rs
├── Cargo.toml
├── Trunk.toml
└── index.html
```

### Core Systems Overview

#### ECS Components
- `Wagon` - Player's caravan
- `Guard` - Hired defenders
- `Enemy` - Hostiles
- `Item` - Collectibles, cargo, supplies
- `Contract` - Delivery missions
- `Resource` - Food, water, gold
- `Position`, `Velocity` - Movement
- `Health`, `Damage` - Combat
- `Inventory` - Item storage

#### ECS Resources
- `WorldState` - Generated world data
- `GameTime` - Day/night cycle
- `Economy` - Prices and supply/demand
- `PlayerProgress` - Unlocks, reputation
- `ContractPool` - Available missions

#### Game States
- `MainMenu` - Start screen
- `ContractSelection` - Choose delivery
- `Traveling` - Main gameplay
- `Combat` - Active encounter
- `City` - Shopping, upgrading
- `GameOver` - Death/bankruptcy screen
- `Multiplayer` - Online lobby (Phase 4+)

---

## Risk Mitigation

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Scope too large | High | High | MVP-first approach, cut features if needed |
| Multiplayer complexity | High | Medium | Phase 4 is optional, single-player first |
| Procedural generation issues | Medium | Medium | Start simple, iterate based on playtesting |
| WASM performance | Medium | Low | Early profiling, optimize continuously |
| Art asset bottleneck | Medium | High | Use placeholder art, simple pixel art style |
| Network cheating | High | Medium | Server-side validation, accept some risk for MVP |
| Balance problems | Medium | High | Extensive playtesting, flexible economy system |

---

## Success Metrics

### MVP (Phase 1-2)
- ✅ One complete delivery playable
- ✅ 10-15 minute session length
- ✅ 60 FPS on modern browsers
- ✅ Core loop is fun (playtest feedback)

### Alpha (Phase 3)
- ✅ 3+ hours of content
- ✅ Permadeath works and feels fair
- ✅ Players want to replay
- ✅ Desktop builds available

### Beta (Phase 4-5)
- ✅ Multiplayer functional
- ✅ 10+ hours of content
- ✅ Economy balanced
- ✅ Positive tester feedback (80%+)

### Release (Phase 6)
- 🎯 1,000+ players in first month
- 🎯 75%+ positive reviews
- 🎯 Active community (Discord, Reddit)
- 🎯 Stable revenue stream

---

## Timeline

**Start Date**: 2025-10-21
**MVP Target**: 2025-12-15 (8 weeks) - Phases 1-2
**Alpha Target**: 2026-02-09 (16 weeks) - Phase 3
**Beta Target**: 2026-03-30 (20 weeks) - Phase 4
**Release Target**: 2026-05-11 (24 weeks) - Phase 6

**Estimated Total**: 6 months to release

---

## Current Status

**Phase**: 0 - Foundation
**Progress**: 80% complete
**Next Milestone**: Test WASM build, create placeholder assets
**Current Sprint**: Initial setup and documentation

**Recent Updates**:
- ✅ Project structure created
- ✅ Bevy setup complete
- ✅ Game design documented
- ✅ Roadmap defined
- ⏳ WASM build testing in progress

---

## Resources

### Bevy
- [Official Bevy Book](https://bevyengine.org/learn/book/introduction/)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)

### Networking
- [bevy_replicon Documentation](https://docs.rs/bevy_replicon/)
- [renet GitHub](https://github.com/lucaspoffo/renet)

### Procedural Generation
- [Red Blob Games](https://www.redblobgames.com/) - Algorithms
- [Procedural Generation Wiki](http://pcg.wikidot.com/)

### Game Design
- [FTL Design Postmortem](https://www.youtube.com/watch?v=P4Um97AUqp4)
- [Roguelike Celebration Talks](https://www.youtube.com/c/roguelikecelebration)

---

*This roadmap is a living document. Update weekly as progress is made.*

**Last Updated**: 2025-10-21
**Next Review**: 2025-10-28
