use bevy::prelude::*;
use rand::Rng;
use std::collections::{HashMap, HashSet};

/// A village/settlement node in the world graph
#[derive(Debug, Clone)]
pub struct Village {
    pub id: usize,
    pub name: String,
    pub position: Vec2,
    pub size: VillageSize,
    pub is_discovered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VillageSize {
    Hamlet,      // Very small
    Village,     // Small
    Town,        // Medium
    City,        // Large
}

impl VillageSize {
    pub fn color(&self) -> Color {
        match self {
            VillageSize::Hamlet => Color::srgb(0.6, 0.6, 0.5),
            VillageSize::Village => Color::srgb(0.7, 0.7, 0.6),
            VillageSize::Town => Color::srgb(0.8, 0.8, 0.7),
            VillageSize::City => Color::srgb(1.0, 0.9, 0.7),
        }
    }

    pub fn radius(&self) -> f32 {
        match self {
            VillageSize::Hamlet => 8.0,
            VillageSize::Village => 12.0,
            VillageSize::Town => 18.0,
            VillageSize::City => 25.0,
        }
    }
}

/// A path connecting two villages - can have multiple waypoints
#[derive(Debug, Clone)]
pub struct Path {
    pub id: usize,
    pub village_a: usize,
    pub village_b: usize,
    pub waypoints: Vec<Vec2>, // Includes start and end positions
    pub distance: f32,
    pub is_discovered: bool,
}

impl Path {
    /// Creates a simple straight path between two points
    pub fn straight(id: usize, village_a: usize, village_b: usize, pos_a: Vec2, pos_b: Vec2) -> Self {
        let distance = pos_a.distance(pos_b);
        Self {
            id,
            village_a,
            village_b,
            waypoints: vec![pos_a, pos_b],
            distance,
            is_discovered: false,
        }
    }

    /// Creates a curved path with waypoints
    pub fn curved(id: usize, village_a: usize, village_b: usize, pos_a: Vec2, pos_b: Vec2, curve_factor: f32) -> Self {
        let mid = (pos_a + pos_b) / 2.0;
        let direction = (pos_b - pos_a).perp().normalize();
        let curve_offset = direction * curve_factor;

        let waypoint = mid + curve_offset;

        let distance = pos_a.distance(waypoint) + waypoint.distance(pos_b);

        Self {
            id,
            village_a,
            village_b,
            waypoints: vec![pos_a, waypoint, pos_b],
            distance,
            is_discovered: false,
        }
    }

    /// Get all waypoints as a polygon for rendering
    pub fn as_polygon(&self, width: f32) -> Vec<Vec2> {
        if self.waypoints.len() < 2 {
            return vec![];
        }

        let mut polygon = Vec::new();
        let half_width = width / 2.0;

        // Create polygon by offsetting along the path
        for i in 0..self.waypoints.len() - 1 {
            let p1 = self.waypoints[i];
            let p2 = self.waypoints[i + 1];
            let dir = (p2 - p1).normalize();
            let perp = Vec2::new(-dir.y, dir.x) * half_width;

            polygon.push(p1 + perp);
            polygon.push(p1 - perp);
        }

        polygon
    }
}

/// The world map graph structure
#[derive(Resource, Debug, Clone)]
pub struct WorldMap {
    pub villages: HashMap<usize, Village>,
    pub paths: HashMap<usize, Path>,
    pub adjacency: HashMap<usize, Vec<usize>>, // village_id -> connected path_ids
    pub starting_village: usize,
    pub discovered_villages: HashSet<usize>,
    pub discovered_paths: HashSet<usize>,
}

impl WorldMap {
    pub fn new() -> Self {
        Self {
            villages: HashMap::new(),
            paths: HashMap::new(),
            adjacency: HashMap::new(),
            starting_village: 0,
            discovered_villages: HashSet::new(),
            discovered_paths: HashSet::new(),
        }
    }

    /// Mark a village as discovered and reveal connected paths
    pub fn discover_village(&mut self, village_id: usize) {
        self.discovered_villages.insert(village_id);

        if let Some(village) = self.villages.get_mut(&village_id) {
            village.is_discovered = true;
        }

        // Reveal all paths connected to this village
        if let Some(path_ids) = self.adjacency.get(&village_id) {
            for path_id in path_ids {
                self.discovered_paths.insert(*path_id);
                if let Some(path) = self.paths.get_mut(path_id) {
                    path.is_discovered = true;
                }
            }
        }
    }

    /// Get all villages connected to a given village
    pub fn get_connected_villages(&self, village_id: usize) -> Vec<usize> {
        let mut connected = Vec::new();

        if let Some(path_ids) = self.adjacency.get(&village_id) {
            for path_id in path_ids {
                if let Some(path) = self.paths.get(path_id) {
                    let other_id = if path.village_a == village_id {
                        path.village_b
                    } else {
                        path.village_a
                    };
                    connected.push(other_id);
                }
            }
        }

        connected
    }

    /// Check if a village is visible (discovered or connected to discovered village)
    pub fn is_village_visible(&self, village_id: usize) -> bool {
        if self.discovered_villages.contains(&village_id) {
            return true;
        }

        // Check if any discovered village has a path to this one
        for discovered_id in &self.discovered_villages {
            if let Some(path_ids) = self.adjacency.get(discovered_id) {
                for path_id in path_ids {
                    if let Some(path) = self.paths.get(path_id) {
                        if path.village_a == village_id || path.village_b == village_id {
                            return false; // Connected but not discovered (under fog)
                        }
                    }
                }
            }
        }

        false
    }
}

/// Component to mark the world map UI
#[derive(Component)]
pub struct WorldMapView;

/// Component for rendered village markers
#[derive(Component)]
pub struct VillageMarker {
    pub village_id: usize,
}

/// Component for rendered path polygons
#[derive(Component)]
pub struct PathMarker {
    pub path_id: usize,
}
