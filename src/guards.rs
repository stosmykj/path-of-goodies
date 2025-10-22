use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Guard type and stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardData {
    pub id: String,
    pub name: String,
    pub guard_type: GuardType,
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
    pub armor: f32,
    pub salary: f32, // Gold per day
    pub level: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GuardType {
    Swordsman,
    Archer,
    Spearman,
    Crossbowman,
}

impl GuardType {
    pub fn name(&self) -> &str {
        match self {
            GuardType::Swordsman => "Swordsman",
            GuardType::Archer => "Archer",
            GuardType::Spearman => "Spearman",
            GuardType::Crossbowman => "Crossbowman",
        }
    }

    pub fn base_health(&self) -> f32 {
        match self {
            GuardType::Swordsman => 100.0,
            GuardType::Archer => 70.0,
            GuardType::Spearman => 90.0,
            GuardType::Crossbowman => 75.0,
        }
    }

    pub fn base_damage(&self) -> f32 {
        match self {
            GuardType::Swordsman => 15.0,
            GuardType::Archer => 20.0,
            GuardType::Spearman => 18.0,
            GuardType::Crossbowman => 25.0,
        }
    }

    pub fn base_armor(&self) -> f32 {
        match self {
            GuardType::Swordsman => 5.0,
            GuardType::Archer => 2.0,
            GuardType::Spearman => 4.0,
            GuardType::Crossbowman => 3.0,
        }
    }

    pub fn base_salary(&self) -> f32 {
        match self {
            GuardType::Swordsman => 5.0,
            GuardType::Archer => 6.0,
            GuardType::Spearman => 5.5,
            GuardType::Crossbowman => 7.0,
        }
    }
}

impl GuardData {
    pub fn new(guard_type: GuardType, level: u32) -> Self {
        let level_multiplier = 1.0 + (level as f32 * 0.2);

        Self {
            id: format!("guard_{}_{}", guard_type.name().to_lowercase(), level),
            name: format!("{} (Lv.{})", guard_type.name(), level),
            guard_type,
            health: guard_type.base_health() * level_multiplier,
            max_health: guard_type.base_health() * level_multiplier,
            damage: guard_type.base_damage() * level_multiplier,
            armor: guard_type.base_armor() + (level as f32 * 0.5),
            salary: guard_type.base_salary() * level_multiplier,
            level,
        }
    }
}

/// Guard component - attached to guard entities
#[derive(Component, Clone)]
pub struct Guard {
    pub data: GuardData,
    pub days_employed: u32,
}

/// Available guards for hire in a town
#[derive(Resource, Default)]
pub struct GuardsForHire {
    pub guards: Vec<GuardData>,
}

impl GuardsForHire {
    pub fn generate_for_town(town_size: crate::world_map::VillageSize) -> Self {
        let mut guards = Vec::new();
        let mut rng = rand::thread_rng();
        use rand::Rng;

        // Number of guards available based on town size
        let count = match town_size {
            crate::world_map::VillageSize::Hamlet => 1,
            crate::world_map::VillageSize::Village => 2,
            crate::world_map::VillageSize::Town => 3,
            crate::world_map::VillageSize::City => 4,
        };

        // Maximum level based on town size
        let max_level = match town_size {
            crate::world_map::VillageSize::Hamlet => 2,
            crate::world_map::VillageSize::Village => 3,
            crate::world_map::VillageSize::Town => 5,
            crate::world_map::VillageSize::City => 7,
        };

        let guard_types = [
            GuardType::Swordsman,
            GuardType::Archer,
            GuardType::Spearman,
            GuardType::Crossbowman,
        ];

        for _ in 0..count {
            let guard_type = guard_types[rng.gen_range(0..guard_types.len())];
            let level = rng.gen_range(1..=max_level);
            guards.push(GuardData::new(guard_type, level));
        }

        Self { guards }
    }
}

/// Player's hired guards
#[derive(Resource, Default)]
pub struct HiredGuards {
    pub guards: Vec<Guard>,
    pub max_guards: usize,
}

impl HiredGuards {
    pub fn new(max_guards: usize) -> Self {
        Self {
            guards: Vec::new(),
            max_guards,
        }
    }

    pub fn can_hire_more(&self) -> bool {
        self.guards.len() < self.max_guards
    }

    pub fn hire(&mut self, guard_data: GuardData) -> bool {
        if self.can_hire_more() {
            self.guards.push(Guard {
                data: guard_data,
                days_employed: 0,
            });
            true
        } else {
            false
        }
    }

    pub fn fire(&mut self, index: usize) -> Option<Guard> {
        if index < self.guards.len() {
            Some(self.guards.remove(index))
        } else {
            None
        }
    }

    pub fn total_daily_cost(&self) -> f32 {
        self.guards.iter().map(|g| g.data.salary).sum()
    }

    pub fn pay_daily_salaries(&mut self, inventory: &mut crate::components::PlayerInventory) -> bool {
        let total_cost = self.total_daily_cost();

        if inventory.gold >= total_cost {
            inventory.gold -= total_cost;

            // Increment days employed
            for guard in &mut self.guards {
                guard.days_employed += 1;
            }

            info!("Paid {:.1}g in guard salaries", total_cost);
            true
        } else {
            warn!("Cannot afford guard salaries! Need {:.1}g", total_cost);
            false
        }
    }
}
