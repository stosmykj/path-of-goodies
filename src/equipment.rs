use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Equipment slot types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Weapon,
    Armor,
    Boots,
    Helmet,
    Accessory,
}

/// Equipment item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub id: String,
    pub name: String,
    pub slot: EquipmentSlot,
    pub health_bonus: f32,
    pub damage_bonus: f32,
    pub armor_bonus: f32,
    pub speed_bonus: f32,
    pub price: f32,
    pub level_required: u32,
}

impl Equipment {
    pub fn new_weapon(name: &str, damage: f32, price: f32, level: u32) -> Self {
        Self {
            id: name.to_lowercase().replace(" ", "_"),
            name: name.to_string(),
            slot: EquipmentSlot::Weapon,
            health_bonus: 0.0,
            damage_bonus: damage,
            armor_bonus: 0.0,
            speed_bonus: 0.0,
            price,
            level_required: level,
        }
    }

    pub fn new_armor(name: &str, armor: f32, health: f32, price: f32, level: u32) -> Self {
        Self {
            id: name.to_lowercase().replace(" ", "_"),
            name: name.to_string(),
            slot: EquipmentSlot::Armor,
            health_bonus: health,
            damage_bonus: 0.0,
            armor_bonus: armor,
            speed_bonus: 0.0,
            price,
            level_required: level,
        }
    }

    pub fn new_boots(name: &str, speed: f32, armor: f32, price: f32, level: u32) -> Self {
        Self {
            id: name.to_lowercase().replace(" ", "_"),
            name: name.to_string(),
            slot: EquipmentSlot::Boots,
            health_bonus: 0.0,
            damage_bonus: 0.0,
            armor_bonus: armor,
            speed_bonus: speed,
            price,
            level_required: level,
        }
    }
}

/// Equipment inventory component
#[derive(Component, Clone, Default)]
pub struct EquipmentInventory {
    pub weapon: Option<Equipment>,
    pub armor: Option<Equipment>,
    pub boots: Option<Equipment>,
    pub helmet: Option<Equipment>,
    pub accessory: Option<Equipment>,
}

impl EquipmentInventory {
    pub fn equip(&mut self, equipment: Equipment) -> Option<Equipment> {
        match equipment.slot {
            EquipmentSlot::Weapon => {
                let old = self.weapon.take();
                self.weapon = Some(equipment);
                old
            }
            EquipmentSlot::Armor => {
                let old = self.armor.take();
                self.armor = Some(equipment);
                old
            }
            EquipmentSlot::Boots => {
                let old = self.boots.take();
                self.boots = Some(equipment);
                old
            }
            EquipmentSlot::Helmet => {
                let old = self.helmet.take();
                self.helmet = Some(equipment);
                old
            }
            EquipmentSlot::Accessory => {
                let old = self.accessory.take();
                self.accessory = Some(equipment);
                old
            }
        }
    }

    pub fn total_health_bonus(&self) -> f32 {
        let mut total = 0.0;
        if let Some(ref w) = self.weapon { total += w.health_bonus; }
        if let Some(ref a) = self.armor { total += a.health_bonus; }
        if let Some(ref b) = self.boots { total += b.health_bonus; }
        if let Some(ref h) = self.helmet { total += h.health_bonus; }
        if let Some(ref acc) = self.accessory { total += acc.health_bonus; }
        total
    }

    pub fn total_damage_bonus(&self) -> f32 {
        let mut total = 0.0;
        if let Some(ref w) = self.weapon { total += w.damage_bonus; }
        if let Some(ref a) = self.armor { total += a.damage_bonus; }
        if let Some(ref b) = self.boots { total += b.damage_bonus; }
        if let Some(ref h) = self.helmet { total += h.damage_bonus; }
        if let Some(ref acc) = self.accessory { total += acc.damage_bonus; }
        total
    }

    pub fn total_armor_bonus(&self) -> f32 {
        let mut total = 0.0;
        if let Some(ref w) = self.weapon { total += w.armor_bonus; }
        if let Some(ref a) = self.armor { total += a.armor_bonus; }
        if let Some(ref b) = self.boots { total += b.armor_bonus; }
        if let Some(ref h) = self.helmet { total += h.armor_bonus; }
        if let Some(ref acc) = self.accessory { total += acc.armor_bonus; }
        total
    }

    pub fn total_speed_bonus(&self) -> f32 {
        let mut total = 0.0;
        if let Some(ref w) = self.weapon { total += w.speed_bonus; }
        if let Some(ref a) = self.armor { total += a.speed_bonus; }
        if let Some(ref b) = self.boots { total += b.speed_bonus; }
        if let Some(ref h) = self.helmet { total += h.speed_bonus; }
        if let Some(ref acc) = self.accessory { total += acc.speed_bonus; }
        total
    }
}

/// Blacksmith inventory
#[derive(Resource, Default)]
pub struct BlacksmithInventory {
    pub equipment: Vec<Equipment>,
}

impl BlacksmithInventory {
    pub fn generate_for_town(town_size: crate::world_map::VillageSize) -> Self {
        let mut equipment = Vec::new();

        // Weapons
        equipment.push(Equipment::new_weapon("Iron Sword", 10.0, 50.0, 1));
        equipment.push(Equipment::new_weapon("Steel Sword", 18.0, 120.0, 3));
        equipment.push(Equipment::new_weapon("Iron Bow", 12.0, 60.0, 1));
        equipment.push(Equipment::new_weapon("Longbow", 22.0, 150.0, 3));
        equipment.push(Equipment::new_weapon("Iron Spear", 14.0, 55.0, 1));
        equipment.push(Equipment::new_weapon("Crossbow", 25.0, 180.0, 4));

        // Armor
        equipment.push(Equipment::new_armor("Leather Armor", 5.0, 20.0, 40.0, 1));
        equipment.push(Equipment::new_armor("Chainmail", 10.0, 40.0, 100.0, 2));
        equipment.push(Equipment::new_armor("Plate Armor", 18.0, 60.0, 250.0, 4));

        // Boots
        equipment.push(Equipment::new_boots("Leather Boots", 0.1, 2.0, 20.0, 1));
        equipment.push(Equipment::new_boots("Iron Boots", 0.05, 5.0, 50.0, 2));
        equipment.push(Equipment::new_boots("Speed Boots", 0.25, 3.0, 80.0, 3));

        // Filter by town size
        let max_level = match town_size {
            crate::world_map::VillageSize::Hamlet => 1,
            crate::world_map::VillageSize::Village => 2,
            crate::world_map::VillageSize::Town => 3,
            crate::world_map::VillageSize::City => 5,
        };

        equipment.retain(|e| e.level_required <= max_level);

        Self { equipment }
    }
}
