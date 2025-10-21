use bevy::prelude::*;
use crate::contracts::*;
use crate::components::*;
use crate::resources::*;
use crate::world_map::WorldMap;
use crate::towns::CurrentTown;

/// Marker for contract UI
#[derive(Component)]
pub struct ContractUI;

/// Marker for contract list items
#[derive(Component)]
pub struct ContractListItem {
    pub contract_id: String,
}

/// Show contracts UI
pub fn show_contracts_ui(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    current_town: Res<CurrentTown>,
    contracts: Res<AvailableContracts>,
    world_map: Res<WorldMap>,
    existing_ui: Query<Entity, With<ContractUI>>,
) {
    // Toggle contracts UI with C key (only in town)
    if keyboard.just_pressed(KeyCode::KeyC) {
        if !existing_ui.is_empty() {
            // Close if already open
            for entity in existing_ui.iter() {
                commands.entity(entity).despawn_recursive();
            }
            return;
        }

        if let Some(village_id) = current_town.village_id {
            let available = contracts.get_contracts_for_village(village_id);

            // Create contracts UI
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
                    ContractUI,
                ))
                .with_children(|parent| {
                    // Contract panel
                    parent
                        .spawn(Node {
                            width: Val::Px(700.0),
                            height: Val::Px(600.0),
                            padding: UiRect::all(Val::Px(20.0)),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(15.0),
                            ..default()
                        })
                        .insert(BackgroundColor(Color::srgb(0.1, 0.1, 0.15)))
                        .with_children(|panel| {
                            // Title
                            panel.spawn((
                                Text::new("Available Contracts"),
                                TextFont {
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.9, 0.6)),
                            ));

                            // Contract list
                            if available.is_empty() {
                                panel.spawn((
                                    Text::new("No contracts available at this village."),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                ));
                            } else {
                                for (i, contract) in available.iter().enumerate() {
                                    let dest_village = world_map.villages.get(&contract.destination_village);
                                    let dest_name = dest_village.map(|v| v.name.as_str()).unwrap_or("Unknown");

                                    // Contract item
                                    panel
                                        .spawn(Node {
                                            width: Val::Percent(100.0),
                                            padding: UiRect::all(Val::Px(10.0)),
                                            flex_direction: FlexDirection::Column,
                                            row_gap: Val::Px(5.0),
                                            ..default()
                                        })
                                        .insert(BackgroundColor(Color::srgb(0.15, 0.15, 0.2)))
                                        .insert(ContractListItem {
                                            contract_id: contract.id.clone(),
                                        })
                                        .with_children(|item| {
                                            // Contract name with difficulty color
                                            item.spawn((
                                                Text::new(&format!("{}. {}", i + 1, contract.name)),
                                                TextFont {
                                                    font_size: 20.0,
                                                    ..default()
                                                },
                                                TextColor(contract.difficulty.color()),
                                            ));

                                            // Description
                                            item.spawn((
                                                Text::new(&contract.description),
                                                TextFont {
                                                    font_size: 14.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                            ));

                                            // Reward and stats
                                            let time_limit_str = contract
                                                .time_limit_days
                                                .map(|d| format!("{} days", d))
                                                .unwrap_or_else(|| "No limit".to_string());

                                            item.spawn((
                                                Text::new(&format!(
                                                    "Reward: {:.0}g | Cargo: {:.0}kg | Time: {}",
                                                    contract.reward_gold, contract.cargo_weight, time_limit_str
                                                )),
                                                TextFont {
                                                    font_size: 14.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(1.0, 0.9, 0.5)),
                                            ));

                                            // Instructions
                                            item.spawn((
                                                Text::new(&format!("Press {} to accept", i + 1)),
                                                TextFont {
                                                    font_size: 12.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.6, 0.8, 1.0)),
                                            ));
                                        });
                                }
                            }

                            // Instructions at bottom
                            panel.spawn((
                                Text::new("\nPress C to close | Press 1-9 to accept contract"),
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

/// Handle contract acceptance
pub fn handle_contract_acceptance(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    current_town: Res<CurrentTown>,
    mut contracts: ResMut<AvailableContracts>,
    game_time: Res<GameTime>,
    wagon_query: Query<Entity, With<Wagon>>,
    contract_ui: Query<Entity, With<ContractUI>>,
) {
    // Check for number keys 1-9
    let keys = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];

    for (i, key) in keys.iter().enumerate() {
        if keyboard.just_pressed(*key) {
            if let Some(village_id) = current_town.village_id {
                let available = contracts.get_contracts_for_village(village_id);

                if let Some(contract) = available.get(i) {
                    let contract_id = contract.id.clone();

                    // Remove from available contracts
                    if let Some(contract) = contracts.remove_contract(&contract_id) {
                        info!("Accepted contract: {}", contract.name);

                        // Add active contract component to wagon
                        if let Ok(wagon_entity) = wagon_query.get_single() {
                            commands.entity(wagon_entity).insert(ActiveContract {
                                contract: contract.clone(),
                                accepted_day: game_time.day,
                                progress: 0.0,
                            });

                            info!(
                                "Deliver cargo to {} for {:.0} gold!",
                                contract.name, contract.reward_gold
                            );
                        }

                        // Close contracts UI
                        for entity in contract_ui.iter() {
                            commands.entity(entity).despawn_recursive();
                        }

                        break;
                    }
                }
            }
        }
    }
}

/// Check for contract completion
pub fn check_contract_completion(
    mut commands: Commands,
    current_town: Res<CurrentTown>,
    mut wagon_query: Query<(Entity, &mut PlayerInventory, &ActiveContract), With<Wagon>>,
) {
    if let Some(village_id) = current_town.village_id {
        if let Ok((entity, mut inventory, active_contract)) = wagon_query.get_single_mut() {
            // Check if we arrived at the destination
            if active_contract.contract.destination_village == village_id {
                // Contract completed!
                let reward = active_contract.contract.reward_gold;
                inventory.gold += reward;

                info!(
                    "✅ Contract completed: {} | Reward: {:.0}g",
                    active_contract.contract.name, reward
                );

                // Remove active contract
                commands.entity(entity).remove::<ActiveContract>();
            }
        }
    }
}
