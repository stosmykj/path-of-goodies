use bevy::prelude::*;

mod components;
mod resources;
mod systems;
mod world_map;
mod world_generator;
mod contracts;
mod towns;

use components::*;
use resources::*;
use systems::*;
use world_map::*;
use world_generator::*;
use contracts::*;
use towns::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Path of Goodies - Medieval Trading Adventure".to_string(),
                resolution: (1280.0, 720.0).into(),
                canvas: Some("#bevy".to_string()), // For WASM
                ..default()
            }),
            ..default()
        }))
        // Initialize game state
        .init_state::<GameState>()
        // Initialize resources
        .insert_resource(GameTime::new())
        .insert_resource(TravelInfo::default())
        .insert_resource(GameSettings::default())
        .insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.9))) // Day time default
        .insert_resource(TownsDatabase::new())
        .insert_resource(AvailableContracts::new())
        .insert_resource(CurrentTown::default())
        // Startup systems
        .add_systems(Startup, (
            setup_world_map_resource,
            setup_towns_database,
            generate_contracts,
            setup_world,
            setup_camera,
            setup_wagon,
            setup_hud,
            setup_world_map_camera,
            setup_map_ui,
        ))
        // Update systems - run during Traveling state
        .add_systems(Update, (
            wagon_movement,
            camera_follow,
            update_horse_stamina,
            update_game_time,
            update_ambient_lighting,
            update_travel_progress,
            handle_arrival,
            handle_whip,
            update_whip_timers,
            toggle_camping,
        ).run_if(in_state(GameState::Traveling)))
        // World map systems - run in MainMenu state (using as map view)
        .add_systems(Update, (
            render_world_map,
            update_world_map_visuals,
            world_map_camera_controls,
            handle_village_click,
        ).run_if(in_state(GameState::MainMenu)))
        // Town systems - run in Town state
        .add_systems(OnEnter(GameState::Town), (
            setup_town_ui,
            check_contract_completion,
        ))
        .add_systems(Update, (
            handle_town_interactions,
            show_contracts_ui,
            handle_contract_acceptance,
        ).run_if(in_state(GameState::Town)))
        .add_systems(OnExit(GameState::Town), (
            close_town_ui,
        ))
        // Camping systems - run in Camping state
        .add_systems(OnEnter(GameState::Camping), (
            setup_camping_ui,
            rest_at_camp,
        ))
        .add_systems(Update, (
            toggle_camping,
        ).run_if(in_state(GameState::Camping)))
        .add_systems(OnExit(GameState::Camping), (
            close_camping_ui,
        ))
        // Global systems - run in all states
        .add_systems(Update, (
            toggle_world_map,
        ))
        // UI update systems - run in all states
        .add_systems(Update, (
            update_resources_display,
            update_time_display,
            update_horse_status_display,
            update_travel_info_display,
        ))
        .run();
}

/// Setup the main camera
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        MainCamera,
    ));
    info!("Camera initialized");
}

/// Setup the player wagon with initial stats
fn setup_wagon(mut commands: Commands) {
    // Spawn the wagon entity with all components
    commands.spawn((
        // Visual
        Sprite {
            color: Color::srgb(0.6, 0.4, 0.2), // Brown wagon placeholder
            custom_size: Some(Vec2::new(48.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        // Game components
        Wagon::default(),
        Horse::default(),
        PlayerInventory::default(),
        TravelState::default(),
        Velocity::default(),
        WhipCooldown::default(),
        Name::new("Player Wagon"),
    ));

    info!("Wagon initialized with default stats");
    info!("Use WASD or Arrow keys to move");
    info!("Press SPACE to whip horse for speed boost");
    info!("Press C to set up camp");
}

/// Generate and initialize the world map
fn setup_world_map_resource(mut commands: Commands) {
    let config = WorldGenConfig::default();
    let world_map = generate_world_map(config);

    info!("World map generated:");
    info!("  - {} villages", world_map.villages.len());
    info!("  - {} paths", world_map.paths.len());
    info!("  - Starting village: {} ({})",
        world_map.starting_village,
        world_map.villages.get(&world_map.starting_village)
            .map(|v| v.name.as_str())
            .unwrap_or("Unknown")
    );
    info!("Press M to toggle world map view");

    commands.insert_resource(world_map);
}

/// Setup towns database from world map villages
fn setup_towns_database(
    mut towns_db: ResMut<TownsDatabase>,
    world_map: Res<WorldMap>,
) {
    for (id, village) in &world_map.villages {
        let town_data = TownData::new(*id, village.size);
        towns_db.add_town(town_data);
    }

    info!("Towns database initialized with {} towns", towns_db.towns.len());
}

/// Generate contracts for all villages
fn generate_contracts(
    mut contracts: ResMut<AvailableContracts>,
    world_map: Res<WorldMap>,
) {
    for (village_id, village) in &world_map.villages {
        // Get connected villages for this village
        let mut connected: Vec<(usize, String, f32)> = Vec::new();

        if let Some(path_ids) = world_map.adjacency.get(village_id) {
            for path_id in path_ids {
                if let Some(path) = world_map.paths.get(path_id) {
                    let other_id = if path.village_a == *village_id {
                        path.village_b
                    } else {
                        path.village_a
                    };

                    if let Some(other_village) = world_map.villages.get(&other_id) {
                        connected.push((other_id, other_village.name.clone(), path.distance));
                    }
                }
            }
        }

        // Generate contracts for this village
        let village_contracts = generate_contracts_for_village(
            *village_id,
            &village.name,
            &connected,
        );

        for contract in village_contracts {
            contracts.add_contract(contract);
        }
    }

    info!("Generated {} contracts across all villages", contracts.contracts.len());
}
