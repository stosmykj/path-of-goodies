use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::world_map::*;
use crate::resources::GameState;

/// Camera for the world map view
#[derive(Component)]
pub struct WorldMapCamera;

/// Setup the world map camera
pub fn setup_world_map_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            is_active: false, // Start inactive, activate when in map view
            ..default()
        },
        WorldMapCamera,
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));

    info!("World map camera initialized");
}

/// Toggle world map view
pub fn toggle_world_map(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut main_camera: Query<&mut Camera, (With<crate::components::MainCamera>, Without<WorldMapCamera>)>,
    mut map_camera: Query<&mut Camera, (With<WorldMapCamera>, Without<crate::components::MainCamera>)>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        match current_state.get() {
            GameState::Traveling => {
                // Switch to map view
                next_state.set(GameState::MainMenu); // Using MainMenu as map view for now
                if let Ok(mut cam) = main_camera.get_single_mut() {
                    cam.is_active = false;
                }
                if let Ok(mut cam) = map_camera.get_single_mut() {
                    cam.is_active = true;
                }
                info!("Switched to world map view");
            }
            GameState::MainMenu => {
                // Switch back to traveling
                next_state.set(GameState::Traveling);
                if let Ok(mut cam) = main_camera.get_single_mut() {
                    cam.is_active = true;
                }
                if let Ok(mut cam) = map_camera.get_single_mut() {
                    cam.is_active = false;
                }
                info!("Switched back to travel view");
            }
            _ => {}
        }
    }
}

/// Render the world map - spawn all visual elements
pub fn render_world_map(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !world_map.is_changed() && !world_map.is_added() {
        return;
    }

    info!("Rendering world map with {} villages and {} paths",
        world_map.villages.len(), world_map.paths.len());

    // Render paths first (so they appear behind villages)
    for (path_id, path) in &world_map.paths {
        if path.is_discovered {
            spawn_path_visual(&mut commands, &mut meshes, &mut materials, path);
        }
    }

    // Render villages
    for (village_id, village) in &world_map.villages {
        spawn_village_visual(&mut commands, &mut meshes, &mut materials, village, &world_map);
    }
}

/// Create visual representation of a path
fn spawn_path_visual(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    path: &Path,
) {
    if path.waypoints.len() < 2 {
        return;
    }

    let path_width = 6.0;
    let path_color = Color::srgb(0.5, 0.4, 0.3); // Brown road color

    // Create path segments
    for i in 0..path.waypoints.len() - 1 {
        let start = path.waypoints[i];
        let end = path.waypoints[i + 1];
        let mid = (start + end) / 2.0;
        let length = start.distance(end);
        let angle = (end.y - start.y).atan2(end.x - start.x);

        // Create a rectangular mesh for the path segment
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(length, path_width)).into(),
                material: materials.add(ColorMaterial::from(path_color)),
                transform: Transform::from_xyz(mid.x, mid.y, 1.0)
                    .with_rotation(Quat::from_rotation_z(angle)),
                ..default()
            },
            PathMarker { path_id: path.id },
        ));
    }
}

/// Create visual representation of a village
fn spawn_village_visual(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    village: &Village,
    world_map: &WorldMap,
) {
    let position = village.position;
    let radius = village.size.radius();

    let (color, alpha) = if village.is_discovered {
        // Fully visible
        (village.size.color(), 1.0)
    } else {
        // Under fog of war - check if connected to discovered village
        let connected_to_discovered = world_map
            .get_connected_villages(village.id)
            .iter()
            .any(|id| world_map.discovered_villages.contains(id));

        if connected_to_discovered {
            // Show as foggy/dimmed
            (Color::srgb(0.3, 0.3, 0.3), 0.5)
        } else {
            // Completely hidden
            return;
        }
    };

    // Create circle mesh for village
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(radius)).into(),
            material: materials.add(ColorMaterial::from(color.with_alpha(alpha))),
            transform: Transform::from_xyz(position.x, position.y, 2.0),
            ..default()
        },
        VillageMarker { village_id: village.id },
    ));

    // Add village name text if discovered
    if village.is_discovered {
        commands.spawn((
            Text2d::new(&village.name),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
            Transform::from_xyz(position.x, position.y - radius - 15.0, 3.0),
            VillageMarker { village_id: village.id },
        ));
    }
}

/// Update world map visuals when discoveries change
pub fn update_world_map_visuals(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    path_markers: Query<(Entity, &PathMarker)>,
    village_markers: Query<(Entity, &VillageMarker)>,
) {
    if !world_map.is_changed() {
        return;
    }

    // Despawn all existing visuals
    for (entity, _) in path_markers.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for (entity, _) in village_markers.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Re-render
    render_world_map(commands, world_map, meshes, materials);
}

/// Camera controls for world map (pan and zoom)
pub fn world_map_camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<WorldMapCamera>>,
) {
    if let Ok(mut transform) = camera_query.get_single_mut() {
        let pan_speed = 300.0;
        let zoom_speed = 2.0;

        // Pan controls
        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
            transform.translation.y += pan_speed * time.delta_secs();
        }
        if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
            transform.translation.y -= pan_speed * time.delta_secs();
        }
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            transform.translation.x -= pan_speed * time.delta_secs();
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            transform.translation.x += pan_speed * time.delta_secs();
        }

        // Zoom controls (not implemented yet - would need orthographic projection)
        // TODO: Add zoom with Q/E keys
    }
}

/// Add map UI overlay with instructions
pub fn setup_map_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            WorldMapView,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("World Map - Use Arrow Keys to Pan | Press M to close | Click village to travel"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));
        });
}
