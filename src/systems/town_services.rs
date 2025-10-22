use bevy::prelude::*;
use crate::components::*;
use crate::towns::*;
use crate::equipment::*;
use crate::guards::*;

/// Marker for blacksmith UI
#[derive(Component)]
pub struct BlacksmithUI;

/// Marker for inn UI
#[derive(Component)]
pub struct InnUI;

/// Marker for temple UI
#[derive(Component)]
pub struct TempleUI;

/// Show blacksmith UI (press K in town)
pub fn show_blacksmith_ui(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    current_town: Res<CurrentTown>,
    towns_db: Res<TownsDatabase>,
    blacksmith_inv: Res<BlacksmithInventory>,
    existing_ui: Query<Entity, With<BlacksmithUI>>,
) {
    if keyboard.just_pressed(KeyCode::KeyK) {
        if !existing_ui.is_empty() {
            for entity in existing_ui.iter() {
                commands.entity(entity).despawn_recursive();
            }
            return;
        }

        if let Some(village_id) = current_town.village_id {
            if let Some(town_data) = towns_db.get_town(village_id) {
                if !town_data.has_blacksmith {
                    info!("This town has no blacksmith!");
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
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                        BlacksmithUI,
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn(Node {
                                width: Val::Px(700.0),
                                height: Val::Px(600.0),
                                padding: UiRect::all(Val::Px(20.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(12.0),
                                ..default()
                            })
                            .insert(BackgroundColor(Color::srgb(0.15, 0.1, 0.1)))
                            .with_children(|panel| {
                                panel.spawn((
                                    Text::new("🔨 Blacksmith - Equipment Shop"),
                                    TextFont {
                                        font_size: 28.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 0.8, 0.4)),
                                ));

                                for (i, eq) in blacksmith_inv.equipment.iter().enumerate() {
                                    panel
                                        .spawn(Node {
                                            width: Val::Percent(100.0),
                                            padding: UiRect::all(Val::Px(10.0)),
                                            ..default()
                                        })
                                        .insert(BackgroundColor(Color::srgb(0.2, 0.15, 0.15)))
                                        .with_children(|item| {
                                            item.spawn((
                                                Text::new(&format!(
                                                    "{}. {} - {:.0}g (Lv.{})\n  +{:.0} DMG | +{:.0} ARM | +{:.0} HP | +{:.1}% SPD",
                                                    i + 1,
                                                    eq.name,
                                                    eq.price,
                                                    eq.level_required,
                                                    eq.damage_bonus,
                                                    eq.armor_bonus,
                                                    eq.health_bonus,
                                                    eq.speed_bonus * 100.0
                                                )),
                                                TextFont {
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                            ));
                                        });
                                }

                                panel.spawn((
                                    Text::new("\nPress K to close | Press 1-9 to buy\n(Coming soon: Equipment will be equippable)"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.6, 0.8, 1.0)),
                                ));
                            });
                    });
            }
        }
    }
}

/// Show inn UI (press I in town)
pub fn show_inn_ui(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    current_town: Res<CurrentTown>,
    towns_db: Res<TownsDatabase>,
    existing_ui: Query<Entity, With<InnUI>>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        if !existing_ui.is_empty() {
            for entity in existing_ui.iter() {
                commands.entity(entity).despawn_recursive();
            }
            return;
        }

        if let Some(village_id) = current_town.village_id {
            if let Some(town_data) = towns_db.get_town(village_id) {
                if !town_data.has_inn {
                    info!("This town has no inn!");
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
                        InnUI,
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn(Node {
                                width: Val::Px(500.0),
                                height: Val::Px(400.0),
                                padding: UiRect::all(Val::Px(25.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(15.0),
                                justify_content: JustifyContent::Center,
                                ..default()
                            })
                            .insert(BackgroundColor(Color::srgb(0.12, 0.1, 0.08)))
                            .with_children(|panel| {
                                panel.spawn((
                                    Text::new("🍺 The Cozy Inn"),
                                    TextFont {
                                        font_size: 32.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 0.9, 0.6)),
                                ));

                                panel.spawn((
                                    Text::new("Rest for the night and restore your party."),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                ));

                                panel.spawn((
                                    Text::new(&format!("Cost: {:.1}g per person", town_data.rest_cost)),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 0.9, 0.5)),
                                ));

                                panel.spawn((
                                    Text::new("Benefits:\n• Full health restore\n• Full stamina restore\n• Skip to next day"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.7, 1.0, 0.7)),
                                ));

                                panel.spawn((
                                    Text::new("\nPress R to rest | Press I to close"),
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
}

/// Handle inn rest
pub fn handle_inn_rest(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inventory: Query<&mut PlayerInventory>,
    mut horse: Query<&mut Horse>,
    mut hired_guards: ResMut<HiredGuards>,
    mut game_time: ResMut<crate::resources::GameTime>,
    current_town: Res<CurrentTown>,
    towns_db: Res<TownsDatabase>,
    inn_ui: Query<Entity, With<InnUI>>,
    mut commands: Commands,
) {
    if !inn_ui.is_empty() && keyboard.just_pressed(KeyCode::KeyR) {
        if let Some(village_id) = current_town.village_id {
            if let Some(town_data) = towns_db.get_town(village_id) {
                let party_size = 1 + hired_guards.guards.len();
                let total_cost = town_data.rest_cost * party_size as f32;

                if let Ok(mut inv) = inventory.get_single_mut() {
                    if inv.gold >= total_cost {
                        inv.gold -= total_cost;

                        // Restore horse
                        if let Ok(mut h) = horse.get_single_mut() {
                            h.health = h.max_health;
                            h.stamina = h.max_stamina;
                            h.exhaustion = 0.0;
                            h.morale = 100.0;
                        }

                        // Restore guards
                        for guard in &mut hired_guards.guards {
                            guard.data.health = guard.data.max_health;
                        }

                        // Advance time to next day
                        game_time.day += 1;
                        game_time.hour = 8.0; // Wake up at 8 AM

                        info!("💤 Rested at the inn. Party fully restored! Cost: {:.1}g", total_cost);

                        // Close UI
                        for entity in inn_ui.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                    } else {
                        info!("Not enough gold! Need {:.1}g", total_cost);
                    }
                }
            }
        }
    }
}

/// Show temple UI (press T in town)
pub fn show_temple_ui(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    current_town: Res<CurrentTown>,
    towns_db: Res<TownsDatabase>,
    existing_ui: Query<Entity, With<TempleUI>>,
) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        if !existing_ui.is_empty() {
            for entity in existing_ui.iter() {
                commands.entity(entity).despawn_recursive();
            }
            return;
        }

        if let Some(village_id) = current_town.village_id {
            if let Some(town_data) = towns_db.get_town(village_id) {
                if !town_data.has_temple {
                    info!("This town has no temple!");
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
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                        TempleUI,
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn(Node {
                                width: Val::Px(550.0),
                                height: Val::Px(450.0),
                                padding: UiRect::all(Val::Px(30.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(18.0),
                                justify_content: JustifyContent::Center,
                                ..default()
                            })
                            .insert(BackgroundColor(Color::srgb(0.08, 0.1, 0.15)))
                            .with_children(|panel| {
                                panel.spawn((
                                    Text::new("⛪ Temple of Healing"),
                                    TextFont {
                                        font_size: 32.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 1.0)),
                                ));

                                panel.spawn((
                                    Text::new("Seek the blessings of the divine."),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.8, 0.8, 0.9)),
                                ));

                                // Services
                                panel.spawn((
                                    Text::new("1. Full Healing (30g) - Restore full HP to all"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.7, 1.0, 0.7)),
                                ));

                                panel.spawn((
                                    Text::new("2. Blessing (50g) - +20 max HP to party"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.7, 0.7, 1.0)),
                                ));

                                panel.spawn((
                                    Text::new("3. Horse Blessing (40g) - +20 HP, +10 morale"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 1.0, 0.7)),
                                ));

                                panel.spawn((
                                    Text::new("\nPress T to close | Press 1-3 to purchase service"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.6, 0.8, 1.0)),
                                ));
                            });
                    });
            }
        }
    }
}

/// Handle blacksmith equipment purchase
pub fn handle_blacksmith_purchase(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inventory: Query<&mut PlayerInventory>,
    mut blacksmith_inv: ResMut<BlacksmithInventory>,
    blacksmith_ui: Query<Entity, With<BlacksmithUI>>,
    mut commands: Commands,
) {
    if !blacksmith_ui.is_empty() {
        let keys = [
            KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3,
            KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6,
            KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9,
        ];

        for (i, key) in keys.iter().enumerate() {
            if keyboard.just_pressed(*key) && i < blacksmith_inv.equipment.len() {
                if let Ok(mut inv) = inventory.get_single_mut() {
                    let eq = &blacksmith_inv.equipment[i];

                    if inv.gold >= eq.price {
                        inv.gold -= eq.price;
                        info!("💰 Purchased {} for {:.0}g! (+{:.0} DMG, +{:.0} ARM, +{:.0} HP, +{:.1}% SPD)",
                            eq.name, eq.price, eq.damage_bonus, eq.armor_bonus, eq.health_bonus, eq.speed_bonus * 100.0);

                        // Remove purchased item
                        blacksmith_inv.equipment.remove(i);

                        // TODO: Add equipment to player/guard inventory when equipment system is implemented
                    } else {
                        info!("❌ Not enough gold! Need {:.0}g (have {:.0}g)", eq.price, inv.gold);
                    }
                }
                break;
            }
        }
    }
}

/// Handle temple service selection
pub fn handle_temple_service(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inventory: Query<&mut PlayerInventory>,
    mut horse: Query<&mut Horse>,
    mut hired_guards: ResMut<HiredGuards>,
    temple_ui: Query<Entity, With<TempleUI>>,
    mut commands: Commands,
) {
    if !temple_ui.is_empty() {
        // Service 1: Full heal (30g)
        if keyboard.just_pressed(KeyCode::Digit1) {
            if let Ok(mut inv) = inventory.get_single_mut() {
                if inv.gold >= 30.0 {
                    inv.gold -= 30.0;

                    // Heal horse
                    if let Ok(mut h) = horse.get_single_mut() {
                        h.health = h.max_health;
                    }

                    // Heal all guards
                    for guard in &mut hired_guards.guards {
                        guard.data.health = guard.data.max_health;
                    }

                    info!("✨ Full healing received! Everyone restored to full health. (-30g)");
                } else {
                    info!("❌ Not enough gold! Need 30g");
                }
            }
        }

        // Service 2: Blessing (50g) - increase max health
        if keyboard.just_pressed(KeyCode::Digit2) {
            if let Ok(mut inv) = inventory.get_single_mut() {
                if inv.gold >= 50.0 {
                    inv.gold -= 50.0;

                    // Bless all guards
                    for guard in &mut hired_guards.guards {
                        guard.data.max_health += 20.0;
                        guard.data.health = guard.data.max_health;
                    }

                    info!("🙏 Blessing received! All party members gained +20 max health. (-50g)");
                } else {
                    info!("❌ Not enough gold! Need 50g");
                }
            }
        }

        // Service 3: Horse blessing (40g)
        if keyboard.just_pressed(KeyCode::Digit3) {
            if let Ok(mut inv) = inventory.get_single_mut() {
                if inv.gold >= 40.0 {
                    inv.gold -= 40.0;

                    if let Ok(mut h) = horse.get_single_mut() {
                        h.max_health += 20.0;
                        h.health = h.max_health;
                        h.morale = (h.morale + 10.0).min(100.0);
                    }

                    info!("🐴 Horse blessed! +20 max health and +10 morale. (-40g)");
                } else {
                    info!("❌ Not enough gold! Need 40g");
                }
            }
        }
    }
}

/// Close service UIs
pub fn close_service_uis(
    mut commands: Commands,
    blacksmith_ui: Query<Entity, With<BlacksmithUI>>,
    inn_ui: Query<Entity, With<InnUI>>,
    temple_ui: Query<Entity, With<TempleUI>>,
) {
    for entity in blacksmith_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in inn_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in temple_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Generate blacksmith inventory when entering town
pub fn generate_blacksmith_inventory(
    mut blacksmith_inv: ResMut<BlacksmithInventory>,
    current_town: Res<CurrentTown>,
    world_map: Res<crate::world_map::WorldMap>,
) {
    if let Some(village_id) = current_town.village_id {
        if let Some(village) = world_map.villages.get(&village_id) {
            *blacksmith_inv = BlacksmithInventory::generate_for_town(village.size);
        }
    }
}
