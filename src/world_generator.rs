use bevy::prelude::*;
use rand::Rng;
use crate::world_map::{WorldMap, Village, VillageSize, Path};

/// Configuration for world generation
pub struct WorldGenConfig {
    pub num_villages: usize,
    pub world_size: f32,
    pub min_distance: f32,
    pub max_connections_per_village: usize,
    pub path_curve_chance: f32,
    pub path_curve_strength: f32,
    pub seed: u64,
}

impl Default for WorldGenConfig {
    fn default() -> Self {
        Self {
            num_villages: 15,
            world_size: 2000.0,
            min_distance: 200.0,
            max_connections_per_village: 4,
            path_curve_chance: 0.3,
            path_curve_strength: 100.0,
            seed: 12345,
        }
    }
}

/// Generate a complete world map
pub fn generate_world_map(config: WorldGenConfig) -> WorldMap {
    let mut rng = rand::thread_rng();
    let mut map = WorldMap::new();

    // Generate village positions using Poisson disk sampling (simplified)
    let village_positions = generate_village_positions(&config, &mut rng);

    // Create villages
    for (i, pos) in village_positions.iter().enumerate() {
        let size = determine_village_size(i, config.num_villages, &mut rng);
        let name = generate_village_name(i, &mut rng);

        let village = Village {
            id: i,
            name,
            position: *pos,
            size,
            is_discovered: false,
        };

        map.villages.insert(i, village);
        map.adjacency.insert(i, Vec::new());
    }

    // Generate paths using a modified Delaunay-like approach
    let paths = generate_paths(&map, &config, &mut rng);

    for (i, path) in paths.into_iter().enumerate() {
        // Add to adjacency list
        map.adjacency.entry(path.village_a).or_default().push(i);
        map.adjacency.entry(path.village_b).or_default().push(i);

        map.paths.insert(i, path);
    }

    // Set starting village (choose a medium-sized one near center)
    map.starting_village = find_starting_village(&map);

    // Discover starting village
    map.discover_village(map.starting_village);

    map
}

/// Generate village positions with minimum distance constraint
fn generate_village_positions(config: &WorldGenConfig, rng: &mut impl Rng) -> Vec<Vec2> {
    let mut positions = Vec::new();
    let half_size = config.world_size / 2.0;

    // Add first village near center
    positions.push(Vec2::new(
        rng.gen_range(-50.0..50.0),
        rng.gen_range(-50.0..50.0),
    ));

    // Generate remaining villages
    let mut attempts = 0;
    let max_attempts = config.num_villages * 100;

    while positions.len() < config.num_villages && attempts < max_attempts {
        attempts += 1;

        let x = rng.gen_range(-half_size..half_size);
        let y = rng.gen_range(-half_size..half_size);
        let pos = Vec2::new(x, y);

        // Check minimum distance to all existing villages
        let mut valid = true;
        for existing_pos in &positions {
            if pos.distance(*existing_pos) < config.min_distance {
                valid = false;
                break;
            }
        }

        if valid {
            positions.push(pos);
        }
    }

    positions
}

/// Determine village size based on index and randomness
fn determine_village_size(index: usize, total: usize, rng: &mut impl Rng) -> VillageSize {
    if index == 0 {
        return VillageSize::Village; // Starting village is always a village
    }

    let roll: f32 = rng.gen();
    if index < total / 8 {
        // First few are more likely to be larger
        if roll < 0.1 {
            VillageSize::City
        } else if roll < 0.4 {
            VillageSize::Town
        } else {
            VillageSize::Village
        }
    } else {
        // Rest are smaller
        if roll < 0.05 {
            VillageSize::City
        } else if roll < 0.2 {
            VillageSize::Town
        } else if roll < 0.5 {
            VillageSize::Village
        } else {
            VillageSize::Hamlet
        }
    }
}

/// Generate a random village name
fn generate_village_name(index: usize, rng: &mut impl Rng) -> String {
    let prefixes = [
        "North", "South", "East", "West", "Old", "New", "High", "Low",
        "Stone", "Wood", "Iron", "Silver", "Gold", "Green", "Red",
        "White", "Black", "Grey", "Fair", "Dark",
    ];

    let roots = [
        "haven", "ford", "bridge", "mill", "field", "wood", "vale",
        "ton", "bury", "chester", "port", "mount", "hill", "dale",
        "crest", "reach", "fall", "gate", "hold", "watch",
    ];

    if index == 0 {
        return "Silvermill".to_string(); // Starting village has fixed name
    }

    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let root = roots[rng.gen_range(0..roots.len())];

    format!("{}{}", prefix, root)
}

/// Generate paths between villages
fn generate_paths(map: &WorldMap, config: &WorldGenConfig, rng: &mut impl Rng) -> Vec<Path> {
    let mut paths = Vec::new();
    let mut connections: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();

    // Get all village pairs sorted by distance
    let mut pairs = Vec::new();
    for (id_a, village_a) in &map.villages {
        for (id_b, village_b) in &map.villages {
            if id_a < id_b {
                let distance = village_a.position.distance(village_b.position);
                pairs.push((*id_a, *id_b, distance));
            }
        }
    }

    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // Connect villages using nearest neighbor with max connections limit
    for (id_a, id_b, _distance) in pairs {
        let count_a = *connections.get(&id_a).unwrap_or(&0);
        let count_b = *connections.get(&id_b).unwrap_or(&0);

        if count_a >= config.max_connections_per_village || count_b >= config.max_connections_per_village {
            continue;
        }

        let village_a = &map.villages[&id_a];
        let village_b = &map.villages[&id_b];

        // Check for path crossing (simplified)
        let mut crosses = false;
        for existing_path in &paths {
            if paths_cross(
                village_a.position,
                village_b.position,
                existing_path.waypoints[0],
                existing_path.waypoints[existing_path.waypoints.len() - 1],
            ) {
                crosses = true;
                break;
            }
        }

        if !crosses {
            // Decide if path should be curved
            let path = if rng.gen::<f32>() < config.path_curve_chance {
                let curve = rng.gen_range(-config.path_curve_strength..config.path_curve_strength);
                Path::curved(paths.len(), id_a, id_b, village_a.position, village_b.position, curve)
            } else {
                Path::straight(paths.len(), id_a, id_b, village_a.position, village_b.position)
            };

            paths.push(path);

            *connections.entry(id_a).or_insert(0) += 1;
            *connections.entry(id_b).or_insert(0) += 1;
        }
    }

    // Ensure all villages are connected (add missing connections)
    ensure_connectivity(map, &mut paths, &connections, rng);

    paths
}

/// Simple line intersection check
fn paths_cross(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> bool {
    let d1 = a2 - a1;
    let d2 = b2 - b1;
    let d3 = b1 - a1;

    let cross = d1.x * d2.y - d1.y * d2.x;
    if cross.abs() < 0.001 {
        return false; // Parallel
    }

    let t1 = (d3.x * d2.y - d3.y * d2.x) / cross;
    let t2 = (d3.x * d1.y - d3.y * d1.x) / cross;

    t1 > 0.0 && t1 < 1.0 && t2 > 0.0 && t2 < 1.0
}

/// Ensure all villages are connected to the graph
fn ensure_connectivity(
    map: &WorldMap,
    paths: &mut Vec<Path>,
    connections: &std::collections::HashMap<usize, usize>,
    rng: &mut impl Rng,
) {
    // Find unconnected villages
    for (id, _village) in &map.villages {
        if !connections.contains_key(id) || connections[id] == 0 {
            // Find nearest connected village
            if let Some(nearest) = find_nearest_connected_village(*id, map, connections) {
                let village_a = &map.villages[id];
                let village_b = &map.villages[&nearest];

                let path = Path::straight(
                    paths.len(),
                    *id,
                    nearest,
                    village_a.position,
                    village_b.position,
                );

                paths.push(path);
            }
        }
    }
}

/// Find the nearest village that has connections
fn find_nearest_connected_village(
    village_id: usize,
    map: &WorldMap,
    connections: &std::collections::HashMap<usize, usize>,
) -> Option<usize> {
    let village = &map.villages[&village_id];
    let mut nearest = None;
    let mut min_distance = f32::MAX;

    for (id, other_village) in &map.villages {
        if *id != village_id && connections.contains_key(id) && connections[id] > 0 {
            let distance = village.position.distance(other_village.position);
            if distance < min_distance {
                min_distance = distance;
                nearest = Some(*id);
            }
        }
    }

    nearest
}

/// Find a good starting village
fn find_starting_village(map: &WorldMap) -> usize {
    // Prefer villages near center with medium size
    let mut best_village = 0;
    let mut best_score = f32::MIN;

    for (id, village) in &map.villages {
        let distance_from_center = village.position.length();
        let size_score = match village.size {
            VillageSize::Hamlet => 1.0,
            VillageSize::Village => 3.0,
            VillageSize::Town => 2.0,
            VillageSize::City => 1.5,
        };

        // Score: prefer villages near center with good size
        let score = size_score - (distance_from_center / 500.0);

        if score > best_score {
            best_score = score;
            best_village = *id;
        }
    }

    best_village
}
