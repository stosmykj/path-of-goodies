use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Contract for delivering goods between villages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: String,
    pub name: String,
    pub description: String,
    pub origin_village: usize,
    pub destination_village: usize,
    pub reward_gold: f32,
    pub cargo_weight: f32,
    pub time_limit_days: Option<u32>, // None = no time limit
    pub difficulty: ContractDifficulty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractDifficulty {
    Easy,
    Medium,
    Hard,
    Extreme,
}

impl ContractDifficulty {
    pub fn color(&self) -> Color {
        match self {
            ContractDifficulty::Easy => Color::srgb(0.5, 1.0, 0.5),
            ContractDifficulty::Medium => Color::srgb(1.0, 1.0, 0.5),
            ContractDifficulty::Hard => Color::srgb(1.0, 0.6, 0.3),
            ContractDifficulty::Extreme => Color::srgb(1.0, 0.3, 0.3),
        }
    }
}

/// Active contract component
#[derive(Component, Debug, Clone)]
pub struct ActiveContract {
    pub contract: Contract,
    pub accepted_day: u32,
    pub progress: f32, // 0.0 to 1.0
}

/// Available contracts at a village
#[derive(Resource, Default)]
pub struct AvailableContracts {
    pub contracts: Vec<Contract>,
}

impl AvailableContracts {
    pub fn new() -> Self {
        Self {
            contracts: Vec::new(),
        }
    }

    pub fn add_contract(&mut self, contract: Contract) {
        self.contracts.push(contract);
    }

    pub fn get_contracts_for_village(&self, village_id: usize) -> Vec<&Contract> {
        self.contracts
            .iter()
            .filter(|c| c.origin_village == village_id)
            .collect()
    }

    pub fn remove_contract(&mut self, contract_id: &str) -> Option<Contract> {
        if let Some(index) = self.contracts.iter().position(|c| c.id == contract_id) {
            Some(self.contracts.remove(index))
        } else {
            None
        }
    }
}

/// Generate contracts for villages
pub fn generate_contracts_for_village(
    village_id: usize,
    village_name: &str,
    connected_villages: &[(usize, String, f32)], // (id, name, distance)
) -> Vec<Contract> {
    let mut contracts = Vec::new();
    let mut rng = rand::thread_rng();

    use rand::Rng;

    // Generate 2-4 contracts per village
    let num_contracts = rng.gen_range(2..=4);

    for i in 0..num_contracts {
        if connected_villages.is_empty() {
            continue;
        }

        let destination_idx = rng.gen_range(0..connected_villages.len());
        let (dest_id, dest_name, distance) = &connected_villages[destination_idx];

        let goods = [
            "Wheat", "Wine", "Cloth", "Iron", "Tools", "Spices", "Leather",
            "Pottery", "Medicine", "Books", "Salt", "Furs",
        ];
        let good = goods[rng.gen_range(0..goods.len())];

        // Calculate difficulty based on distance
        let difficulty = if *distance < 300.0 {
            ContractDifficulty::Easy
        } else if *distance < 600.0 {
            ContractDifficulty::Medium
        } else if *distance < 900.0 {
            ContractDifficulty::Hard
        } else {
            ContractDifficulty::Extreme
        };

        // Calculate reward based on distance and difficulty
        let base_reward = distance * 0.5;
        let difficulty_multiplier = match difficulty {
            ContractDifficulty::Easy => 1.0,
            ContractDifficulty::Medium => 1.5,
            ContractDifficulty::Hard => 2.0,
            ContractDifficulty::Extreme => 3.0,
        };

        let reward = base_reward * difficulty_multiplier;
        let cargo_weight = rng.gen_range(50.0..200.0);

        let contract = Contract {
            id: format!("{}_contract_{}", village_name.to_lowercase(), i),
            name: format!("Deliver {} to {}", good, dest_name),
            description: format!(
                "Transport {} from {} to {}. Distance: {:.0} units.",
                good, village_name, dest_name, distance
            ),
            origin_village: village_id,
            destination_village: *dest_id,
            reward_gold: reward,
            cargo_weight,
            time_limit_days: Some(((distance / 100.0) as u32).max(3)), // ~1 day per 100 units
            difficulty,
        };

        contracts.push(contract);
    }

    contracts
}
