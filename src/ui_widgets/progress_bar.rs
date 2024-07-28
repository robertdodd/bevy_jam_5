use bevy::{color::palettes::css, prelude::*};
use bevy_ui_helpers::*;

pub struct ProgressBarWidgetPlugin;

impl Plugin for ProgressBarWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_progress_bar_widgets);
    }
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct ProgressBar {
    pub amount: f32,
    pub total: f32,
    pub height: Val,
    pub color: Color,
}

impl ProgressBar {
    pub fn percent(&self) -> f32 {
        self.fraction() * 100.
    }

    pub fn fraction(&self) -> f32 {
        self.amount / self.total
    }
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct ProgressBarInner;

pub fn progress_bar_widget(
    parent: &mut ChildBuilder,
    progress_bar: ProgressBar,
    extras: impl Bundle,
) {
    let percent = progress_bar.percent();
    let height = progress_bar.height;
    let color = progress_bar.color;

    node(
        parent,
        (progress_bar, extras),
        (
            c_row,
            |b: &mut NodeBundle| {
                b.style.height = height;
            },
            c_width_percent(100.),
            c_bg_dark_gray,
            c_border_1,
            c_border_color(css::WHITE),
        ),
    )
    .with_children(|p| {
        node(
            p,
            ProgressBarInner,
            (
                c_background_color(color),
                c_height_percent(100.),
                c_row,
                c_width_percent(percent),
                c_align_center,
            ),
        );
    });
}

fn update_progress_bar_widgets(
    query: Query<(&Children, &ProgressBar), Changed<ProgressBar>>,
    mut inner_query: Query<&mut Style, With<ProgressBarInner>>,
) {
    for (children, progress_bar) in query.iter() {
        for &child in children.iter() {
            if let Ok(mut style) = inner_query.get_mut(child) {
                // clamp the percent so it doesn't exceed the bounds
                let clamped_percent = progress_bar.percent().clamp(0., 100.);

                style.width = Val::Percent(clamped_percent);
            }
        }
    }
}
