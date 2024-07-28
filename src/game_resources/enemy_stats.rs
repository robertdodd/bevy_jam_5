use bevy::prelude::*;

use crate::constants;

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct EnemyStats {
    /// Movement speed multiplier
    pub movement_speed: f32,
    /// Damage multiplier
    pub damage: f32,
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
