use bevy::prelude::*;

use crate::constants;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct EnemyStatsTimer(pub Timer);

impl Default for EnemyStatsTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            constants::ENEMY_DEFAULT_SPAWN_SPEED,
            TimerMode::Once,
        ))
    }
}
