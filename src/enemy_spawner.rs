use bevy::prelude::*;

use crate::{in_game_not_paused, AppState, SpawnEnemies};

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), init_enemy_spawner)
            .add_systems(Update, tick_spawners.run_if(in_game_not_paused));
    }
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct EnemySpawner {
    pub timer: Timer,
}

fn init_enemy_spawner(mut commands: Commands) {
    commands.insert_resource(EnemySpawner {
        timer: Timer::from_seconds(2., TimerMode::Repeating),
    });
}

fn tick_spawners(
    mut spawner: ResMut<EnemySpawner>,
    time: Res<Time>,
    mut spawn_writer: EventWriter<SpawnEnemies>,
) {
    spawner.timer.tick(time.delta());
    if spawner.timer.just_finished() {
        spawn_writer.send(SpawnEnemies);
    }
}
