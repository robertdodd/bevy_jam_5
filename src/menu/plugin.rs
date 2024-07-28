use bevy::prelude::*;

use crate::{game_state::*, *};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MainMenuPlugin,
            GameOverMenuPlugin,
            PauseMenuPlugin,
            PowerUpMenuPlugin,
        ))
        .add_systems(Update, handle_button_click);
    }
}

fn handle_button_click(
    query: Query<(&MenuButtonAction, &Interaction), Changed<Interaction>>,
    mut app_exit_writer: EventWriter<AppExit>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (action, interaction) in query.iter() {
        if *interaction == Interaction::Pressed {
            match *action {
                MenuButtonAction::Play => {
                    next_app_state.set(AppState::Game);
                }
                MenuButtonAction::Quit => {
                    app_exit_writer.send(AppExit::Success);
                }
                MenuButtonAction::MainMenu => {
                    next_app_state.set(AppState::Menu);
                }
                MenuButtonAction::Resume => {
                    next_game_state.set(GameState::Play);
                }
            }
        }
    }
}
