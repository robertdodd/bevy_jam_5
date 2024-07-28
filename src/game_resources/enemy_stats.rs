use bevy::prelude::*;

use crate::constants;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct EnemyStats {
    /// Movement speed multiplier
    pub movement_speed: f32,
    /// Damage multiplier
    pub damage: f32,
    /// Health
    pub health: f32,
    /// Number of enemies per mob
    pub mob_size: f32,
    /// Number of mobs to spawn
    pub mob_count: f32,
    /// spawn speed
    pub spawn_speed: f32,
}

impl EnemyStats {
    pub fn upgrade(&mut self) {
        self.movement_speed += 0.25;
        self.damage *= 1.1;
        self.health *= 1.1;
        self.mob_size *= 1.1;
        self.mob_count *= 1.25;
        // self.spawn_speed *= 0.8;
    }
}

impl Default for EnemyStats {
    fn default() -> Self {
        Self {
            movement_speed: constants::ENEMY_MOVEMENT_SPEED,
            damage: constants::ENEMY_BASE_DAMAGE,
            mob_size: constants::ENEMY_DEFAULT_MOB_SIZE as f32,
            mob_count: constants::ENEMY_DEFAULT_MOB_COUNT as f32,
            spawn_speed: constants::ENEMY_DEFAULT_SPAWN_SPEED,
            health: constants::ENEMY_DEFAULT_HEALTH,
        }
    }
}
