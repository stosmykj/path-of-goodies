use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Town/Village details - extends world map villages with gameplay data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownData {
    pub village_id: usize,
    pub population: u32,
    pub has_inn: bool,
    pub has_market: bool,
    pub has_blacksmith: bool,
    pub has_temple: bool,
    pub food_price: f32,
    pub water_price: f32,
    pub rest_cost: f32,
}

impl TownData {
    pub fn new(village_id: usize, size: crate::world_map::VillageSize) -> Self {
        use crate::world_map::VillageSize;

        match size {
            VillageSize::Hamlet => Self {
                village_id,
                population: 50,
                has_inn: false,
                has_market: true,
                has_blacksmith: false,
                has_temple: false,
                food_price: 2.0,
                water_price: 1.5,
                rest_cost: 5.0,
            },
            VillageSize::Village => Self {
                village_id,
                population: 200,
                has_inn: true,
                has_market: true,
                has_blacksmith: true,
                has_temple: false,
                food_price: 1.5,
                water_price: 1.0,
                rest_cost: 8.0,
            },
            VillageSize::Town => Self {
                village_id,
                population: 800,
                has_inn: true,
                has_market: true,
                has_blacksmith: true,
                has_temple: true,
                food_price: 1.2,
                water_price: 0.8,
                rest_cost: 12.0,
            },
            VillageSize::City => Self {
                village_id,
                population: 2000,
                has_inn: true,
                has_market: true,
                has_blacksmith: true,
                has_temple: true,
                food_price: 1.0,
                water_price: 0.5,
                rest_cost: 15.0,
            },
        }
    }
}

/// Resource holding all town data
#[derive(Resource, Default)]
pub struct TownsDatabase {
    pub towns: std::collections::HashMap<usize, TownData>,
}

impl TownsDatabase {
    pub fn new() -> Self {
        Self {
            towns: std::collections::HashMap::new(),
        }
    }

    pub fn add_town(&mut self, town: TownData) {
        self.towns.insert(town.village_id, town);
    }

    pub fn get_town(&self, village_id: usize) -> Option<&TownData> {
        self.towns.get(&village_id)
    }
}

/// Current town the player is in
#[derive(Resource, Default)]
pub struct CurrentTown {
    pub village_id: Option<usize>,
}

/// UI marker for town view
#[derive(Component)]
pub struct TownUI;

/// Setup town UI when entering a town
pub fn setup_town_ui(
    mut commands: Commands,
    current_town: Res<CurrentTown>,
    towns_db: Res<TownsDatabase>,
    world_map: Res<crate::world_map::WorldMap>,
) {
    if let Some(village_id) = current_town.village_id {
        if let Some(village) = world_map.villages.get(&village_id) {
            if let Some(town_data) = towns_db.get_town(village_id) {
                // Create town UI
                commands
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                        TownUI,
                    ))
                    .with_children(|parent| {
                        // Town panel
                        parent
                            .spawn(Node {
                                width: Val::Px(600.0),
                                height: Val::Px(500.0),
                                padding: UiRect::all(Val::Px(20.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(15.0),
                                ..default()
                            })
                            .insert(BackgroundColor(Color::srgb(0.15, 0.15, 0.2)))
                            .with_children(|panel| {
                                // Town name
                                panel.spawn((
                                    Text::new(&format!("Welcome to {}", village.name)),
                                    TextFont {
                                        font_size: 32.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 0.9, 0.7)),
                                ));

                                // Population
                                panel.spawn((
                                    Text::new(&format!("Population: {}", town_data.population)),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));

                                // Available services
                                let mut services = Vec::new();
                                if town_data.has_inn {
                                    services.push("Inn");
                                }
                                if town_data.has_market {
                                    services.push("Market");
                                }
                                if town_data.has_blacksmith {
                                    services.push("Blacksmith");
                                }
                                if town_data.has_temple {
                                    services.push("Temple");
                                }

                                panel.spawn((
                                    Text::new(&format!("Services: {}", services.join(", "))),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                                ));

                                // Prices
                                panel.spawn((
                                    Text::new(&format!(
                                        "Food: {:.1}g | Water: {:.1}g | Rest: {:.1}g",
                                        town_data.food_price, town_data.water_price, town_data.rest_cost
                                    )),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                ));

                                // Instructions
                                panel.spawn((
                                    Text::new("\nPress ESC to leave town\nPress B to buy supplies\nPress C to view contracts\nPress H to hire guards\nPress G to view your party\nPress K for blacksmith\nPress I for inn\nPress T for temple"),
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

/// Close town UI
pub fn close_town_ui(
    mut commands: Commands,
    town_ui: Query<Entity, With<TownUI>>,
) {
    for entity in town_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handle town interactions
pub fn handle_town_interactions(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut current_town: ResMut<CurrentTown>,
    mut next_state: ResMut<NextState<crate::resources::GameState>>,
    towns_db: Res<TownsDatabase>,
    mut inventory: Query<&mut crate::components::PlayerInventory>,
) {
    if current_town.village_id.is_none() {
        return;
    }

    // Leave town
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Leaving town");
        current_town.village_id = None;
        next_state.set(crate::resources::GameState::Traveling);
        return;
    }

    // Buy supplies
    if keyboard.just_pressed(KeyCode::KeyB) {
        if let Some(village_id) = current_town.village_id {
            if let Some(town_data) = towns_db.get_town(village_id) {
                if let Ok(mut inv) = inventory.get_single_mut() {
                    // Buy 20 food and 20 water
                    let food_cost = 20.0 * town_data.food_price;
                    let water_cost = 20.0 * town_data.water_price;
                    let total_cost = food_cost + water_cost;

                    if inv.gold >= total_cost {
                        inv.gold -= total_cost;
                        inv.food += 20.0;
                        inv.water += 20.0;
                        info!("Bought supplies for {:.1} gold", total_cost);
                    } else {
                        info!("Not enough gold! Need {:.1}g", total_cost);
                    }
                }
            }
        }
    }
}
