use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use components::*;
use resources::*;
use systems::*;

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
        // Startup systems
        .add_systems(Startup, (
            setup_world,
            setup_camera,
            setup_wagon,
            setup_hud,
        ))
        // Update systems - run during Traveling state
        .add_systems(Update, (
            wagon_movement,
            camera_follow,
            update_horse_stamina,
            update_game_time,
            update_ambient_lighting,
        ).run_if(in_state(GameState::Traveling)))
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
        Name::new("Player Wagon"),
    ));

    info!("Wagon initialized with default stats");
    info!("Use WASD or Arrow keys to move");
}
