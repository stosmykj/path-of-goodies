use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Types of random encounters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncounterType {
    // Combat encounters
    Bandits,
    WildAnimals,
    HostileMerchants,

    // Friendly encounters
    TravelingMerchant,
    Pilgrims,
    WoundedTraveler,

    // Environmental
    BadWeather,
    BrokenWheel,
    RiverCrossing,

    // Opportunities
    AbandonedCamp,
    WildHorses,
    TreasureCache,

    // Neutral
    Checkpoint,
    RestingCaravan,
    StrangeOmen,
}

impl EncounterType {
    pub fn chance(&self) -> f32 {
        match self {
            // Combat - less common but significant
            EncounterType::Bandits => 0.10,
            EncounterType::WildAnimals => 0.12,
            EncounterType::HostileMerchants => 0.05,

            // Friendly - fairly common
            EncounterType::TravelingMerchant => 0.15,
            EncounterType::Pilgrims => 0.10,
            EncounterType::WoundedTraveler => 0.08,

            // Environmental - common
            EncounterType::BadWeather => 0.15,
            EncounterType::BrokenWheel => 0.08,
            EncounterType::RiverCrossing => 0.07,

            // Opportunities - rare
            EncounterType::AbandonedCamp => 0.06,
            EncounterType::WildHorses => 0.04,
            EncounterType::TreasureCache => 0.03,

            // Neutral - moderate
            EncounterType::Checkpoint => 0.08,
            EncounterType::RestingCaravan => 0.10,
            EncounterType::StrangeOmen => 0.05,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            EncounterType::Bandits => "Bandit Ambush!",
            EncounterType::WildAnimals => "Wild Animal Attack!",
            EncounterType::HostileMerchants => "Hostile Merchants",
            EncounterType::TravelingMerchant => "Traveling Merchant",
            EncounterType::Pilgrims => "Group of Pilgrims",
            EncounterType::WoundedTraveler => "Wounded Traveler",
            EncounterType::BadWeather => "Bad Weather",
            EncounterType::BrokenWheel => "Broken Wheel!",
            EncounterType::RiverCrossing => "River Crossing",
            EncounterType::AbandonedCamp => "Abandoned Camp",
            EncounterType::WildHorses => "Wild Horses",
            EncounterType::TreasureCache => "Hidden Cache",
            EncounterType::Checkpoint => "Guard Checkpoint",
            EncounterType::RestingCaravan => "Resting Caravan",
            EncounterType::StrangeOmen => "Strange Omen",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            EncounterType::Bandits => "A group of bandits blocks your path, demanding your cargo!",
            EncounterType::WildAnimals => "Wolves surround your wagon, hungry and aggressive!",
            EncounterType::HostileMerchants => "Rival merchants try to sabotage your delivery.",
            EncounterType::TravelingMerchant => "A friendly merchant offers to trade goods.",
            EncounterType::Pilgrims => "Pilgrims on their way to a temple share their food.",
            EncounterType::WoundedTraveler => "A wounded traveler needs help. Aid them?",
            EncounterType::BadWeather => "Heavy rain slows your progress significantly.",
            EncounterType::BrokenWheel => "One of your wagon wheels breaks! Repairs needed.",
            EncounterType::RiverCrossing => "A river blocks your path. Find a safe crossing.",
            EncounterType::AbandonedCamp => "You find an abandoned camp with supplies.",
            EncounterType::WildHorses => "A herd of wild horses runs nearby. Try to tame one?",
            EncounterType::TreasureCache => "You stumble upon a hidden stash of goods!",
            EncounterType::Checkpoint => "Guards inspect your wagon. Pay the toll or talk your way through?",
            EncounterType::RestingCaravan => "A caravan rests nearby. They offer shelter for the night.",
            EncounterType::StrangeOmen => "Dark clouds and eerie sounds fill the air...",
        }
    }

    pub fn is_combat(&self) -> bool {
        matches!(
            self,
            EncounterType::Bandits | EncounterType::WildAnimals | EncounterType::HostileMerchants
        )
    }
}

/// Active encounter
#[derive(Debug, Clone)]
pub struct Encounter {
    pub encounter_type: EncounterType,
    pub choices: Vec<EncounterChoice>,
}

/// Player choices during encounter
#[derive(Debug, Clone)]
pub struct EncounterChoice {
    pub text: String,
    pub outcome: EncounterOutcome,
}

/// Outcome of an encounter choice
#[derive(Debug, Clone)]
pub enum EncounterOutcome {
    Combat { enemies: u32, difficulty: f32 },
    GainResources { gold: f32, food: f32, water: f32 },
    LoseResources { gold: f32, food: f32, water: f32 },
    HorseEffect { health: f32, stamina: f32, morale: f32 },
    TimeDelay { hours: f32 },
    StartHorseTaming,
    Continue,
}

/// Current active encounter resource
#[derive(Resource, Default)]
pub struct ActiveEncounter {
    pub encounter: Option<Encounter>,
}

/// Encounter chance tracker
#[derive(Resource)]
pub struct EncounterChance {
    pub distance_since_last: f32,
    pub base_chance_per_100_units: f32,
}

impl Default for EncounterChance {
    fn default() -> Self {
        Self {
            distance_since_last: 0.0,
            base_chance_per_100_units: 0.3, // 30% chance per 100 units traveled
        }
    }
}

/// Generate a random encounter
pub fn generate_encounter(encounter_type: EncounterType) -> Encounter {
    let choices = match encounter_type {
        EncounterType::Bandits => vec![
            EncounterChoice {
                text: "Fight the bandits!".to_string(),
                outcome: EncounterOutcome::Combat {
                    enemies: 3,
                    difficulty: 1.2,
                },
            },
            EncounterChoice {
                text: "Give them 20 gold to pass".to_string(),
                outcome: EncounterOutcome::LoseResources {
                    gold: 20.0,
                    food: 0.0,
                    water: 0.0,
                },
            },
            EncounterChoice {
                text: "Try to flee (whip horse)".to_string(),
                outcome: EncounterOutcome::HorseEffect {
                    health: -10.0,
                    stamina: -30.0,
                    morale: -15.0,
                },
            },
        ],

        EncounterType::TravelingMerchant => vec![
            EncounterChoice {
                text: "Buy supplies (30g for 15 food, 15 water)".to_string(),
                outcome: EncounterOutcome::LoseResources {
                    gold: 30.0,
                    food: -15.0,
                    water: -15.0,
                },
            },
            EncounterChoice {
                text: "Sell goods (gain 20g)".to_string(),
                outcome: EncounterOutcome::GainResources {
                    gold: 20.0,
                    food: 0.0,
                    water: 0.0,
                },
            },
            EncounterChoice {
                text: "Just pass by".to_string(),
                outcome: EncounterOutcome::Continue,
            },
        ],

        EncounterType::WoundedTraveler => vec![
            EncounterChoice {
                text: "Help them (use 5 food, 5 water)".to_string(),
                outcome: EncounterOutcome::LoseResources {
                    gold: -15.0, // They give you gold as thanks
                    food: 5.0,
                    water: 5.0,
                },
            },
            EncounterChoice {
                text: "Leave them be".to_string(),
                outcome: EncounterOutcome::Continue,
            },
        ],

        EncounterType::BrokenWheel => vec![
            EncounterChoice {
                text: "Repair it yourself (takes 2 hours)".to_string(),
                outcome: EncounterOutcome::TimeDelay { hours: 2.0 },
            },
            EncounterChoice {
                text: "Pay for quick fix (15g, 30 min)".to_string(),
                outcome: EncounterOutcome::LoseResources {
                    gold: 15.0,
                    food: 0.0,
                    water: 0.0,
                },
            },
        ],

        EncounterType::AbandonedCamp => vec![
            EncounterChoice {
                text: "Search the camp".to_string(),
                outcome: EncounterOutcome::GainResources {
                    gold: 10.0,
                    food: 8.0,
                    water: 12.0,
                },
            },
            EncounterChoice {
                text: "Leave it alone".to_string(),
                outcome: EncounterOutcome::Continue,
            },
        ],

        EncounterType::WildHorses => vec![
            EncounterChoice {
                text: "Try to tame one (mini-game)".to_string(),
                outcome: EncounterOutcome::StartHorseTaming,
            },
            EncounterChoice {
                text: "Just watch them pass".to_string(),
                outcome: EncounterOutcome::Continue,
            },
        ],

        EncounterType::BadWeather => vec![
            EncounterChoice {
                text: "Push through (tires horse)".to_string(),
                outcome: EncounterOutcome::HorseEffect {
                    health: -5.0,
                    stamina: -20.0,
                    morale: -10.0,
                },
            },
            EncounterChoice {
                text: "Wait it out (2 hours)".to_string(),
                outcome: EncounterOutcome::TimeDelay { hours: 2.0 },
            },
        ],

        EncounterType::Pilgrims => vec![
            EncounterChoice {
                text: "Share a meal with them".to_string(),
                outcome: EncounterOutcome::LoseResources {
                    gold: 0.0,
                    food: 3.0,
                    water: -5.0, // They give you water
                },
            },
            EncounterChoice {
                text: "Continue on your way".to_string(),
                outcome: EncounterOutcome::Continue,
            },
        ],

        EncounterType::TreasureCache => vec![
            EncounterChoice {
                text: "Take the treasure!".to_string(),
                outcome: EncounterOutcome::GainResources {
                    gold: 50.0,
                    food: 5.0,
                    water: 5.0,
                },
            },
        ],

        _ => vec![EncounterChoice {
            text: "Continue".to_string(),
            outcome: EncounterOutcome::Continue,
        }],
    };

    Encounter {
        encounter_type,
        choices,
    }
}

/// Roll for random encounter
pub fn roll_encounter() -> Option<EncounterType> {
    let mut rng = rand::thread_rng();

    // List all encounter types with their chances
    let encounters = [
        EncounterType::Bandits,
        EncounterType::WildAnimals,
        EncounterType::TravelingMerchant,
        EncounterType::Pilgrims,
        EncounterType::WoundedTraveler,
        EncounterType::BadWeather,
        EncounterType::BrokenWheel,
        EncounterType::AbandonedCamp,
        EncounterType::WildHorses,
        EncounterType::TreasureCache,
    ];

    // Calculate total chance
    let total_chance: f32 = encounters.iter().map(|e| e.chance()).sum();

    // Roll
    let roll: f32 = rng.gen_range(0.0..total_chance);

    // Find which encounter was rolled
    let mut accumulated = 0.0;
    for encounter_type in encounters.iter() {
        accumulated += encounter_type.chance();
        if roll < accumulated {
            return Some(*encounter_type);
        }
    }

    None
}
