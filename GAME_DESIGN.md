# Path of Goodies - Game Design Document

**Version**: 0.1
**Last Updated**: 2025-10-21
**Status**: Pre-Alpha Design

---

## 1. Executive Summary

**Path of Goodies** is a reverse tower defense game set in a fantasy medieval kingdom where players take on the role of traders/delivery contractors navigating dangerous routes between settlements. Unlike traditional tower defense where you defend a static position, players are constantly moving through procedurally generated landscapes, defending their cargo from ambushes, bandits, and wild creatures while managing resources and making strategic decisions about routes, equipment, and companions.

### High Concept
> "Oregon Trail meets reverse tower defense with roguelike progression in a shared medieval fantasy world"

### Core Pillars
1. **Strategic Route Planning** - Risk vs reward path choices
2. **Real-time Defense** - Protect cargo while traveling
3. **Resource Management** - Food, water, gold, and goods
4. **Progression & Persistence** - Upgrade caravan, hire guards, unlock new opportunities
5. **Social Trading World** - Multiplayer guilds, cooperation, and competition

---

## 2. Game Overview

### 2.1 Setting
A procedurally generated fantasy medieval kingdom featuring:
- **Major Cities** - Trading hubs with quest boards and merchants
- **Towns & Villages** - Rest stops, supply points
- **Castles & Fortresses** - Secure but may require tolls
- **Outposts** - Guard stations, safe havens
- **Wilderness** - Forests, mountains, plains (dangerous, unpredictable)
- **Points of Interest** - Shrines, abandoned houses, artifact sites, bandit camps

### 2.2 Core Gameplay Loop

```
Accept Delivery Contract
    ↓
Plan Route (safe vs fast vs profitable)
    ↓
Equip & Prepare (buy supplies, hire guards)
    ↓
Travel in Real-time
    ├─→ Encounter Events (shrines, artifacts, ambushes)
    ├─→ Defend Cargo (combat)
    ├─→ Manage Resources (food, water)
    └─→ Make Decisions (rest, detour, push forward)
    ↓
Arrive at Destination
    ↓
Complete Delivery → Earn Gold & Reputation
    ↓
Upgrade & Expand (better wagon, guards, equipment)
    ↓
[Repeat with harder contracts]
```

### 2.3 Victory Conditions
- **Session Goal**: Complete delivery contract without dying or going bankrupt
- **Long-term Goal**: Build a successful trading empire (upgraded caravan, guild membership, reputation)
- **Meta Goal**: Master the kingdom's geography and optimize trade routes

---

## 3. Core Mechanics

### 3.1 Delivery System

#### Contract Types
1. **Standard Delivery** - Basic goods, moderate pay, low risk
2. **Valuable Cargo** - High pay, attracts bandits and thieves
3. **Perishable Goods** - Time-sensitive, bonuses for fast delivery
4. **Illegal Goods** - Highest pay, extreme risk (guards attack if caught)
5. **Rescue Missions** - Escort NPCs, reputation rewards
6. **Guild Contracts** - Exclusive high-tier missions

#### Contract Attributes
- **Origin & Destination** - Distance determines base pay and time
- **Cargo Type & Volume** - Affects wagon speed and risk profile
- **Time Limit** - Optional deadlines for bonus pay
- **Difficulty Rating** - Estimated danger level based on route
- **Base Payment** - Gold reward on completion
- **Reputation Impact** - Success/failure affects future opportunities

### 3.2 Travel & Movement

#### Real-time Travel
- Game world uses **real-time movement** - no turn-based abstraction
- Player controls caravan directly (isometric view)
- Movement speed affected by:
  - Wagon weight (cargo + supplies)
  - Terrain type (roads fast, forests slow)
  - Weather conditions (rain, snow)
  - Wagon upgrades

#### Route Planning
- **Map View** - Shows known settlements, paths, and discovered POIs
- **Path Types**:
  - **King's Road** - Safe, guarded, but slow and tolls required
  - **Trade Routes** - Moderate traffic, moderate risk
  - **Wilderness Paths** - Fast but dangerous
  - **Secret Routes** - Discovered through exploration, high risk/reward

#### Fog of War
- Unexplored areas hidden until discovered
- Previous routes remain visible
- Shared knowledge in multiplayer (guild members share maps)

### 3.3 Resource Management

#### Core Resources
1. **Gold** - Currency for everything
2. **Food** - Consumed over time; starvation causes health loss
3. **Water** - Consumed faster in hot biomes
4. **Cargo** - The goods being delivered (must protect)
5. **Wagon Durability** - Damaged by combat and rough terrain

#### Resource Mechanics
- **Consumption Rates** scale with party size
- **Spoilage** - Food degrades over time
- **Resupply** - Buy at settlements or find/scavenge
- **Weight Management** - Tradeoff between supplies and cargo capacity

### 3.4 Combat & Defense

#### Threat Types
1. **Bandits** - Target valuable cargo, can be bribed or fought
2. **Wild Animals** - Wolves, bears - attack when hungry
3. **Rival Traders (PvP)** - Can steal cargo in multiplayer
4. **Deserters/Outlaws** - Former soldiers, organized attacks
5. **Supernatural** - Rare encounters (undead, demons at shrines)

#### Defense Mechanics (Reverse Tower Defense)
- **Guard Positioning** - Place hired guards around wagon
- **Auto-combat** - Guards fight automatically based on AI
- **Player Actions**:
  - Command guards (hold position, aggressive, defensive)
  - Use items (healing potions, smoke bombs)
  - Flee vs fight decisions
  - Bribe/negotiate with intelligent enemies

#### Combat Resolution
- **Real-time** - Enemies approach from fog of war
- **Guard Stats** - Health, damage, range, special abilities
- **Positioning Matters** - Flanking, wagon protection
- **Casualties** - Guards can be wounded or killed permanently
- **Cargo Damage** - If defenses fail, cargo is stolen/destroyed

### 3.5 Progression Systems

#### Player Character
- Start as **Solo Trader** - Just you and a small cart
- **Skills/Attributes**:
  - Combat (personal defense capability)
  - Negotiation (better contracts, lower prices)
  - Survival (reduced food/water consumption)
  - Perception (spot ambushes, find hidden routes)

#### Wagon Upgrades
- **Capacity** - Carry more cargo/supplies
- **Armor** - Reduce damage from attacks
- **Speed** - Move faster on roads
- **Storage** - Hidden compartments for illegal goods
- **Comfort** - Reduce guard morale loss

#### Companions & Guards

**Hired Guards** (temporary, paid per mission)
- Archer, Swordsman, Spearman
- Varying costs and effectiveness
- Morale system - low morale = desertion risk

**Permanent Companions** (recruited through events)
- Named NPCs with personalities and backstories
- Level up and gain abilities
- Can die permanently (emotional investment)
- Special companion quests

#### Reputation System
- **Merchant Guilds** - Access to better contracts
- **Local Lords** - Permissions, safe passage
- **Outlaw Factions** - Black market access, or targets you
- **Consequences** - Actions have lasting impacts

---

## 4. World & Encounters

### 4.1 Procedural Generation

#### World Generation
- **Persistent World** per server/save file
- Cities, major roads, and castles are **fixed** positions
- Everything between is **procedurally generated**:
  - Terrain features (forests, rivers, hills)
  - Minor settlements (villages, outposts)
  - POI placement (shrines, ruins, camps)
  - Random events and encounters

#### Biomes
- **Grasslands** - Easy travel, moderate encounters
- **Forests** - Slow travel, ambush risk, hunting opportunities
- **Mountains** - Very slow, bandits in passes, scenic shrines
- **Swamps** - Difficult terrain, disease risk, rare herbs
- **Deserts** - High water consumption, extreme temperatures

### 4.2 Random Encounters

#### Event Types
1. **Combat Encounters** - Bandits, animals, etc.
2. **Trade Encounters** - Meet other traders, exchange goods
3. **Shrines & Temples** - Pray for bonuses, risk curses
4. **Abandoned Structures** - Loot or traps
5. **Strangers in Need** - Help for rewards or recruit
6. **Natural Hazards** - Storms, floods, avalanches
7. **Artifacts** - Rare finds with special properties
8. **Ambush Sites** - Corpses of failed caravans (could be yours!)

#### Dynamic Events
- **Frequency** depends on path danger rating
- **Types** influenced by cargo (valuable = more bandits)
- **Time of Day** affects encounter types (bandits at night)
- **Weather** can trigger specific events

### 4.3 Death & Persistence (Roguelike Elements)

#### When You Die
1. **Caravan Remains** - Your wagon stays where you died
2. **Cargo Lost** - Goods scattered at death site
3. **Options**:
   - **Continue as New Trader** - Start fresh, but world persists
   - **New World** - Complete reset

#### Legacy System
- **Death Markers** - Dead caravans visible on map
- **Recovery Missions** - Find your old wagon and loot it
- **Risk** - Other players/NPCs can loot your corpse first
- **Difficulty** - Must reach death site without dying again

#### Bankruptcy
- Alternative to death - can't afford food or contracts
- Forced to take risky jobs to recover
- Can sell wagon parts/items to survive

---

## 5. Multiplayer Systems

### 5.1 Game Modes

#### Offline Solo
- Procedurally generated personal world
- Full single-player campaign
- No online requirements
- Save/load anytime

#### LAN Multiplayer
- Local network co-op or competitive
- Host creates world
- 2-8 players

#### Online Server
- Persistent shared world hosted on server
- 20-100+ players (scalable)
- Asynchronous gameplay (players don't need to be online simultaneously)
- Server tick rate updates world state

### 5.2 Multiplayer Interactions

#### Guilds
- **Player-created Trading Guilds**
- Shared resources/warehouse
- Guild contracts with bigger rewards
- Shared map knowledge
- Guild reputation

#### Player Encounters
- **Friendly Interactions**:
  - Trade goods/supplies
  - Share map information
  - Travel together for protection
  - Help in combat

- **Hostile Interactions**:
  - Steal cargo (PvP)
  - Ambush other traders
  - Compete for contracts

#### Safe Zones
- **Cities & Major Settlements** - PvP disabled
- **Guild Halls** - Safe meeting spaces
- **Wilderness** - PvP enabled (with consequences)

#### Reputation & Consequences
- **Bandit Players** - Attacking others lowers reputation
- **Law System** - Bounties placed on griefers
- **Guards Hunt Criminals** - NPC guards attack low-rep players
- **Balanced Risk/Reward** - Being an outlaw is viable but harder

### 5.3 Server Architecture

#### World Persistence
- Server maintains single persistent world state
- Player caravans save position when logging off
- Events continue to spawn/despawn
- Economy is shared (supply/demand affects prices)

#### Session Structure
- Players log in/out freely
- Current delivery saved on logout
- Can resume mid-journey
- Death markers persist across sessions

---

## 6. Art & Visual Design

### 6.1 Art Style (To Be Decided)

#### Option A: Pixel Art
**Pros:**
- Retro charm, nostalgic appeal
- Easier to animate and iterate
- Lower asset production cost
- Clear readability
- Strong indie aesthetic

**Cons:**
- Saturated market
- May feel generic without strong style

#### Option B: Hand-Drawn
**Pros:**
- Unique, memorable art style
- Emotional connection
- Stands out in market
- Artistic flexibility

**Cons:**
- Higher production cost
- Animation complexity
- Requires consistent artist

**Recommendation:** Start with **pixel art** for prototyping and MVP, consider hand-drawn for full release if budget allows.

### 6.2 Camera Perspective

#### Isometric View (Recommended)
- **Classic isometric** (2:1 pixel ratio)
- Clear depth perception
- Perfect for tactical positioning
- Medieval fantasy tradition (Age of Empires, Diablo)
- Proven for this genre

#### Alternative: 3/4 Top-Down
- Slight angle (like Stardew Valley)
- Simpler to implement
- Good compromise

**Decision:** Start with **isometric**, can adjust if needed.

### 6.3 Visual Elements

#### UI Requirements
- **Map Screen** - World map with routes and markers
- **Inventory** - Cargo, supplies, equipment
- **Contract Board** - Available deliveries
- **Character Sheet** - Stats, skills, reputation
- **Guard Management** - Hire, position, upgrade
- **Guild Panel** (multiplayer)

#### World Rendering
- **Tile-based Environment** - Procedurally generated terrain
- **Sprite-based Entities** - Characters, wagons, enemies
- **Particle Effects** - Combat, weather, magic
- **Day/Night Cycle** - Lighting changes, affects gameplay
- **Weather Effects** - Rain, snow, fog

---

## 7. Audio Design

### 7.1 Music
- **Ambient Travel Music** - Celtic/medieval themes
- **City Music** - Bustling, lively
- **Combat Music** - Tense, rhythmic
- **Shrine/Temple Music** - Ethereal, mysterious

### 7.2 Sound Effects
- **Travel Sounds** - Wagon creaking, horse hooves, footsteps
- **Combat** - Sword clashes, arrows, impact sounds
- **Ambient** - Birds, wind, water, wildlife
- **UI** - Menu clicks, coin jingles, notifications

---

## 8. Monetization & Distribution

### 8.1 Business Model

#### Premium (Recommended)
- **One-time Purchase** - $15-20 USD
- All content included
- Free updates
- No microtransactions
- Optional cosmetic DLC later

#### Free-to-Play Alternative
- Free base game
- Cosmetic-only shop (wagon skins, guard outfits)
- Battle pass for seasonal content
- No pay-to-win

**Recommendation:** Start with **premium** to build trust and avoid P2W concerns.

### 8.2 Platform Strategy

#### Phase 1: Web (itch.io)
- Free demo or early access
- Build community
- Gather feedback
- Prove concept

#### Phase 2: Desktop (Steam)
- Full release with multiplayer
- Steam Workshop support (custom maps/mods)
- Achievements and trading cards

#### Phase 3: Mobile
- Touchscreen-optimized UI
- Potential for larger audience
- Consider premium + cosmetics hybrid

---

## 9. Technical Considerations

### 9.1 Networking (Multiplayer)

#### Architecture Options

**Option A: Client-Server (Recommended)**
- Authoritative server prevents cheating
- Server handles world state, AI, events
- Clients send inputs, receive updates
- Required for online multiplayer

**Option B: Peer-to-Peer**
- Only viable for LAN
- No central server needed
- Harder to prevent cheating

**Implementation:**
- Use **bevy_replicon** or **renet** for Bevy networking
- WebSocket support for WASM compatibility
- UDP for fast game state updates

#### Challenges
- **WASM Networking** - Limited to WebSocket/WebRTC
- **Latency** - Handle prediction and lag compensation
- **Cheating** - Server-side validation
- **Scalability** - Design for 100+ concurrent players

### 9.2 Procedural Generation

#### World Generation Algorithm
- **Noise-based Terrain** - Perlin/Simplex for natural landscapes
- **Graph-based Roads** - Connect settlements with paths
- **Spatial Hashing** - Efficient POI placement
- **Seeded Generation** - Reproducible worlds

#### Runtime Generation
- **Chunk-based Streaming** - Generate as player explores
- **Caching** - Save generated chunks to disk
- **Deterministic** - Same seed = same world

### 9.3 Save System

#### Save Data
- Player stats, inventory, reputation
- World state (if single-player)
- Discovered map areas
- Active contracts and progress
- Guild membership and data

#### Format
- **RON** (Rusty Object Notation) or **JSON**
- Compressed for disk space
- Versioned for backwards compatibility

---

## 10. Development Priorities

### 10.1 MVP (Minimum Viable Product)

**Core Features for First Playable:**
1. ✅ Basic movement and controls
2. Simple procedural map (one biome)
3. Single delivery contract system
4. Basic resource management (food, water)
5. Simple enemy encounters (bandits)
6. Win/loss conditions
7. Rudimentary upgrade shop

**Goal:** Prove core gameplay loop is fun

### 10.2 Alpha Build

**Features:**
1. Full world generation (multiple biomes)
2. Complete contract variety
3. Guard hiring and positioning
4. Companion system
5. Death and persistence mechanics
6. Multiple settlements and paths

**Goal:** Feature-complete single-player

### 10.3 Beta Build

**Features:**
1. Multiplayer implementation (LAN + Online)
2. Guild systems
3. PvP mechanics
4. Full UI/UX polish
5. Audio integration
6. Balance and tuning

**Goal:** Ready for early access release

---

## 11. Questions & Decisions Needed

### 11.1 Immediate Decisions
- [ ] **Art Style**: Pixel art vs hand-drawn? (Recommend: start pixel art)
- [ ] **Initial Scope**: Solo-only MVP or include basic multiplayer early?
- [ ] **Monetization**: Premium or F2P?

### 11.2 Design Refinements
- [ ] **Permadeath Severity**: How punishing should death be?
- [ ] **PvP Balance**: How to prevent griefing while allowing meaningful interaction?
- [ ] **Progression Curve**: How fast should players advance?
- [ ] **Contract Complexity**: Simple "A to B" or add multi-stop deliveries?

### 11.3 Technical Investigations
- [ ] **Networking Library**: bevy_replicon vs renet vs custom?
- [ ] **Procedural Generation**: Which algorithm for world gen?
- [ ] **Server Hosting**: Self-hosted or dedicated servers?

---

## 12. Inspiration & References

### Similar Games
- **FTL: Faster Than Light** - Route planning, encounters, permadeath
- **Oregon Trail** - Resource management, travel survival
- **Convoy** (PC) - Vehicle-based combat, convoy defense
- **Death Road to Canada** - Group management, procedural events
- **Darkest Dungeon** - Stress, permadeath, resource management
- **They Are Billions** - Reverse tower defense inspiration

### Art References
- **Stardew Valley** - Pixel art quality bar
- **Hades** - Hand-drawn character art
- **Battle Brothers** - Medieval mercenary aesthetic
- **Slay the Spire** - Clean UI for complex systems

---

## 13. Success Metrics

### Gameplay Metrics
- **Session Length**: Target 10-40 minutes per delivery
- **Completion Rate**: 50-70% of contracts successful (difficulty sweet spot)
- **Retention**: Players return for 5+ sessions
- **Progression Feel**: Tangible improvement every 2-3 deliveries

### Business Metrics
- **Wishlists**: 5,000+ before launch (Steam)
- **Conversion**: 10%+ wishlist to purchase
- **Reviews**: 80%+ positive (Steam)
- **Community**: Active Discord with 500+ members

---

## Appendix A: Terminology

- **Caravan** - Player's wagon + guards + cargo
- **POI** - Point of Interest (shrine, ruin, etc.)
- **Contract** - Delivery mission/quest
- **Session** - One complete delivery journey
- **Run** - One life (until death or bankruptcy)
- **Legacy** - Your previous caravan's remains
- **Guild** - Player-formed trading organization
- **Reputation** - Standing with various factions

---

**Next Steps:**
1. Finalize art style decision
2. Create technical architecture doc
3. Build prototype of core travel and encounter system
4. Test core gameplay loop
5. Iterate based on feedback

*This is a living document - update as design evolves.*
