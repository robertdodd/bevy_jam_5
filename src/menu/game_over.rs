use bevy::{color::palettes::css, prelude::*};
use bevy_ui_helpers::*;

use crate::{game_state::*, *};

use super::{MENU_SPACER, TABLE_SPACER};

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), setup_menu);
    }
}

fn setup_menu(mut commands: Commands, score: Res<PlayerScore>, timer: Res<GameTimer>) {
    root(
        &mut commands,
        StateScoped(AppState::GameOver),
        (c_full_screen, c_center, c_col),
    )
    .with_children(|p| {
        menu_title(p, "Game Over");
        stats_table(p, |p| {
            stats_table_row(p, "Time Alive", timer.to_string(), css::YELLOW);
            stats_table_row(p, "Points", format!("{}", score.total_points), css::BLUE);
            stats_table_row(
                p,
                "Enemies Killed",
                format!("{}", score.enemies_killed),
                css::RED,
            );
            stats_table_row(p, "Level", format!("{}", score.level), css::GOLD);
        });
        menu_button_widget(p, "Retry", MenuButtonAction::Play);
        menu_button_widget(p, "Quit to Menu", MenuButtonAction::MainMenu);
    });
}

fn stats_table(parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
    node(
        parent,
        (),
        (
            c_row,
            c_width(400.),
            c_margin_bottom(MENU_SPACER),
            |b: &mut NodeBundle| {
                b.style.display = Display::Grid;
                b.style.grid_template_columns =
                    vec![RepeatedGridTrack::fr(1, 1.), RepeatedGridTrack::fr(1, 1.)];
                b.style.grid_template_rows = RepeatedGridTrack::min_content(1);
                b.style.justify_content = JustifyContent::SpaceBetween;
            },
        ),
    )
    .with_children(children);
}

fn stats_table_row(
    parent: &mut ChildBuilder,
    label: impl Into<String>,
    value: impl Into<String>,
    label_color: impl Into<Color>,
) {
    node(parent, (), (c_justify_end, c_padding(TABLE_SPACER))).with_children(|p| {
        text(
            p,
            label,
            (),
            TextStyle {
                color: label_color.into(),
                ..default()
            },
        );
    });
    node(parent, (), c_padding(TABLE_SPACER)).with_children(|p| {
        text(p, value, (), TextStyle::default());
    });
}
