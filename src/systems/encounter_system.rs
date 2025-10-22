use bevy::prelude::*;
use crate::encounters::*;
use crate::components::*;
use crate::resources::*;
use crate::systems::travel::TravelProgress;

/// Marker for encounter UI
#[derive(Component)]
pub struct EncounterUI;

/// Marker for encounter choice buttons
#[derive(Component)]
pub struct EncounterChoiceButton {
    pub choice_index: usize,
}

/// Check for random encounters during travel
pub fn check_for_encounter(
    mut encounter_chance: ResMut<EncounterChance>,
    mut active_encounter: ResMut<ActiveEncounter>,
    travel_query: Query<(&TravelState, &TravelProgress)>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    // Only check during actual travel
    for (travel_state, progress) in travel_query.iter() {
        if travel_state.is_moving && travel_state.current_speed > 0.0 {
            // Accumulate distance
            let distance = travel_state.current_speed * time.delta_secs();
            encounter_chance.distance_since_last += distance;

            // Check for encounter every 100 units
            if encounter_chance.distance_since_last >= 100.0 {
                let mut rng = rand::thread_rng();
                use rand::Rng;

                let roll: f32 = rng.gen();

                if roll < encounter_chance.base_chance_per_100_units {
                    // Encounter triggered!
                    if let Some(encounter_type) = roll_encounter() {
                        info!("⚠️ Encounter: {}", encounter_type.name());

                        let encounter = generate_encounter(encounter_type);
                        active_encounter.encounter = Some(encounter);

                        // Pause travel, show encounter
                        next_state.set(GameState::Paused);

                        // Reset distance
                        encounter_chance.distance_since_last = 0.0;

                        break;
                    }
                }

                // Reset counter even if no encounter
                encounter_chance.distance_since_last -= 100.0;
            }
        }
    }
}

/// Show encounter UI
pub fn show_encounter_ui(
    mut commands: Commands,
    active_encounter: Res<ActiveEncounter>,
) {
    if let Some(ref encounter) = active_encounter.encounter {
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.95)),
                EncounterUI,
            ))
            .with_children(|parent| {
                parent
                    .spawn(Node {
                        width: Val::Px(700.0),
                        height: Val::Px(500.0),
                        padding: UiRect::all(Val::Px(30.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(20.0),
                        ..default()
                    })
                    .insert(BackgroundColor(Color::srgb(0.12, 0.1, 0.1)))
                    .with_children(|panel| {
                        // Title - color based on encounter type
                        let title_color = if encounter.encounter_type.is_combat() {
                            Color::srgb(1.0, 0.3, 0.3) // Red for combat
                        } else {
                            Color::srgb(1.0, 0.9, 0.5) // Yellow for other
                        };

                        panel.spawn((
                            Text::new(encounter.encounter_type.name()),
                            TextFont {
                                font_size: 36.0,
                                ..default()
                            },
                            TextColor(title_color),
                        ));

                        // Description
                        panel.spawn((
                            Text::new(encounter.encounter_type.description()),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));

                        // Spacer
                        panel.spawn(Node {
                            height: Val::Px(20.0),
                            ..default()
                        });

                        // Choices
                        for (i, choice) in encounter.choices.iter().enumerate() {
                            panel
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        padding: UiRect::all(Val::Px(15.0)),
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.2, 0.18, 0.18)),
                                    EncounterChoiceButton { choice_index: i },
                                ))
                                .with_children(|button| {
                                    button.spawn((
                                        Text::new(&format!("{}. {}", i + 1, choice.text)),
                                        TextFont {
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                                    ));
                                });
                        }

                        // Instructions
                        panel.spawn((
                            Text::new(&format!(
                                "\nPress 1-{} to choose",
                                encounter.choices.len()
                            )),
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

/// Close encounter UI
pub fn close_encounter_ui(
    mut commands: Commands,
    encounter_ui: Query<Entity, With<EncounterUI>>,
) {
    for entity in encounter_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handle encounter choice selection
pub fn handle_encounter_choice(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active_encounter: ResMut<ActiveEncounter>,
    mut inventory: Query<&mut PlayerInventory>,
    mut horse: Query<&mut Horse>,
    mut game_time: ResMut<GameTime>,
    mut taming_state: ResMut<crate::systems::HorseTamingState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(ref encounter) = active_encounter.encounter {
        let keys = [
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
        ];

        for (i, key) in keys.iter().enumerate() {
            if keyboard.just_pressed(*key) && i < encounter.choices.len() {
                let choice = &encounter.choices[i];

                info!("Selected: {}", choice.text);

                // Apply outcome
                match &choice.outcome {
                    EncounterOutcome::Combat { enemies, difficulty } => {
                        info!("Combat started! {} enemies, difficulty {}", enemies, difficulty);
                        // TODO: Transition to combat state
                        // For now, just auto-resolve
                        if let Ok(mut inv) = inventory.get_single_mut() {
                            inv.gold -= 10.0; // Lost some gold in combat
                        }
                    }

                    EncounterOutcome::GainResources { gold, food, water } => {
                        if let Ok(mut inv) = inventory.get_single_mut() {
                            inv.gold += gold;
                            inv.food += food;
                            inv.water += water;
                            info!("Gained: {:.0}g, {:.0} food, {:.0} water", gold, food, water);
                        }
                    }

                    EncounterOutcome::LoseResources { gold, food, water } => {
                        if let Ok(mut inv) = inventory.get_single_mut() {
                            inv.gold = (inv.gold - gold).max(0.0);
                            inv.food = (inv.food - food).max(0.0);
                            inv.water = (inv.water - water).max(0.0);
                            info!("Lost: {:.0}g, {:.0} food, {:.0} water", gold, food, water);
                        }
                    }

                    EncounterOutcome::HorseEffect {
                        health,
                        stamina,
                        morale,
                    } => {
                        if let Ok(mut h) = horse.get_single_mut() {
                            h.health = (h.health + health).max(0.0).min(h.max_health);
                            h.stamina = (h.stamina + stamina).max(0.0).min(h.max_stamina);
                            h.morale = (h.morale + morale).max(0.0).min(100.0);
                            info!("Horse affected: {:.0} health, {:.0} stamina, {:.0} morale", health, stamina, morale);
                        }
                    }

                    EncounterOutcome::TimeDelay { hours } => {
                        game_time.hour += hours;
                        while game_time.hour >= 24.0 {
                            game_time.hour -= 24.0;
                            game_time.day += 1;
                        }
                        info!("Time passed: {:.1} hours", hours);
                    }

                    EncounterOutcome::StartHorseTaming => {
                        // Start horse taming mini-game
                        use rand::Rng;
                        let mut rng = rand::thread_rng();

                        taming_state.active = true;
                        taming_state.attempts_remaining = 5;
                        taming_state.success_threshold = rng.gen_range(8..15);
                        taming_state.current_progress = 0;

                        info!("🐴 Horse taming started! Press SPACE repeatedly to tame the horse!");

                        // Clear encounter but stay in Paused state for taming
                        active_encounter.encounter = None;
                        // State remains Paused for the taming mini-game
                        return; // Don't transition to Traveling yet
                    }

                    EncounterOutcome::Continue => {
                        info!("Continued on your way");
                    }
                }

                // Clear encounter and return to traveling
                active_encounter.encounter = None;
                next_state.set(GameState::Traveling);

                break;
            }
        }
    }
}
