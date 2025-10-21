use bevy::prelude::*;

/// Game state enum
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    Traveling,
    Combat,
    Town,
    Camping,
    Paused,
}

/// Global game time resource
#[derive(Resource, Default)]
pub struct GameTime {
    pub day: u32,
    pub hour: f32, // 0.0 - 24.0
    pub elapsed_seconds: f32,
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            day: 1,
            hour: 8.0, // Start at 8 AM
            elapsed_seconds: 0.0,
        }
    }

    pub fn update(&mut self, delta: f32, time_scale: f32) {
        self.elapsed_seconds += delta;

        // Time passes: 1 real second = 1 game minute (configurable)
        let game_minutes = delta * time_scale;
        self.hour += game_minutes / 60.0;

        // Handle day transitions
        while self.hour >= 24.0 {
            self.hour -= 24.0;
            self.day += 1;
        }
    }

    pub fn is_night(&self) -> bool {
        self.hour < 6.0 || self.hour >= 20.0
    }

    pub fn is_day(&self) -> bool {
        !self.is_night()
    }

    pub fn time_of_day(&self) -> TimeOfDay {
        match self.hour {
            h if h >= 6.0 && h < 8.0 => TimeOfDay::Dawn,
            h if h >= 8.0 && h < 18.0 => TimeOfDay::Day,
            h if h >= 18.0 && h < 20.0 => TimeOfDay::Dusk,
            _ => TimeOfDay::Night,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeOfDay {
    Dawn,
    Day,
    Dusk,
    Night,
}

/// Current destination and travel information
#[derive(Resource, Default)]
pub struct TravelInfo {
    pub origin: String,
    pub destination: String,
    pub distance_remaining: f32,
    pub total_distance: f32,
}

impl TravelInfo {
    pub fn progress_percentage(&self) -> f32 {
        if self.total_distance > 0.0 {
            ((self.total_distance - self.distance_remaining) / self.total_distance * 100.0).min(100.0)
        } else {
            0.0
        }
    }
}

/// Settings and configuration
#[derive(Resource)]
pub struct GameSettings {
    pub time_scale: f32, // How fast game time passes
    pub movement_speed_multiplier: f32,
    pub debug_mode: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            time_scale: 60.0, // 1 real second = 1 game minute
            movement_speed_multiplier: 1.0,
            debug_mode: true, // Enable for development
        }
    }
}
