use bevy::prelude::*;

/// The player's wagon entity
#[derive(Component)]
pub struct Wagon {
    pub speed: f32,
    pub durability: f32,
    pub max_durability: f32,
    pub cargo_capacity: f32,
    pub current_cargo: f32,
}

impl Default for Wagon {
    fn default() -> Self {
        Self {
            speed: 100.0,
            durability: 100.0,
            max_durability: 100.0,
            cargo_capacity: 500.0,
            current_cargo: 0.0,
        }
    }
}

/// Horse attached to the wagon
#[derive(Component)]
pub struct Horse {
    pub health: f32,
    pub max_health: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub exhaustion: f32,
    pub morale: f32,
    pub speed_multiplier: f32,
}

impl Default for Horse {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            stamina: 100.0,
            max_stamina: 100.0,
            exhaustion: 0.0,
            morale: 100.0,
            speed_multiplier: 1.0,
        }
    }
}

/// Player's resources
#[derive(Component)]
pub struct PlayerInventory {
    pub gold: f32,
    pub food: f32,
    pub water: f32,
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self {
            gold: 100.0,
            food: 50.0,
            water: 50.0,
        }
    }
}

/// Marks the main camera
#[derive(Component)]
pub struct MainCamera;

/// Component for entities that are part of the player's party
#[derive(Component)]
pub struct PartyMember {
    pub name: String,
    pub health: f32,
    pub max_health: f32,
}

/// Velocity component for physics
#[derive(Component, Default)]
pub struct Velocity {
    pub value: Vec2,
}

/// Travel state - whether the wagon is moving or stopped
#[derive(Component, Default)]
pub struct TravelState {
    pub is_moving: bool,
    pub current_speed: f32,
}
