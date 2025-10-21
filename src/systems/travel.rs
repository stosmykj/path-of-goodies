use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::world_map::WorldMap;

/// Travel progress component - attached to wagon
#[derive(Component)]
pub struct TravelProgress {
    pub current_path_id: Option<usize>,
    pub progress: f32, // 0.0 to 1.0 along the path
    pub origin_village: usize,
    pub destination_village: usize,
}

/// System to update travel progress
pub fn update_travel_progress(
    time: Res<Time>,
    world_map: Res<WorldMap>,
    mut travel_query: Query<(&TravelState, &mut TravelProgress, &mut Transform)>,
    mut travel_info: ResMut<TravelInfo>,
) {
    for (travel_state, mut progress, mut transform) in travel_query.iter_mut() {
        if let Some(path_id) = progress.current_path_id {
            if let Some(path) = world_map.paths.get(&path_id) {
                // Calculate travel speed (units per second)
                let speed = travel_state.current_speed;

                if speed > 0.0 && travel_state.is_moving {
                    // Update progress based on speed and path distance
                    let distance_traveled = speed * time.delta_secs();
                    let progress_increment = distance_traveled / path.distance;

                    progress.progress += progress_increment;

                    // Update visual position along path
                    if progress.progress >= 1.0 {
                        // Arrived at destination
                        progress.progress = 1.0;
                        if let Some(dest_village) = world_map.villages.get(&progress.destination_village) {
                            transform.translation.x = dest_village.position.x;
                            transform.translation.y = dest_village.position.y;

                            info!("Arrived at {}!", dest_village.name);
                            // Progress will be handled by arrival system
                        }
                    } else {
                        // Interpolate position along path
                        let pos = interpolate_path_position(&path.waypoints, progress.progress);
                        transform.translation.x = pos.x;
                        transform.translation.y = pos.y;
                    }

                    // Update travel info
                    travel_info.distance_remaining = path.distance * (1.0 - progress.progress);
                }
            }
        }
    }
}

/// Interpolate position along a path with waypoints
fn interpolate_path_position(waypoints: &[Vec2], progress: f32) -> Vec2 {
    if waypoints.len() < 2 {
        return Vec2::ZERO;
    }

    // Calculate total distance
    let mut segment_distances = Vec::new();
    let mut total_distance = 0.0;

    for i in 0..waypoints.len() - 1 {
        let dist = waypoints[i].distance(waypoints[i + 1]);
        segment_distances.push(dist);
        total_distance += dist;
    }

    // Find which segment we're on
    let target_distance = total_distance * progress;
    let mut accumulated_distance = 0.0;

    for (i, &segment_dist) in segment_distances.iter().enumerate() {
        if accumulated_distance + segment_dist >= target_distance {
            // We're on this segment
            let segment_progress = (target_distance - accumulated_distance) / segment_dist;
            return waypoints[i].lerp(waypoints[i + 1], segment_progress);
        }
        accumulated_distance += segment_dist;
    }

    // Fallback to last waypoint
    waypoints[waypoints.len() - 1]
}

/// Handle arrival at destination village
pub fn handle_arrival(
    mut commands: Commands,
    mut travel_query: Query<(Entity, &TravelProgress)>,
    world_map: Res<WorldMap>,
    mut current_town: ResMut<crate::towns::CurrentTown>,
    mut next_state: ResMut<NextState<GameState>>,
    mut world_map_mut: ResMut<WorldMap>,
    mut inventory: Query<&mut PlayerInventory>,
) {
    for (entity, travel_progress) in travel_query.iter_mut() {
        if travel_progress.progress >= 1.0 {
            // Arrived!
            let dest_id = travel_progress.destination_village;

            // Discover the destination village
            world_map_mut.discover_village(dest_id);

            // Consume resources at destination (per design doc)
            if let Ok(mut inv) = inventory.get_single_mut() {
                let party_size = 1.0; // TODO: Count actual party members
                let food_cost = 5.0 * party_size;
                let water_cost = 8.0 * party_size;

                inv.food = (inv.food - food_cost).max(0.0);
                inv.water = (inv.water - water_cost).max(0.0);

                info!("Consumed {:.0} food and {:.0} water at destination", food_cost, water_cost);

                // Check for starvation/dehydration
                if inv.food <= 0.0 {
                    warn!("Warning: Out of food!");
                }
                if inv.water <= 0.0 {
                    warn!("Warning: Out of water!");
                }
            }

            // Enter town
            current_town.village_id = Some(dest_id);
            next_state.set(GameState::Town);

            // Remove travel progress component
            commands.entity(entity).remove::<TravelProgress>();

            if let Some(village) = world_map.villages.get(&dest_id) {
                info!("Entered {}", village.name);
            }
        }
    }
}

/// Start travel to a destination village
pub fn start_travel_to_village(
    commands: &mut Commands,
    wagon_entity: Entity,
    origin_village: usize,
    destination_village: usize,
    world_map: &WorldMap,
    travel_info: &mut TravelInfo,
) -> Result<(), String> {
    // Find path between villages
    if let Some(path_ids) = world_map.adjacency.get(&origin_village) {
        for path_id in path_ids {
            if let Some(path) = world_map.paths.get(path_id) {
                if (path.village_a == origin_village && path.village_b == destination_village)
                    || (path.village_a == destination_village && path.village_b == origin_village)
                {
                    // Found the path!
                    let origin = world_map.villages.get(&origin_village).ok_or("Origin not found")?;
                    let dest = world_map.villages.get(&destination_village).ok_or("Destination not found")?;

                    commands.entity(wagon_entity).insert(TravelProgress {
                        current_path_id: Some(*path_id),
                        progress: 0.0,
                        origin_village,
                        destination_village,
                    });

                    travel_info.origin = origin.name.clone();
                    travel_info.destination = dest.name.clone();
                    travel_info.total_distance = path.distance;
                    travel_info.distance_remaining = path.distance;

                    info!("Started travel from {} to {} ({:.0} units)",
                          origin.name, dest.name, path.distance);

                    return Ok(());
                }
            }
        }
    }

    Err("No direct path found between villages".to_string())
}

/// System to consume resources during camping
pub fn consume_camping_resources(
    mut inventory: Query<&mut PlayerInventory>,
) {
    if let Ok(mut inv) = inventory.get_single_mut() {
        let party_size = 1.0; // TODO: Count actual party members
        let food_cost = 10.0 * party_size;
        let water_cost = 15.0 * party_size;

        inv.food = (inv.food - food_cost).max(0.0);
        inv.water = (inv.water - water_cost).max(0.0);

        info!("Consumed {:.0} food and {:.0} water while camping", food_cost, water_cost);

        // Check for starvation/dehydration
        if inv.food <= 0.0 {
            warn!("Warning: Out of food!");
        }
        if inv.water <= 0.0 {
            warn!("Warning: Out of water!");
        }
    }
}
