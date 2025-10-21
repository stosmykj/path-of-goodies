use bevy::prelude::*;
use rand::Rng;

/// Marks a tile in the world
#[derive(Component)]
pub struct WorldTile {
    pub tile_type: TileType,
}

/// Types of tiles in the world
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Grass,
    Road,
    Forest,
    Water,
    Mountain,
}

impl TileType {
    /// Get the color for this tile type (placeholder until we have sprites)
    pub fn color(&self) -> Color {
        match self {
            TileType::Grass => Color::srgb(0.2, 0.6, 0.2),
            TileType::Road => Color::srgb(0.5, 0.4, 0.3),
            TileType::Forest => Color::srgb(0.1, 0.4, 0.1),
            TileType::Water => Color::srgb(0.2, 0.4, 0.8),
            TileType::Mountain => Color::srgb(0.5, 0.5, 0.5),
        }
    }
}

/// Configuration for world generation
pub struct WorldConfig {
    pub chunk_size: i32,
    pub tile_size: f32,
    pub seed: u64,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            chunk_size: 32,
            tile_size: 32.0,
            seed: 42,
        }
    }
}

/// Generates a simple procedural world chunk
pub fn setup_world(mut commands: Commands) {
    let config = WorldConfig::default();
    let mut rng = rand::thread_rng();

    // Generate a simple chunk of world around spawn
    let half_size = config.chunk_size / 2;

    for x in -half_size..half_size {
        for y in -half_size..half_size {
            let world_x = x as f32 * config.tile_size;
            let world_y = y as f32 * config.tile_size;

            // Simple procedural generation
            let tile_type = if x.abs() < 3 && y.abs() < 3 {
                // Road in the center
                TileType::Road
            } else {
                // Random terrain
                let rand_val: f32 = rng.gen();
                if rand_val < 0.6 {
                    TileType::Grass
                } else if rand_val < 0.8 {
                    TileType::Forest
                } else if rand_val < 0.95 {
                    TileType::Water
                } else {
                    TileType::Mountain
                }
            };

            // Spawn tile
            commands.spawn((
                Sprite {
                    color: tile_type.color(),
                    custom_size: Some(Vec2::new(config.tile_size, config.tile_size)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 0.0),
                WorldTile { tile_type },
            ));
        }
    }

    info!("World chunk generated: {} tiles", config.chunk_size * config.chunk_size);
}
