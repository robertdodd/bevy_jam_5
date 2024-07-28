use bevy::prelude::*;

use crate::*;

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_level_ups.run_if(in_game_not_paused));
    }
}

fn handle_level_ups(mut next_state: ResMut<NextState<GameState>>, mut score: ResMut<PlayerScore>) {
    if score.get_level_up() {
        next_state.set(GameState::PowerUp);
    }
}
