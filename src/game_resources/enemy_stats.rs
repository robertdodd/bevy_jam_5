use bevy::prelude::*;

use crate::constants;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct EnemyStats {
    /// Movement speed multiplier
    pub movement_speed: f32,
    /// Damage multiplier
    pub damage: f32,
    /// Damage multiplier
    pub mob_size: u32,
}

impl Default for EnemyStats {
    fn default() -> Self {
        Self {
            movement_speed: constants::ENEMY_MOVEMENT_SPEED,
            damage: constants::ENEMY_BASE_DAMAGE,
            mob_size: constants::ENEMY_DEFAULT_MOB_SIZE,
        }
    }
}
