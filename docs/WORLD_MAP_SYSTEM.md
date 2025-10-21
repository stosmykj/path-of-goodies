# World Map System

## Overview

The world map is a graph-based strategic layer that represents the game world as a network of villages (nodes) connected by paths (edges). This system features procedural generation and fog of war exploration mechanics.

## Architecture

### Core Components

#### Villages (Graph Nodes)
- **Position**: Vec2 coordinates in world space
- **Size**: Hamlet < Village < Town < City
- **Discovery State**: Whether the village has been explored
- **Connections**: Links to other villages via paths

#### Paths (Graph Edges)
- **Waypoints**: Series of Vec2 points defining the path
- **Village Connections**: Which two villages this path connects
- **Distance**: Total travel distance along the path
- **Discovery State**: Whether the path is visible to the player
- **Rendering**: Paths are rendered as polygonal strips

### Data Structures

```rust
pub struct Village {
    pub id: usize,
    pub name: String,
    pub position: Vec2,
    pub size: VillageSize,
    pub is_discovered: bool,
}

pub struct Path {
    pub id: usize,
    pub village_a: usize,
    pub village_b: usize,
    pub waypoints: Vec<Vec2>, // Can have multiple points for curved paths
    pub distance: f32,
    pub is_discovered: bool,
}

pub struct WorldMap {
    pub villages: HashMap<usize, Village>,
    pub paths: HashMap<usize, Path>,
    pub adjacency: HashMap<usize, Vec<usize>>, // village_id -> path_ids
    pub starting_village: usize,
    pub discovered_villages: HashSet<usize>,
    pub discovered_paths: HashSet<usize>,
}
```

## Procedural Generation

### Generation Algorithm

1. **Village Placement** (Poisson Disk Sampling)
   - Generate positions with minimum distance constraint
   - First village spawns near world center
   - Total villages: 15 (configurable)
   - World size: 2000x2000 units
   - Minimum distance: 200 units

2. **Village Size Assignment**
   - First village: Always "Village" size (starting point)
   - Early villages: Higher chance of being larger settlements
   - Later villages: More likely to be hamlets
   - Distribution: ~5% Cities, ~20% Towns, ~45% Villages, ~30% Hamlets

3. **Path Generation**
   - Sort all village pairs by distance
   - Connect nearest neighbors first
   - Maximum connections per village: 4
   - Avoid path crossings when possible
   - 30% chance for curved paths (with waypoint)

4. **Connectivity Assurance**
   - Ensure all villages are reachable
   - Add missing connections to isolated villages
   - Creates a connected graph

5. **Starting Village Selection**
   - Prefer medium-sized villages (Village or Town)
   - Prefer locations near world center
   - Scoring: size_score - (distance_from_center / 500)

### Generation Configuration

```rust
pub struct WorldGenConfig {
    pub num_villages: usize,           // Default: 15
    pub world_size: f32,               // Default: 2000.0
    pub min_distance: f32,             // Default: 200.0
    pub max_connections_per_village: usize, // Default: 4
    pub path_curve_chance: f32,        // Default: 0.3
    pub path_curve_strength: f32,      // Default: 100.0
    pub seed: u64,                     // For reproducible generation
}
```

## Fog of War System

### Visibility Rules

1. **Starting Village**
   - Automatically discovered on game start
   - Fully visible with name displayed

2. **Connected Paths**
   - All paths from discovered villages are revealed
   - Paths shown in brown color
   - Multiple waypoints create curved roads

3. **Undiscovered Villages**
   - **Connected to Discovered**: Shown as dimmed circles (fog of war)
   - **Not Connected**: Completely hidden
   - Names only shown when discovered

### Discovery Mechanics

When a village is discovered:
1. Village marked as `is_discovered = true`
2. Added to `discovered_villages` set
3. All connected paths are revealed
4. All connected paths marked as `is_discovered = true`
5. Neighboring villages become visible (but foggy)

```rust
pub fn discover_village(&mut self, village_id: usize) {
    self.discovered_villages.insert(village_id);

    // Reveal all connected paths
    if let Some(path_ids) = self.adjacency.get(&village_id) {
        for path_id in path_ids {
            self.discovered_paths.insert(*path_id);
            if let Some(path) = self.paths.get_mut(path_id) {
                path.is_discovered = true;
            }
        }
    }
}
```

## Rendering System

### Visual Representation

**Paths:**
- Rectangular strips connecting waypoints
- Width: 6.0 units
- Color: Brown (0.5, 0.4, 0.3)
- Z-layer: 1.0 (behind villages)

**Villages:**
- Circular markers
- Radius: 8-25 units (based on size)
- Colors:
  - Hamlet: Grey
  - Village: Light grey
  - Town: Light yellow-grey
  - City: Bright gold
- Z-layer: 2.0

**Village Names:**
- Text displayed below village marker
- Font size: 14.0
- Color: White
- Only shown for discovered villages
- Z-layer: 3.0

**Fog of War Effect:**
- Undiscovered but visible villages: Dark grey (0.3, 0.3, 0.3) with 50% alpha
- No name displayed

### Rendering Pipeline

1. **Initial Render** (on map generation)
   - Spawn all discovered paths
   - Spawn all visible villages
   - Spawn village names for discovered settlements

2. **Dynamic Updates** (on discovery)
   - Despawn all existing visuals
   - Re-render with updated discovery state
   - Triggered by `WorldMap` resource changes

## Camera System

### World Map Camera

Separate camera from the travel view:
- **Default Position**: (0, 0, 1000) world center
- **State**: Inactive by default, activated on map view
- **Controls**: WASD/Arrows for panning

### Camera Controls

```rust
// Pan speed: 300 units/second
Arrow Keys / WASD: Pan camera
M key: Toggle between travel view and map view
```

### View Switching

- **M Key**: Toggle between travel and world map
- Switches game state: `Traveling` ↔ `MainMenu` (temporary)
- Activates appropriate camera
- Deactivates other camera

## UI Elements

### Map Overlay

Top bar with instructions:
- "World Map - Use Arrow Keys to Pan | Press M to close | Click village to travel"
- Font size: 18.0
- Center-justified

### Integration with HUD

The world map UI is separate from the travel HUD:
- Map UI only visible in map view state
- Travel HUD only visible in traveling state

## Usage Guide

### Opening the Map

1. Press `M` key during travel
2. Map view opens showing the world
3. Starting village and connected paths visible

### Navigation

1. Use WASD or Arrow keys to pan camera
2. View discovered areas and connected paths
3. Undiscovered villages appear as foggy circles

### Returning to Travel

1. Press `M` key again
2. Returns to travel view
3. Map state is preserved

## Future Enhancements

### Planned Features

1. **Click to Travel**
   - Click discovered village to set as destination
   - Show travel route on map
   - Estimate travel time

2. **Zoom Controls**
   - Q/E keys for zoom in/out
   - Orthographic camera scaling
   - Min/max zoom limits

3. **Map Legend**
   - Village size indicators
   - Distance scale
   - Compass rose

4. **Route Planning**
   - Pathfinding between villages
   - Show optimal route
   - Display total distance

5. **Dynamic Events**
   - Show active contracts on map
   - Highlight danger zones
   - Mark points of interest

6. **Map Annotations**
   - Player notes on villages
   - Markers for important locations
   - Quest objectives

## Technical Details

### Performance

- **Entity Count**: ~15 villages + ~30 paths = ~45 entities
- **Rendering**: Simple 2D meshes (circles and rectangles)
- **Updates**: Only on discovery (infrequent)
- **Memory**: Minimal (graph data structure)

### State Management

The world map is stored as a global resource:
```rust
commands.insert_resource(world_map);
```

Accessed in systems via:
```rust
fn system(world_map: Res<WorldMap>) { ... }
```

Modified via:
```rust
fn system(mut world_map: ResMut<WorldMap>) { ... }
```

### File Structure

```
src/
├── world_map.rs           # Core data structures
├── world_generator.rs     # Procedural generation
└── systems/
    └── world_map_render.rs  # Rendering and UI
```

## Integration with Game Systems

### Travel System

- World map provides available destinations
- Paths define travel routes
- Distance affects travel time

### Contract System (Future)

- Contracts reference village IDs
- Delivery destinations on the map
- Route planning for optimal delivery

### Encounter System (Future)

- Encounters can occur along paths
- Path waypoints as encounter locations
- Discovery unlocks new encounter areas

### Save/Load System (Future)

- Save discovery state
- Preserve village names (procedural seed)
- Restore map exploration progress

---

## Example: World Map Generation

```rust
// Generate world map
let config = WorldGenConfig::default();
let world_map = generate_world_map(config);

// Access starting village
let starting_village = &world_map.villages[&world_map.starting_village];
println!("Starting at: {}", starting_village.name);

// Discover a village
world_map.discover_village(5);

// Get connected villages
let connections = world_map.get_connected_villages(5);
for village_id in connections {
    println!("Connected to: {}", world_map.villages[&village_id].name);
}
```

---

**Implementation Status**: ✅ Complete
**Last Updated**: 2025-10-21
