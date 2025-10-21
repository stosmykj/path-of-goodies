use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

/// System to handle wagon movement based on player input
pub fn wagon_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<GameSettings>,
    mut wagon_query: Query<(&mut Transform, &mut TravelState, &Wagon, Option<&Horse>)>,
) {
    for (mut transform, mut travel_state, wagon, horse) in wagon_query.iter_mut() {
        let mut direction = Vec2::ZERO;

        // Check for movement input
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        // Normalize diagonal movement
        if direction.length() > 0.0 {
            direction = direction.normalize();
            travel_state.is_moving = true;
        } else {
            travel_state.is_moving = false;
        }

        // Calculate speed based on horse and wagon stats
        let base_speed = wagon.speed;
        let horse_multiplier = horse.map(|h| h.speed_multiplier).unwrap_or(0.3); // 30% speed if pulling wagon manually
        let final_speed = base_speed * horse_multiplier * settings.movement_speed_multiplier;

        travel_state.current_speed = if travel_state.is_moving { final_speed } else { 0.0 };

        // Apply movement
        let movement = direction * final_speed * time.delta_secs();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}

/// System to make camera follow the wagon smoothly
pub fn camera_follow(
    wagon_query: Query<&Transform, (With<Wagon>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Wagon>)>,
) {
    if let Ok(wagon_transform) = wagon_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Smoothly follow wagon
            let target = wagon_transform.translation;
            camera_transform.translation = camera_transform.translation.lerp(
                target,
                0.1, // Smoothing factor
            );
            // Keep camera Z position fixed
            camera_transform.translation.z = 999.9;
        }
    }
}

/// Update horse stamina based on movement
pub fn update_horse_stamina(
    time: Res<Time>,
    mut horse_query: Query<(&mut Horse, &TravelState), With<Wagon>>,
) {
    for (mut horse, travel_state) in horse_query.iter_mut() {
        if travel_state.is_moving {
            // Stamina drains while moving
            let drain_rate = 2.0; // stamina per second
            horse.stamina = (horse.stamina - drain_rate * time.delta_secs()).max(0.0);

            // Build exhaustion if stamina is low
            if horse.stamina < 30.0 {
                let exhaustion_rate = 5.0;
                horse.exhaustion = (horse.exhaustion + exhaustion_rate * time.delta_secs()).min(100.0);
            }

            // Speed multiplier decreases with exhaustion
            horse.speed_multiplier = 1.0 - (horse.exhaustion / 100.0) * 0.7; // Can drop to 30% speed
        } else {
            // Stamina regenerates when stopped
            let regen_rate = 5.0;
            horse.stamina = (horse.stamina + regen_rate * time.delta_secs()).min(horse.max_stamina);

            // Exhaustion slowly recovers when resting
            let recovery_rate = 1.0;
            horse.exhaustion = (horse.exhaustion - recovery_rate * time.delta_secs()).max(0.0);
        }
    }
}
