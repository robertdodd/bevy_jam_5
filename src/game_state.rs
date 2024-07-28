use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Game,
    GameOver,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Play,
    Paused,
    PowerUp,
}

/// A [`Condition`](Condition)-satisfying system that returns `true` if the game is not paused.
pub fn not_paused(pause_state: Res<State<GameState>>) -> bool {
    *pause_state.get() == GameState::Play
}

/// A [`Condition`](Condition)-satisfying system that returns `true` if the game is active.
pub fn in_game(game_state: Res<State<AppState>>) -> bool {
    *game_state.get() == AppState::Game
}

/// A [`Condition`](Condition)-satisfying system that returns `true` if the game is active.
pub fn in_game_not_paused(
    game_state: Res<State<AppState>>,
    pause_state: Res<State<GameState>>,
) -> bool {
    in_game(game_state) && not_paused(pause_state)
}
