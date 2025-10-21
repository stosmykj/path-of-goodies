use bevy::prelude::*;
use crate::components::*;

/// Whip cooldown tracker
#[derive(Component)]
pub struct WhipCooldown {
    pub remaining: f32, // seconds
    pub duration: f32,  // total cooldown duration
}

impl Default for WhipCooldown {
    fn default() -> Self {
        Self {
            remaining: 0.0,
            duration: 15.0, // 15 second cooldown
        }
    }
}

/// Whip boost active state
#[derive(Component)]
pub struct WhipBoost {
    pub remaining: f32, // seconds
    pub duration: f32,  // boost duration
    pub multiplier: f32, // speed multiplier
}

/// Handle whip mechanic (Space key)
pub fn handle_whip(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut horse_query: Query<(Entity, &mut Horse, Option<&WhipCooldown>, Option<&WhipBoost>)>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for (entity, mut horse, cooldown, boost) in horse_query.iter_mut() {
            // Check if already boosted
            if boost.is_some() {
                info!("Whip boost already active!");
                continue;
            }

            // Check cooldown
            if let Some(cd) = cooldown {
                if cd.remaining > 0.0 {
                    info!("Whip on cooldown: {:.1}s remaining", cd.remaining);
                    continue;
                }
            }

            // Apply whip boost
            info!("🏇 Whip used! Horse speed doubled for 5 seconds!");

            // Damage horse slightly (morale and slight health)
            horse.morale = (horse.morale - 5.0).max(0.0);
            horse.health = (horse.health - 2.0).max(1.0);

            // Add boost component
            commands.entity(entity).insert(WhipBoost {
                remaining: 5.0,
                duration: 5.0,
                multiplier: 2.0, // 2x speed
            });

            // Start cooldown
            if cooldown.is_none() {
                commands.entity(entity).insert(WhipCooldown::default());
            }
        }
    }
}

/// Update whip boost and cooldown timers
pub fn update_whip_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut horse_query: Query<(
        Entity,
        &mut Horse,
        Option<&mut WhipCooldown>,
        Option<&mut WhipBoost>,
    )>,
) {
    for (entity, mut horse, cooldown, boost) in horse_query.iter_mut() {
        // Update boost
        if let Some(mut boost_comp) = boost {
            boost_comp.remaining -= time.delta_secs();

            if boost_comp.remaining <= 0.0 {
                // Boost ended
                info!("Whip boost ended");
                commands.entity(entity).remove::<WhipBoost>();

                // Start cooldown
                if let Some(mut cd) = cooldown {
                    cd.remaining = cd.duration;
                }
            } else {
                // Apply boost to horse speed
                let base_speed = horse.speed_multiplier;
                // Only apply if not already at max from boost
                if base_speed < 1.5 {
                    horse.speed_multiplier = base_speed * boost_comp.multiplier;
                }
            }
        }

        // Update cooldown
        if let Some(mut cd) = cooldown {
            if cd.remaining > 0.0 {
                cd.remaining -= time.delta_secs();
                cd.remaining = cd.remaining.max(0.0);
            }
        }
    }
}

/// Reset speed when boost ends (handled in update_whip_timers via component removal)
/// This is called when exiting Traveling state to clean up
pub fn cleanup_whip_effects(
    mut commands: Commands,
    horse_query: Query<Entity, Or<(With<WhipBoost>, With<WhipCooldown>)>>,
) {
    for entity in horse_query.iter() {
        commands.entity(entity).remove::<WhipBoost>();
        // Keep cooldown between state transitions
    }
}
