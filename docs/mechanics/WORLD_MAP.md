# World Map System

## Overview

The world map is a graph-based strategic layer where players plan routes, discover new locations, and make high-level decisions about their journey. It's the "board game" view of the world.

## Core Design

### Graph Structure

**Villages = Nodes**
**Paths = Edges**

```
         [Village A]
           /    \
          /      \
    [Path 1]   [Path 2]
      /            \
     /              \
[Village B]      [Village C]
```

### Why Graph-Based?

✅ Clear strategic choices (which path to take?)
✅ Natural fog of war (unexplored areas)
✅ Easy to generate and balance
✅ Supports distance-based mechanics
✅ Feels like planning a journey
✅ Naturally creates "routes" and "shortcuts"

❌ Not Open World: More strategic, less exploration
❌ Not Grid-Based: Not every pixel explorable
❌ Not Linear: Multiple paths exist

---

## World Generation

### Village Generation

#### Village Count
```
Total villages: 15-20
Starting village: 1 (always same)
Capital city: 1 (far from start)
Large towns: 3-4
Villages: 6-8
Hamlets: 5-7
```

#### Placement Algorithm (Poisson Disk Sampling)
```
1. Place starting village at (0, 0)
2. Define world bounds: 2000x2000 units
3. Minimum distance between villages: 150 units
4. Maximum attempts per village: 30
5. Use rejection sampling to ensure spacing
6. Prefer edges of map for variety
```

#### Village Sizes
```rust
enum VillageSize {
    Hamlet,      // Population: 50-200
    Village,     // Population: 200-800
    Town,        // Population: 800-2000
    City,        // Population: 2000-5000
}
```

**Size Distribution**:
- 1 City (capital, late game)
- 3-4 Towns (mid game hubs)
- 6-8 Villages (common stops)
- 5-7 Hamlets (early game, fewer services)

**Size Affects**:
- Available services (blacksmith, temple, etc.)
- Number of contracts
- Guard quality/availability
- Resource prices
- Number of connected paths

---

### Path Generation

#### Connection Strategy
```
1. Connect starting village to 2-3 nearby villages
2. Ensure all villages are reachable (DFS/BFS check)
3. Add extra connections for interesting routing
4. Prefer connecting similar-sized villages
5. Avoid crossing paths (visual clarity)
```

#### Path Properties
```rust
struct Path {
    id: usize,
    village_a: usize,
    village_b: usize,
    distance: f32,          // 100-400 units
    waypoints: Vec<Vec2>,   // For curved paths
    danger_level: f32,      // 0.0-1.0
    terrain: TerrainType,   // Forest, Plain, Mountain
}
```

#### Distance Calculation
```
Short path: 100-150 units
Medium path: 150-250 units
Long path: 250-400 units

Distance affects:
- Travel time (1 unit = 0.5 seconds)
- Resource consumption
- Encounter probability
- Contract rewards
```

#### Waypoints & Curves
```
Every path has 3-5 waypoints
- Start: Village A position
- End: Village B position
- Middle: 1-3 intermediate points

Curves are calculated using:
- Bezier curves for smooth paths
- Slight randomization for organic feel
- Avoidance of other villages/paths
```

---

## Fog of War System

### Three Visibility States

#### 1. Hidden (Unexplored)
```
- Not visible on map
- Completely black/empty
- Player doesn't know it exists
```

#### 2. Foggy (Discovered but not visited)
```
- Village visible as gray silhouette
- Name visible
- Paths visible as faint lines
- Cannot see services/contracts
```

#### 3. Clear (Visited)
```
- Fully visible and colored
- All details accessible
- Can view contracts from map
- Can fast-select as destination
```

### Discovery Rules

**Village Discovery**:
```
When you visit a village:
1. That village becomes Clear
2. All connected paths become Foggy
3. Villages at end of those paths become Foggy
4. Draw lines to show connections
```

**Example Flow**:
```
Start:
[START] → ??? → ??? → ???

After visiting neighbor:
[START]—[Town A]—???
              |
             ???

After visiting another:
[START]—[Town A]—[Town B]
              |         |
        [Village C]    ???
```

### Fog of War Visuals

```
Hidden:      ████████  (completely black)
Foggy:       ░░░Village░░░  (gray silhouette)
Clear:       [Village Name]  (full color sprite)

Path States:
Hidden:      Not drawn
Foggy:       ---- ---- ----  (dashed gray line)
Clear:       ―――――――――――――――  (solid brown/yellow)
```

---

## Navigation & Travel

### Selecting Destination

#### From World Map View
```
1. Press M to open world map
2. Click on any Clear or Foggy village
3. If Clear: See contracts, services, info
4. If Foggy: Only see name and distance
5. Click "Travel Here" to confirm
6. Path is highlighted
7. Press M again to start traveling
```

#### Travel Validation
```
Before starting travel:
✓ Destination is connected to current village
✓ Path is discovered (at least Foggy)
✓ Player confirms travel decision
✗ Cannot travel to Hidden villages
✗ Cannot travel to disconnected villages
```

### Travel Progress

#### During Travel (Gameplay View)
```
- Switch from map view to gameplay view
- Wagon moves along path
- Background scrolls (parallax effect)
- Encounters can interrupt travel
- Resources consumed gradually
- Can press M to check map progress
```

#### Travel Stats Display
```
┌──────────────────────────┐
│ 🎯 Destination: Millhaven │
│ 📏 Distance: 150 / 250    │
│ ⏱️  Time: 02:15 / 04:00   │
│ ⚡ Horse: [■■■■■■░░░░] 60%│
└──────────────────────────┘
```

---

## Map UI & Controls

### Map View Layout

```
┌─────────────────────────────────────────┐
│  Path of Goodies - World Map            │
├─────────────────────────────────────────┤
│                                         │
│    [Village]───────[Village]           │
│       │               │                │
│       │               │                │
│   [YOU ARE]       [Village]           │
│    HERE!             │                │
│       │               │                │
│    [Village]───────[Town]             │
│                                         │
├─────────────────────────────────────────┤
│ Selected: Millhaven (Town, 150 units)  │
│ Services: Inn, Market, Blacksmith      │
│ [View Contracts] [Travel Here] [Close] │
└─────────────────────────────────────────┘
```

### Controls

**Map Navigation**:
```
M Key: Toggle map view on/off
Mouse: Drag to pan camera
Scroll: Zoom in/out
Click: Select village
ESC: Close map
```

**Village Interaction**:
```
Click village → Show info panel
Double-click → Confirm travel
Right-click → Show quick info
```

**Map Legend**:
```
⭐ = Current location
🏰 = City
🏛️ = Town
🏠 = Village
🏡 = Hamlet
═══ = Traveled path (clear)
--- = Known path (foggy)
```

---

## Strategic Elements

### Route Planning

#### Shortest Route
```
Pros:
+ Fastest completion
+ Least resource consumption
+ Fewer encounters

Cons:
- Might miss opportunities
- Less exploration
- Predictable
```

#### Scenic Route
```
Pros:
+ Discover new villages
+ Find better contracts
+ More encounters (risk/reward)

Cons:
- Longer travel time
- More resources needed
- Higher danger
```

#### Hub Strategy
```
Use central town as base:
1. Take contracts from hub
2. Deliver to nearby villages
3. Return to hub to resupply
4. Gradually expand territory

Pros:
+ Predictable and safe
+ Good for beginners
+ Consistent profits

Cons:
- Repetitive
- Lower rewards
- Slower exploration
```

### Risk vs Reward

#### Safe Paths (Short, well-traveled)
```
Encounter chance: 20-30%
Distance: 100-150 units
Rewards: Lower (30-60g contracts)
Good for: Learning, low resources
```

#### Risky Paths (Long, dangerous)
```
Encounter chance: 50-70%
Distance: 250-400 units
Rewards: Higher (120-250g contracts)
Good for: Experienced, equipped party
```

---

## Map Generation Example

### Procedural Generation Steps

```rust
// Pseudocode for world generation

fn generate_world_map() -> WorldMap {
    // Step 1: Create villages
    let mut villages = Vec::new();
    villages.push(Village::new_starting());

    for _ in 0..15 {
        let pos = poisson_disk_sample();
        let size = random_village_size();
        villages.push(Village::new(pos, size));
    }

    // Step 2: Create paths using minimum spanning tree
    let mut paths = Vec::new();
    let mst = kruskal_mst(&villages);

    for edge in mst {
        let path = Path::new(
            edge.from,
            edge.to,
            distance(edge.from, edge.to),
            generate_waypoints(edge.from, edge.to)
        );
        paths.push(path);
    }

    // Step 3: Add extra connections for loops
    add_shortcut_paths(&mut paths, &villages, 0.2);

    // Step 4: Set fog of war
    let mut discovered = HashSet::new();
    discovered.insert(0); // Starting village

    WorldMap {
        villages,
        paths,
        discovered_villages: discovered,
        current_village: 0,
    }
}
```

### Sample Generated Map

```
Legend:
S = Start
C = City (capital)
T = Town
V = Village
H = Hamlet

    H───V───T───V
    │       │   │
    S───V───T───C
    │       │
    H───V───V───H
```

**Analysis**:
- 11 locations total
- Multiple routes between any two points
- Starting area has 3 connections
- Capital is far but reachable
- Hamlets on periphery
- Towns form "hubs"

---

## Technical Implementation Notes

### Data Structures

```rust
// Core types
struct WorldMap {
    villages: HashMap<usize, Village>,
    paths: HashMap<usize, Path>,
    adjacency: HashMap<usize, Vec<usize>>, // village_id → connected path_ids
    starting_village: usize,
    current_village: Option<usize>,
    discovered_villages: HashSet<usize>,
    discovered_paths: HashSet<usize>,
}

struct Village {
    id: usize,
    name: String,
    position: Vec2,
    size: VillageSize,
    services: Services,
}

struct Path {
    id: usize,
    village_a: usize,
    village_b: usize,
    distance: f32,
    waypoints: Vec<Vec2>,
    danger_level: f32,
}
```

### Algorithms Needed

1. **Pathfinding** - Dijkstra's or A* for route planning
2. **Poisson Disk Sampling** - Village placement
3. **Minimum Spanning Tree** - Initial path connections
4. **Bezier Curves** - Smooth path rendering
5. **Breadth-First Search** - Fog of war discovery

### Performance Considerations

```
Villages: 15-20 → O(n) operations fine
Paths: 20-30 → O(n) operations fine
Pathfinding: Rarely called → Can use simple algorithms
Rendering: Only visible nodes → Culling important

Expected performance: 60 FPS with simple implementation
```

---

## Progression & Unlocks

### Map Expansion

**Early Game (Trips 1-3)**:
- Only starting area visible
- 2-3 villages discovered
- Short, safe paths

**Mid Game (Trips 4-10)**:
- 8-10 villages discovered
- Found first Town
- Some long paths revealed

**Late Game (Trips 11+)**:
- Most of map explored
- Found Capital city
- Access to all regions

### Discovery Rewards

```
First time discovering:
- Hamlet: +10 reputation
- Village: +20 reputation
- Town: +50 reputation
- City: +100 reputation

Discovering all villages:
- Achievement unlocked
- Special contract available
- Bonus starting gold on next run
```

---

## Visual Design Notes

### Map Aesthetics

**Style**: Hand-drawn parchment map
**Colors**:
- Background: Aged paper (beige/tan)
- Paths: Brown ink
- Villages: Black ink with icon
- Current location: Gold star
- Fog: Gray overlay

**Animations**:
- Fog fades away when discovering
- Paths draw out like ink
- Villages "appear" with pop effect
- Current location pulses gently

### Icon Design

```
Hamlet:   🏡 Simple house
Village:  🏠 Two houses
Town:     🏛️ Building with tower
City:     🏰 Castle/walls

Path line: ══════ (double line)
Your location: ⭐ (star)
Active destination: 🎯 (target)
```

---

## Testing & Balance

### Playtest Metrics

1. **Discovery Rate**
   - Target: Discover 1-2 new locations per trip
   - Should reveal full map in 10-15 trips

2. **Route Variety**
   - Players should have 2-3 viable routes to most destinations
   - Should occasionally require detours

3. **Navigation Clarity**
   - Players shouldn't get lost or confused
   - Clear visual feedback on current location
   - Easy to tell where paths lead

### Common Issues

**Too Linear**:
- Add more cross-connections
- Create loop paths
- Add shortcuts between distant nodes

**Too Confusing**:
- Reduce path crossings
- Space villages further
- Stronger visual hierarchy

**Too Small**:
- Increase world bounds
- Add more villages
- Increase minimum distances

---

## Future Features

### Potential Additions

1. **Fast Travel**: Return to previously visited towns
2. **Landmarks**: Points of interest along paths
3. **Weather**: Affects visibility and path difficulty
4. **Seasons**: Change map appearance and conditions
5. **Territory Control**: Factions own regions
6. **Dynamic Events**: Paths close due to bandits/weather

---

**Version**: 1.0
**Last Updated**: 2025-10-22
**Status**: DRAFT - Ready for implementation
