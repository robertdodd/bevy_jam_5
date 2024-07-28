use bevy::prelude::*;
use bevy_ui_helpers::*;

use crate::{game_state::*, *};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(Update, handle_input.run_if(in_state(AppState::Menu)));
    }
}

fn setup_menu(mut commands: Commands) {
    root(
        &mut commands,
        StateScoped(AppState::Menu),
        (c_full_screen, c_center, c_col),
    )
    .with_children(|p| {
        menu_title(p, constants::APP_NAME);
        menu_button_widget(p, "Play", MenuButtonAction::Play);
        menu_button_widget(p, "Quit", MenuButtonAction::Quit);
    });
}

fn handle_input(mut next_game_state: ResMut<NextState<AppState>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Enter) {
        next_game_state.set(AppState::Game);
    }
}
