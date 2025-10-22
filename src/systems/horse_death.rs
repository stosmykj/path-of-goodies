use bevy::prelude::*;
use crate::components::*;

/// Check if horse dies and handle player pulling wagon
pub fn check_horse_death(
    mut commands: Commands,
    mut horse_query: Query<(Entity, &mut Horse)>,
    mut wagon_query: Query<&mut Wagon>,
) {
    for (entity, mut horse) in horse_query.iter_mut() {
        if horse.health <= 0.0 {
            // Horse died!
            warn!("💀 Your horse has died! You must pull the wagon yourself (70% slower).");

            // Reduce wagon speed to 30% of normal
            if let Ok(mut wagon) = wagon_query.get_single_mut() {
                wagon.speed = wagon.speed * 0.3;
                info!("Wagon speed reduced to 30% - you're now pulling manually");
            }

            // Remove horse component
            commands.entity(entity).remove::<Horse>();

            break;
        }
    }
}

/// Show warning when horse health is low
pub fn warn_low_horse_health(
    horse_query: Query<&Horse, Changed<Horse>>,
) {
    for horse in horse_query.iter() {
        if horse.health <= 20.0 && horse.health > 0.0 {
            warn!("⚠️ WARNING: Horse health critically low ({:.0})! Rest soon or your horse will die!", horse.health);
        } else if horse.health <= 50.0 && horse.health > 20.0 {
            info!("⚠️ Horse health low ({:.0}). Consider resting.", horse.health);
        }
    }
}

/// Component marker for "no horse" state
#[derive(Component)]
pub struct PullingWagonManually;
