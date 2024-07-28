use bevy::prelude::*;

use crate::*;

pub struct GameResourcesPlugin;

impl Plugin for GameResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerStats>()
            .init_resource::<EnemyStats>()
            .init_resource::<PlayerScore>()
            .init_resource::<GameTimer>()
            .add_systems(OnEnter(AppState::Game), reset_game_resources)
            .add_systems(Update, tick_game_timer.run_if(in_game_not_paused));
    }
}

fn reset_game_resources(mut commands: Commands) {
    commands.insert_resource(PlayerStats::default());
    commands.insert_resource(EnemyStats::default());
    commands.insert_resource(PlayerScore::default());
    commands.insert_resource(GameTimer::default());
    commands.insert_resource(EnemyStatsTimer::default());
}

fn tick_game_timer(time: Res<Time>, mut game_timer: ResMut<GameTimer>) {
    game_timer.0.tick(time.delta());
}
