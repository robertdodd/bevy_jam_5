use std::time::Duration;

use bevy::prelude::*;

use crate::{constants, in_game_not_paused, EnemyStats, SpawnEnemies};

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawner>().add_systems(
            Update,
            (
                tick_spawners,
                update_spawn_speed.run_if(resource_exists_and_changed::<EnemyStats>),
            )
                .run_if(in_game_not_paused),
        );
    }
}

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct EnemySpawner {
    pub timer: Timer,
    pub has_started: bool,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(constants::ENEMY_DEFAULT_SPAWN_SPEED, TimerMode::Repeating),
            has_started: false,
        }
    }
}

fn tick_spawners(
    mut spawner: ResMut<EnemySpawner>,
    time: Res<Time>,
    mut spawn_writer: EventWriter<SpawnEnemies>,
) {
    spawner.timer.tick(time.delta());
    if spawner.timer.just_finished() || !spawner.has_started {
        spawner.has_started = true;
        spawn_writer.send(SpawnEnemies);
    }
}

fn update_spawn_speed(stats: Res<EnemyStats>, mut spawner: ResMut<EnemySpawner>) {
    spawner
        .timer
        .set_duration(Duration::from_secs_f32(stats.spawn_speed));
}
