use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

/// Marker for camping UI
#[derive(Component)]
pub struct CampingUI;

/// Toggle camping
pub fn toggle_camping(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        match current_state.get() {
            GameState::Traveling => {
                // Enter camping
                next_state.set(GameState::Camping);
                info!("Setting up camp...");
            }
            GameState::Camping => {
                // Leave camp
                next_state.set(GameState::Traveling);
                info!("Breaking camp, continuing journey...");
            }
            _ => {}
        }
    }
}

/// Setup camping UI
pub fn setup_camping_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.1, 0.9)),
            CampingUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Px(500.0),
                    height: Val::Px(400.0),
                    padding: UiRect::all(Val::Px(30.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .insert(BackgroundColor(Color::srgb(0.1, 0.1, 0.12)))
                .with_children(|panel| {
                    // Title
                    panel.spawn((
                        Text::new("🔥 Camp for the Night 🔥"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.7, 0.3)),
                    ));

                    // Description
                    panel.spawn((
                        Text::new("You've set up camp to rest for the night."),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));

                    // Resource consumption warning
                    panel.spawn((
                        Text::new("Consuming: 10 food and 15 water per party member"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.8, 0.4)),
                    ));

                    // Benefits
                    panel.spawn((
                        Text::new("Benefits:\n• Horse stamina fully restored\n• Party health restored\n• Game saved"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 1.0, 0.7)),
                    ));

                    // Instructions
                    panel.spawn((
                        Text::new("\nPress C to continue journey"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.8, 1.0)),
                    ));
                });
        });

    info!("Camping UI displayed");
}

/// Close camping UI
pub fn close_camping_ui(mut commands: Commands, camping_ui: Query<Entity, With<CampingUI>>) {
    for entity in camping_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Rest at camp - restore horse and consume resources
pub fn rest_at_camp(
    mut horse_query: Query<&mut Horse>,
    mut inventory_query: Query<&mut PlayerInventory>,
) {
    // Restore horse stamina and reduce exhaustion
    for mut horse in horse_query.iter_mut() {
        horse.stamina = horse.max_stamina;
        horse.exhaustion = 0.0;
        horse.speed_multiplier = 1.0;
        info!("Horse fully rested");
    }

    // Consume resources (per design: 10 food, 15 water per party member)
    if let Ok(mut inv) = inventory_query.get_single_mut() {
        let party_size = 1.0; // TODO: Count actual party members
        let food_cost = 10.0 * party_size;
        let water_cost = 15.0 * party_size;

        inv.food = (inv.food - food_cost).max(0.0);
        inv.water = (inv.water - water_cost).max(0.0);

        info!(
            "Consumed {:.0} food and {:.0} water while camping",
            food_cost, water_cost
        );

        if inv.food <= 0.0 {
            warn!("⚠️ Out of food!");
        }
        if inv.water <= 0.0 {
            warn!("⚠️ Out of water!");
        }
    }
}
