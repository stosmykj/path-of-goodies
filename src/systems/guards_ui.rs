use bevy::prelude::*;
use crate::guards::*;
use crate::components::*;
use crate::towns::CurrentTown;
use crate::world_map::WorldMap;

/// Marker for guard hiring UI
#[derive(Component)]
pub struct GuardHireUI;

/// Marker for hired guards display UI
#[derive(Component)]
pub struct HiredGuardsDisplay;

/// Show guard hiring UI
pub fn show_guard_hire_ui(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    current_town: Res<CurrentTown>,
    guards_for_hire: Res<GuardsForHire>,
    hired_guards: Res<HiredGuards>,
    world_map: Res<WorldMap>,
    existing_ui: Query<Entity, With<GuardHireUI>>,
) {
    // Toggle with H key (only in town)
    if keyboard.just_pressed(KeyCode::KeyH) {
        if !existing_ui.is_empty() {
            // Close if already open
            for entity in existing_ui.iter() {
                commands.entity(entity).despawn_recursive();
            }
            return;
        }

        if let Some(village_id) = current_town.village_id {
            let village = world_map.villages.get(&village_id);
            let village_name = village.map(|v| v.name.as_str()).unwrap_or("Unknown");

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
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                    GuardHireUI,
                ))
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            width: Val::Px(750.0),
                            height: Val::Px(650.0),
                            padding: UiRect::all(Val::Px(25.0)),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            ..default()
                        })
                        .insert(BackgroundColor(Color::srgb(0.12, 0.1, 0.15)))
                        .with_children(|panel| {
                            // Title
                            panel.spawn((
                                Text::new(&format!("Hire Guards - {}", village_name)),
                                TextFont {
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.9, 0.6)),
                            ));

                            // Currently hired info
                            panel.spawn((
                                Text::new(&format!(
                                    "Your Guards: {}/{} | Daily Cost: {:.1}g",
                                    hired_guards.guards.len(),
                                    hired_guards.max_guards,
                                    hired_guards.total_daily_cost()
                                )),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.8, 0.8, 1.0)),
                            ));

                            // Available guards
                            panel.spawn((
                                Text::new("\nAvailable for Hire:"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.9, 0.6)),
                            ));

                            if guards_for_hire.guards.is_empty() {
                                panel.spawn((
                                    Text::new("No guards available in this town."),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                ));
                            } else {
                                for (i, guard) in guards_for_hire.guards.iter().enumerate() {
                                    panel
                                        .spawn(Node {
                                            width: Val::Percent(100.0),
                                            padding: UiRect::all(Val::Px(12.0)),
                                            flex_direction: FlexDirection::Column,
                                            row_gap: Val::Px(5.0),
                                            ..default()
                                        })
                                        .insert(BackgroundColor(Color::srgb(0.18, 0.16, 0.2)))
                                        .with_children(|item| {
                                            // Guard name and type
                                            item.spawn((
                                                Text::new(&format!("{}. {}", i + 1, guard.name)),
                                                TextFont {
                                                    font_size: 20.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                                            ));

                                            // Stats
                                            item.spawn((
                                                Text::new(&format!(
                                                    "Health: {:.0} | Damage: {:.0} | Armor: {:.0}",
                                                    guard.health, guard.damage, guard.armor
                                                )),
                                                TextFont {
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                            ));

                                            // Cost
                                            item.spawn((
                                                Text::new(&format!("Salary: {:.1}g per day", guard.salary)),
                                                TextFont {
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(1.0, 0.9, 0.5)),
                                            ));

                                            // Instructions
                                            if hired_guards.can_hire_more() {
                                                item.spawn((
                                                    Text::new(&format!("Press {} to hire", i + 1)),
                                                    TextFont {
                                                        font_size: 14.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::srgb(0.6, 1.0, 0.6)),
                                                ));
                                            } else {
                                                item.spawn((
                                                    Text::new("Party full!"),
                                                    TextFont {
                                                        font_size: 14.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::srgb(1.0, 0.5, 0.5)),
                                                ));
                                            }
                                        });
                                }
                            }

                            // Instructions
                            panel.spawn((
                                Text::new("\nPress H to close | Press 1-9 to hire | Press G to view your guards"),
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
}

/// Handle guard hiring
pub fn handle_guard_hiring(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut guards_for_hire: ResMut<GuardsForHire>,
    mut hired_guards: ResMut<HiredGuards>,
    mut inventory: Query<&mut PlayerInventory>,
    guard_ui: Query<Entity, With<GuardHireUI>>,
    mut commands: Commands,
) {
    if guard_ui.is_empty() {
        return; // UI not open
    }

    let keys = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
    ];

    for (i, key) in keys.iter().enumerate() {
        if keyboard.just_pressed(*key) && i < guards_for_hire.guards.len() {
            if !hired_guards.can_hire_more() {
                info!("Cannot hire more guards! Maximum reached.");
                continue;
            }

            let guard = guards_for_hire.guards.remove(i);
            let hiring_fee = guard.salary * 3.0; // 3 days upfront

            if let Ok(mut inv) = inventory.get_single_mut() {
                if inv.gold >= hiring_fee {
                    inv.gold -= hiring_fee;
                    info!(
                        "Hired {} for {:.1}g (3 days advance payment)",
                        guard.name, hiring_fee
                    );
                    hired_guards.hire(guard);

                    // Close UI
                    for entity in guard_ui.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                } else {
                    info!(
                        "Not enough gold! Need {:.1}g (3 days salary)",
                        hiring_fee
                    );
                    // Add guard back
                    guards_for_hire.guards.insert(i, guard);
                }
            }

            break;
        }
    }
}

/// Show hired guards display (G key)
pub fn show_hired_guards_display(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    hired_guards: Res<HiredGuards>,
    existing_ui: Query<Entity, With<HiredGuardsDisplay>>,
) {
    if keyboard.just_pressed(KeyCode::KeyG) {
        if !existing_ui.is_empty() {
            for entity in existing_ui.iter() {
                commands.entity(entity).despawn_recursive();
            }
            return;
        }

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
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
                HiredGuardsDisplay,
            ))
            .with_children(|parent| {
                parent
                    .spawn(Node {
                        width: Val::Px(600.0),
                        height: Val::Px(500.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.0),
                        ..default()
                    })
                    .insert(BackgroundColor(Color::srgb(0.1, 0.12, 0.15)))
                    .with_children(|panel| {
                        panel.spawn((
                            Text::new("Your Guards"),
                            TextFont {
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.9, 0.6)),
                        ));

                        panel.spawn((
                            Text::new(&format!(
                                "Party: {}/{} | Daily Cost: {:.1}g",
                                hired_guards.guards.len(),
                                hired_guards.max_guards,
                                hired_guards.total_daily_cost()
                            )),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 1.0)),
                        ));

                        if hired_guards.guards.is_empty() {
                            panel.spawn((
                                Text::new("\nNo guards hired. Press H in a town to hire guards."),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                            ));
                        } else {
                            for (i, guard) in hired_guards.guards.iter().enumerate() {
                                panel
                                    .spawn(Node {
                                        width: Val::Percent(100.0),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    })
                                    .insert(BackgroundColor(Color::srgb(0.16, 0.14, 0.18)))
                                    .with_children(|item| {
                                        item.spawn((
                                            Text::new(&format!(
                                                "{}: {} | HP: {:.0}/{:.0} | DMG: {:.0} | ARM: {:.0} | {:.1}g/day",
                                                i + 1,
                                                guard.data.name,
                                                guard.data.health,
                                                guard.data.max_health,
                                                guard.data.damage,
                                                guard.data.armor,
                                                guard.data.salary
                                            )),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                        ));
                                    });
                            }
                        }

                        panel.spawn((
                            Text::new("\nPress G to close"),
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

/// Pay guard salaries each day
pub fn pay_guard_salaries(
    mut hired_guards: ResMut<HiredGuards>,
    mut inventory: Query<&mut PlayerInventory>,
    game_time: Res<crate::resources::GameTime>,
) {
    // Pay at the start of each new day (when hour wraps around)
    // For simplicity, we'll check if we're early in the day
    if game_time.hour >= 0.0 && game_time.hour < 1.0 {
        // Only pay once per day - track last payment day
        // For now, this will pay multiple times, TODO: add resource to track last pay day
        if let Ok(mut inv) = inventory.get_single_mut() {
            hired_guards.pay_daily_salaries(&mut inv);
        }
    }
}
