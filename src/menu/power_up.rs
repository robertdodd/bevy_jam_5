use bevy::{color::palettes::css, prelude::*};
use bevy_ui_helpers::*;

use crate::{game_state::*, *};

use super::{MENU_SPACER, TABLE_SPACER};

pub struct PowerUpMenuPlugin;

impl Plugin for PowerUpMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PowerUp), setup_menu)
            .add_systems(
                Update,
                handle_power_up_button_clicked.run_if(in_state(GameState::PowerUp)),
            );
    }
}

const POWER_UP_FONT_SIZE_NAME: f32 = 24.;
const POWER_UP_FONT_SIZE_DESCRIPTION: f32 = 18.;
const POWER_UP_SPACER: f32 = 30.;
const POWER_UP_SPACER_LG: f32 = 60.;

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Debug)]
pub struct PowerUpButton(pub PowerUp);

fn setup_menu(mut commands: Commands, score: Res<PlayerScore>, stats: Res<PlayerStats>) {
    // Generate 3 random stat upgrades
    let power_ups = [
        PowerUp::new_random(),
        PowerUp::new_random(),
        PowerUp::new_random(),
    ];

    root(
        &mut commands,
        StateScoped(GameState::PowerUp),
        (c_full_screen, c_center, c_col),
    )
    .with_children(|p| {
        stats_panel(p, &stats);
        power_up_panel(p, |p| {
            menu_title(p, "Choose Power Up");
            power_up_description(p, format!("You have reached level {}!", score.level));
            power_up_layout(p, |p| {
                for power_up in power_ups {
                    power_up_widget(p, power_up);
                }
            });
        });
    });
}

fn stats_panel(parent: &mut ChildBuilder, stats: &PlayerStats) {
    node(
        parent,
        (),
        (
            c_col,
            c_width_percent(20.),
            c_center,
            |b: &mut NodeBundle| {
                b.style.position_type = PositionType::Absolute;
                b.style.top = Val::Px(MENU_SPACER);
                b.style.left = Val::Px(MENU_SPACER);
                b.style.bottom = Val::Px(MENU_SPACER);
            },
        ),
    )
    .with_children(|p| {
        node(
            p,
            (),
            (
                c_col,
                c_background_color(css::DARK_SLATE_GRAY),
                c_border_white,
                c_padding(MENU_SPACER),
                c_border_radius(10.),
            ),
        )
        .with_children(|p| {
            node(p, (), (c_margin_bottom(MENU_SPACER * 2.), c_justify_center)).with_children(|p| {
                text(p, "Stats", (), TextStyle::default());
            });
            stats_table(p, |p| {
                // Health
                stats_table_row(p, "Max Health", format!("{}", stats.max_health));
                stats_table_row(p, "Recovery", format!("{}", stats.recovery));
                stats_table_row(p, "Armor", format!("{}", stats.armor));
                stats_table_row(p, " ", " ");
                // Movement
                stats_table_row(p, "Move Speed", format!("{}", stats.move_speed));
                stats_table_row(p, " ", " ");
                // Attack
                stats_table_row(p, "Damage", format!("{}%", stats.damage_percent));
                stats_table_row(p, "Attack Size", format!("{}%", stats.attack_size_percent));
                stats_table_row(p, "Cooldown", format!("{}", stats.attack_cooldown));
                stats_table_row(p, "Weapon Amount", format!("{}", stats.attack_amount_extra));
                stats_table_row(p, " ", " ");
                // Pickup Radius
                stats_table_row(p, "Pickup Radius", format!("{}", stats.pickup_radius));
            });
        });
    });
}

fn power_up_panel(parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
    node(
        parent,
        (),
        (
            c_col,
            c_center,
            c_background_color(css::DARK_SLATE_GRAY),
            c_border_white,
            c_padding(POWER_UP_SPACER_LG),
            c_width_percent(50.),
            c_border_radius(20.),
        ),
    )
    .with_children(children);
}

fn power_up_layout(parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
    node(
        parent,
        (),
        (c_row, c_w_100, |b: &mut NodeBundle| {
            b.style.display = Display::Grid;
            b.style.grid_template_columns = vec![
                RepeatedGridTrack::fr(1, 1.),
                RepeatedGridTrack::fr(1, 1.),
                RepeatedGridTrack::fr(1, 1.),
            ];
            b.style.grid_template_rows = RepeatedGridTrack::min_content(1);
            b.style.column_gap = Val::Px(MENU_SPACER);
            b.style.justify_content = JustifyContent::SpaceBetween;
        }),
    )
    .with_children(children);
}

fn power_up_description(parent: &mut ChildBuilder, value: impl Into<String>) {
    node(parent, (), c_margin_bottom(POWER_UP_SPACER)).with_children(|p| {
        text(p, value, (), TextStyle::default());
    });
}

fn power_up_widget(parent: &mut ChildBuilder, power_up: PowerUp) {
    node(
        parent,
        Name::new(format!("Power Up: {:?}", power_up.stat)),
        (
            c_col,
            c_border_white,
            c_background_color(css::SLATE_GRAY),
            c_align_center,
            c_padding(MENU_SPACER),
            c_border_radius(10.),
        ),
    )
    .with_children(|p| {
        node(
            p,
            (),
            (
                c_w_100,
                c_center,
                c_margin_bottom(POWER_UP_SPACER),
                c_overflow_clip_x,
            ),
        )
        .with_children(|p| {
            text_centered(
                p,
                format!("{}", power_up.stat),
                (),
                TextStyle {
                    font_size: POWER_UP_FONT_SIZE_NAME,
                    ..default()
                },
            );
        });
        node(
            p,
            (),
            (c_col, c_margin_bottom(POWER_UP_SPACER), c_overflow_clip_x),
        )
        .with_children(|p| {
            text_centered(
                p,
                power_up.description(),
                (),
                TextStyle {
                    font_size: POWER_UP_FONT_SIZE_DESCRIPTION,
                    ..default()
                },
            );
        });
        node(
            p,
            (),
            (c_col_w_100, c_overflow_clip_x, |b: &mut NodeBundle| {
                b.style.margin.top = Val::Auto;
            }),
        )
        .with_children(|p| {
            button_widget(
                p,
                "Choose",
                PowerUpButton(power_up.clone()),
                |b: &mut ButtonBundle| {
                    b.style.padding = UiRect::all(Val::Px(MENU_SPACER));
                    b.style.width = Val::Percent(100.);
                },
            )
        });
    });
}

fn handle_power_up_button_clicked(
    query: Query<(&PowerUpButton, &Interaction), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut stats: ResMut<PlayerStats>,
) {
    for (button, interaction) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        stats.add_power_up(&button.0);
        next_state.set(GameState::Play);
    }
}

fn stats_table(parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
    node(
        parent,
        (),
        (
            c_row_w_100,
            c_margin_bottom(MENU_SPACER * 2.),
            |b: &mut NodeBundle| {
                b.style.display = Display::Grid;
                b.style.grid_template_columns =
                    vec![RepeatedGridTrack::fr(1, 1.), RepeatedGridTrack::auto(1)];
                b.style.grid_template_rows = RepeatedGridTrack::min_content(1);
                b.style.column_gap = Val::Px(TABLE_SPACER);
                b.style.row_gap = Val::Px(TABLE_SPACER);
                b.style.justify_content = JustifyContent::SpaceBetween;
            },
        ),
    )
    .with_children(children);
}

fn stats_table_row(parent: &mut ChildBuilder, label: impl Into<String>, value: impl Into<String>) {
    let table_font_size = 16.;
    node(parent, (), c_justify_end).with_children(|p| {
        text(
            p,
            label,
            (),
            TextStyle {
                font_size: table_font_size,
                color: css::GOLD.into(),
                ..default()
            },
        );
    });
    node(parent, (), ()).with_children(|p| {
        text(
            p,
            value,
            (),
            TextStyle {
                font_size: table_font_size,
                ..default()
            },
        );
    });
}
