use bevy::prelude::*;
use crate::components::*;
use rand::Rng;

/// Horse taming mini-game state
#[derive(Resource, Default)]
pub struct HorseTamingState {
    pub active: bool,
    pub attempts_remaining: u32,
    pub success_threshold: u32,
    pub current_progress: u32,
}

/// Component marker for taming UI
#[derive(Component)]
pub struct HorseTamingUI;

/// Start horse taming mini-game
pub fn start_horse_taming(
    mut taming_state: ResMut<HorseTamingState>,
) {
    let mut rng = rand::thread_rng();

    taming_state.active = true;
    taming_state.attempts_remaining = 5;
    taming_state.success_threshold = rng.gen_range(8..15);
    taming_state.current_progress = 0;

    info!("🐴 Horse taming started! Press SPACE repeatedly to tame the horse!");
}

/// Show taming UI
pub fn show_horse_taming_ui(
    mut commands: Commands,
    taming_state: Res<HorseTamingState>,
    existing_ui: Query<Entity, With<HorseTamingUI>>,
) {
    if taming_state.active && existing_ui.is_empty() {
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                HorseTamingUI,
            ))
            .with_children(|parent| {
                parent
                    .spawn(Node {
                        width: Val::Px(500.0),
                        height: Val::Px(300.0),
                        padding: UiRect::all(Val::Px(30.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(20.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .insert(BackgroundColor(Color::srgb(0.1, 0.15, 0.1)))
                    .with_children(|panel| {
                        panel.spawn((
                            Text::new("🐴 Taming Wild Horse!"),
                            TextFont {
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.9, 0.6)),
                        ));

                        // Progress bar representation
                        let progress_text = "█".repeat(taming_state.current_progress as usize);
                        let remaining_text = "░".repeat((taming_state.success_threshold - taming_state.current_progress) as usize);

                        panel.spawn((
                            Text::new(&format!("Progress: {}{}", progress_text, remaining_text)),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 1.0, 0.7)),
                        ));

                        panel.spawn((
                            Text::new(&format!("Attempts Left: {}", taming_state.attempts_remaining)),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 1.0, 0.7)),
                        ));

                        panel.spawn((
                            Text::new("\nPress SPACE rapidly to calm the horse!"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.6, 0.8, 1.0)),
                        ));
                    });
            });
    }
}

/// Handle taming input
pub fn handle_horse_taming(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut taming_state: ResMut<HorseTamingState>,
    wagon_query: Query<Entity, With<Wagon>>,
    taming_ui: Query<Entity, With<HorseTamingUI>>,
    mut next_state: ResMut<NextState<crate::resources::GameState>>,
) {
    if !taming_state.active {
        return;
    }

    if keyboard.just_pressed(KeyCode::Space) {
        let mut rng = rand::thread_rng();

        // Random success on each press (30% chance)
        if rng.gen::<f32>() < 0.3 {
            taming_state.current_progress += 1;
            info!("Progress: {}/{}", taming_state.current_progress, taming_state.success_threshold);
        }

        taming_state.attempts_remaining -= 1;

        // Check for success
        if taming_state.current_progress >= taming_state.success_threshold {
            info!("🎉 Successfully tamed the wild horse!");

            // Add new horse to wagon
            if let Ok(wagon_entity) = wagon_query.get_single() {
                commands.entity(wagon_entity).insert(Horse::default());
            }

            // Close UI
            for entity in taming_ui.iter() {
                commands.entity(entity).despawn_recursive();
            }

            // Reset state
            taming_state.active = false;
            next_state.set(crate::resources::GameState::Traveling);
        }
        // Check for failure
        else if taming_state.attempts_remaining == 0 {
            info!("😔 Failed to tame the horse. It ran away.");

            // Close UI
            for entity in taming_ui.iter() {
                commands.entity(entity).despawn_recursive();
            }

            // Reset state
            taming_state.active = false;
            next_state.set(crate::resources::GameState::Traveling);
        }
    }
}

/// Close taming UI
pub fn close_horse_taming_ui(
    mut commands: Commands,
    taming_ui: Query<Entity, With<HorseTamingUI>>,
) {
    for entity in taming_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
