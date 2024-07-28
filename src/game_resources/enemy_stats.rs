use bevy::prelude::*;

use crate::constants;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct EnemyStats {
    /// Movement speed multiplier
    pub movement_speed: f32,
    /// Damage multiplier
    pub damage: f32,
    /// Number of enemies per mob
    pub mob_size: u32,
    /// Number of mobs to spawn
    pub mob_count: u32,
    /// spawn speed
    pub spawn_speed: f32,
}

impl Default for EnemyStats {
    fn default() -> Self {
        Self {
            movement_speed: constants::ENEMY_MOVEMENT_SPEED,
            damage: constants::ENEMY_BASE_DAMAGE,
            mob_size: constants::ENEMY_DEFAULT_MOB_SIZE,
            mob_count: constants::ENEMY_DEFAULT_MOB_COUNT,
            spawn_speed: constants::ENEMY_DEFAULT_SPAWN_SPEED,
        }
    }
}
