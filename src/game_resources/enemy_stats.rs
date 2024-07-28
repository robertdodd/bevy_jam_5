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

impl EnemyStats {
    pub fn get_movement_speed(&self) -> f32 {
        let base_amount = constants::ENEMY_MOVEMENT_SPEED;
        base_amount + base_amount * self.movement_speed / 100.
    }

    pub fn get_damage(&self) -> f32 {
        let base_amount = constants::ENEMY_BASE_DAMAGE;
        base_amount + base_amount * self.damage / 100.
    }
}
