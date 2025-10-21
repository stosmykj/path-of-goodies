use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Path of Goodies".to_string(),
                resolution: (800.0, 600.0).into(),
                canvas: Some("#bevy".to_string()), // For WASM
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_movement,
            camera_follow,
        ))
        .run();
}

// Components
#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct MainCamera;

// Startup system
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn camera
    commands.spawn((
        Camera2d,
        MainCamera,
    ));

    // Spawn player
    // TODO: Replace with actual sprite when assets are added
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.7, 0.3),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player { speed: 200.0 },
    ));

    // Add a simple background color
    commands.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)));

    info!("Game initialized! Use WASD or Arrow keys to move.");
}

// Player movement system
fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        // Keyboard input
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        // Normalize diagonal movement
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        // Apply movement
        transform.translation += direction * player.speed * time.delta_secs();
    }
}

// Camera follows player
fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Smoothly follow player
            camera_transform.translation = camera_transform.translation.lerp(
                player_transform.translation,
                0.1,
            );
            // Keep camera Z position
            camera_transform.translation.z = 999.9;
        }
    }
}
