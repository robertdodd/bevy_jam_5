use bevy::prelude::*;
use bevy_ui_helpers::*;

use crate::{game_state::*, *};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), setup_menu)
            .add_systems(
                Update,
                (
                    handle_game_input.run_if(in_state(GameState::Play)),
                    handle_menu_input.run_if(in_state(GameState::Paused)),
                ),
            );
    }
}

fn setup_menu(mut commands: Commands) {
    root(
        &mut commands,
        StateScoped(GameState::Paused),
        (c_full_screen, c_center, c_col),
    )
    .with_children(|p| {
        menu_title(p, "Paused");
        menu_button_widget(p, "Resume", MenuButtonAction::Resume);
        menu_button_widget(p, "Quit", MenuButtonAction::Quit);
    });
}

fn handle_menu_input(
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Play);
    }
}

fn handle_game_input(
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}
