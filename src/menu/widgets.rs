use bevy::prelude::*;
use bevy_ui_helpers::*;

pub fn menu_title(parent: &mut ChildBuilder, title: impl Into<String>) {
    node(parent, (), c_margin_bottom(10.)).with_children(|p| {
        text(
            p,
            title,
            (),
            TextStyle {
                font_size: 40.,
                color: Color::WHITE,
                ..default()
            },
        );
    });
}
