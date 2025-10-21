use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

/// Marker component for the main HUD container
#[derive(Component)]
pub struct HudRoot;

/// Marker for the resources display (gold, food, water)
#[derive(Component)]
pub struct ResourcesDisplay;

/// Marker for the time display
#[derive(Component)]
pub struct TimeDisplay;

/// Marker for the horse status display
#[derive(Component)]
pub struct HorseStatusDisplay;

/// Marker for the travel info display
#[derive(Component)]
pub struct TravelInfoDisplay;

/// Setup the main HUD
pub fn setup_hud(mut commands: Commands) {
    // Root HUD container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            // Top bar - Resources and Time
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                })
                .with_children(|top_bar| {
                    // Left side - Resources
                    top_bar.spawn((
                        Text::new("Gold: 0 | Food: 0 | Water: 0"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        ResourcesDisplay,
                    ));

                    // Right side - Time
                    top_bar.spawn((
                        Text::new("Day 1 - 08:00"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 0.8)),
                        TimeDisplay,
                    ));
                });

            // Bottom bar - Horse status and travel info
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(80.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                })
                .with_children(|bottom_bar| {
                    // Horse status
                    bottom_bar.spawn((
                        Text::new("Horse: Health 100 | Stamina 100"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 1.0, 0.8)),
                        HorseStatusDisplay,
                    ));

                    // Travel info
                    bottom_bar.spawn((
                        Text::new("Traveling to: None | Progress: 0%"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 1.0)),
                        TravelInfoDisplay,
                    ));
                });
        });
}

/// Update resources display
pub fn update_resources_display(
    inventory_query: Query<&PlayerInventory, Changed<PlayerInventory>>,
    mut text_query: Query<&mut Text, With<ResourcesDisplay>>,
) {
    if let Ok(inventory) = inventory_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            **text = format!(
                "Gold: {:.0} | Food: {:.0} | Water: {:.0}",
                inventory.gold, inventory.food, inventory.water
            );
        }
    }
}

/// Update time display
pub fn update_time_display(
    game_time: Res<GameTime>,
    mut text_query: Query<&mut Text, With<TimeDisplay>>,
) {
    if game_time.is_changed() || true { // Always update for now
        if let Ok(mut text) = text_query.get_single_mut() {
            let hour = game_time.hour as u32;
            let minute = ((game_time.hour % 1.0) * 60.0) as u32;
            **text = format!("Day {} - {:02}:{:02}", game_time.day, hour, minute);
        }
    }
}

/// Update horse status display
pub fn update_horse_status_display(
    horse_query: Query<&Horse, Changed<Horse>>,
    mut text_query: Query<&mut Text, With<HorseStatusDisplay>>,
) {
    if let Ok(horse) = horse_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            **text = format!(
                "Horse: Health {:.0}/{:.0} | Stamina {:.0}/{:.0} | Speed: {:.0}%",
                horse.health,
                horse.max_health,
                horse.stamina,
                horse.max_stamina,
                horse.speed_multiplier * 100.0
            );
        }
    }
}

/// Update travel info display
pub fn update_travel_info_display(
    travel_info: Res<TravelInfo>,
    mut text_query: Query<&mut Text, With<TravelInfoDisplay>>,
) {
    if travel_info.is_changed() || true { // Always update for now
        if let Ok(mut text) = text_query.get_single_mut() {
            if !travel_info.destination.is_empty() {
                **text = format!(
                    "Traveling to: {} | Progress: {:.1}% ({:.0}/{:.0})",
                    travel_info.destination,
                    travel_info.progress_percentage(),
                    travel_info.total_distance - travel_info.distance_remaining,
                    travel_info.total_distance
                );
            } else {
                **text = "No destination set".to_string();
            }
        }
    }
}
