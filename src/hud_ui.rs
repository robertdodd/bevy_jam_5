use bevy::{color::palettes::css, prelude::*};
use bevy_ui_helpers::*;

use crate::*;

pub struct HudUIPlugin;

impl Plugin for HudUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_hud)
            .add_systems(
                Update,
                (
                    update_score_progress_bar,
                    (
                        update_level_up_bar,
                        update_level_label,
                        update_total_score_label,
                        update_kill_count_label,
                    )
                        .run_if(resource_exists_and_changed::<PlayerScore>),
                    update_timer_label,
                )
                    .run_if(in_game),
            );
    }
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
struct HealthBar;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
struct LevelUpBar;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
struct LevelUpLabel;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
struct TimerLabel;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
struct KillCountLabel;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
struct TotalPointsLabel;

fn setup_hud(mut commands: Commands, score: Res<PlayerScore>) {
    root(
        &mut commands,
        (Name::new("HUD layout"), StateScoped(AppState::Game)),
        (c_full_screen, c_col),
    )
    .with_children(|p| {
        hud_level_progress(p, &score);
        hud_second_row_layout(p, |p| {
            // left column, contains score
            node(p, (), c_col).with_children(|p| {
                hud_health_bar(p);
            });
            // middle column, contains timer
            node(p, (), (c_col, c_center)).with_children(|p| {
                hud_timer_label(p);
            });
            // right column, contains enemies killed and score
            node(p, (), c_col).with_children(|p| {
                hud_last_column_layout(p, |p| {
                    hud_label_value_column(p, "KILL COUNT", "0", css::RED, KillCountLabel);
                    hud_label_value_column(p, "SCORE", "0", css::GOLD, TotalPointsLabel);
                });
            });
        });
    });
}

fn hud_second_row_layout(parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
    node(
        parent,
        (),
        (c_row, c_w_100, |b: &mut NodeBundle| {
            b.style.display = Display::Grid;
            b.style.grid_template_columns = vec![
                RepeatedGridTrack::fr(1, 1.),
                RepeatedGridTrack::auto(1),
                RepeatedGridTrack::fr(1, 1.),
            ];
            b.style.grid_template_rows = RepeatedGridTrack::min_content(1);
            b.style.justify_content = JustifyContent::SpaceBetween;
        }),
    )
    .with_children(children);
}

fn hud_last_column_layout(parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
    node(
        parent,
        (),
        (c_row, c_w_100, |b: &mut NodeBundle| {
            b.style.display = Display::Grid;
            b.style.grid_template_columns =
                vec![RepeatedGridTrack::fr(1, 1.), RepeatedGridTrack::fr(1, 1.)];
            b.style.grid_template_rows = RepeatedGridTrack::min_content(1);
            b.style.justify_content = JustifyContent::SpaceBetween;
        }),
    )
    .with_children(children);
}

fn hud_label_value_text(
    parent: &mut ChildBuilder,
    value: impl Into<String>,
    color: impl Into<Color>,
    extras: impl Bundle,
) {
    text(
        parent,
        value,
        extras,
        TextStyle {
            font_size: 18.,
            color: color.into(),
            ..default()
        },
    );
}

fn hud_label_value_column(
    parent: &mut ChildBuilder,
    label: impl Into<String>,
    value: impl Into<String>,
    label_color: impl Into<Color>,
    value_extras: impl Bundle,
) {
    node(parent, (), (c_row, c_justify_end)).with_children(|p| {
        node(p, (), (c_col, c_margin_right(10.))).with_children(|p| {
            hud_label_value_text(p, value, css::WHITE, value_extras);
        });
        hud_label_value_text(p, label, label_color, ());
    });
}

fn hud_health_bar(parent: &mut ChildBuilder) {
    node(parent, (), (c_col, c_width(200.))).with_children(|p| {
        progress_bar_widget(
            p,
            ProgressBar {
                amount: 100.,
                total: 100.,
                height: Val::Px(20.),
                color: css::LIMEGREEN.into(),
            },
            HealthBar,
        );
    });
}

fn hud_timer_label(parent: &mut ChildBuilder) {
    text(
        parent,
        "00:00",
        TimerLabel,
        TextStyle {
            font_size: 24.,
            ..default()
        },
    );
}

fn hud_level_progress(parent: &mut ChildBuilder, score: &PlayerScore) {
    node(
        parent,
        (),
        (c_row, c_width_percent(100.), c_margin_bottom(10.)),
    )
    .with_children(|p| {
        progress_bar_widget(
            p,
            ProgressBar {
                amount: score.current_points as f32,
                total: score.next_level as f32,
                height: Val::Px(30.),
                color: css::BLUE.into(),
            },
            LevelUpBar,
        );
        node(
            p,
            (),
            (
                c_position_absolute,
                c_top(0.),
                c_right(0.),
                c_bottom(0.),
                c_center,
            ),
        )
        .with_children(|p| {
            text(p, "Lv: 1", LevelUpLabel, TextStyle::default());
        });
    });
}

fn update_score_progress_bar(
    health_query: Query<&Health, (With<Player>, Changed<Health>)>,
    mut progress_bar_query: Query<&mut ProgressBar, With<HealthBar>>,
) {
    for health in health_query.iter() {
        for mut progress_bar in progress_bar_query.iter_mut() {
            progress_bar.amount = health.current;
            progress_bar.total = health.max_health;
        }
    }
}

fn update_level_up_bar(
    score: Res<PlayerScore>,
    mut bar_query: Query<&mut ProgressBar, With<LevelUpBar>>,
) {
    for mut progress_bar in bar_query.iter_mut() {
        progress_bar.amount = score.current_points as f32;
        progress_bar.total = score.next_level as f32;
    }
}

fn update_level_label(
    score: Res<PlayerScore>,
    mut label_query: Query<&mut Text, With<LevelUpLabel>>,
) {
    for mut text in label_query.iter_mut() {
        text.sections[0].value = format!("Lv: {}", score.level);
    }
}

fn update_kill_count_label(
    score: Res<PlayerScore>,
    mut label_query: Query<&mut Text, With<KillCountLabel>>,
) {
    for mut text in label_query.iter_mut() {
        text.sections[0].value = format!("{}", score.enemies_killed);
    }
}

fn update_total_score_label(
    score: Res<PlayerScore>,
    mut label_query: Query<&mut Text, With<TotalPointsLabel>>,
) {
    for mut text in label_query.iter_mut() {
        text.sections[0].value = format!("{}", score.total_points);
    }
}

fn update_timer_label(timer: Res<GameTimer>, mut label_query: Query<&mut Text, With<TimerLabel>>) {
    for mut text in label_query.iter_mut() {
        text.sections[0].value = timer.to_time_string();
    }
}
