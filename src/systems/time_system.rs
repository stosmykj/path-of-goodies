use bevy::prelude::*;
use crate::resources::GameTime;
use crate::resources::GameSettings;

/// Update game time
pub fn update_game_time(
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
    settings: Res<GameSettings>,
) {
    game_time.update(time.delta_secs(), settings.time_scale);
}

/// Update clear color based on time of day
pub fn update_ambient_lighting(
    game_time: Res<GameTime>,
    mut clear_color: ResMut<ClearColor>,
) {
    // Change background color based on time of day
    let color = match game_time.time_of_day() {
        crate::resources::TimeOfDay::Dawn => Color::srgb(0.4, 0.3, 0.5), // Purple-ish
        crate::resources::TimeOfDay::Day => Color::srgb(0.5, 0.7, 0.9),   // Light blue
        crate::resources::TimeOfDay::Dusk => Color::srgb(0.6, 0.4, 0.3),  // Orange-ish
        crate::resources::TimeOfDay::Night => Color::srgb(0.1, 0.1, 0.2), // Dark blue
    };

    // Smoothly transition to new color
    clear_color.0 = clear_color.0.mix(&color, 0.02);
}
