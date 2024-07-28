use bevy::{color::palettes::css, prelude::*};

use bevy_ui_helpers::*;

pub struct MenuButtonPlugin;

impl Plugin for MenuButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_interactions);
    }
}

const BG_PRESSED: Srgba = css::RED;
const BG_HOVERED: Srgba = css::RED;
const BG_NONE: Srgba = css::RED;

const BORDER_PRESSED: Srgba = css::WHITE;
const BORDER_HOVERED: Srgba = css::WHITE;
const BORDER_NONE: Srgba = css::WHITE;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct MenuButton;

pub fn button_widget(
    parent: &mut ChildBuilder,
    title: impl Into<String>,
    extras: impl Bundle,
    class: impl ClassBuilder<ButtonBundle>,
) {
    let mut bundle = ButtonBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        border_radius: BorderRadius::all(Val::Px(5.)),
        background_color: BG_NONE.into(),
        border_color: Color::WHITE.into(),
        ..default()
    };
    class.apply(&mut bundle);

    parent
        .spawn((Name::new("Menu Button"), MenuButton, bundle, extras))
        .with_children(|p| {
            text(
                p,
                title,
                (),
                TextStyle {
                    font_size: 22.,
                    color: Color::WHITE,
                    ..default()
                },
            );
        });
}

pub fn menu_button_widget(
    parent: &mut ChildBuilder,
    title: impl Into<String>,
    extras: impl Bundle,
) {
    button_widget(parent, title, extras, |b: &mut ButtonBundle| {
        b.style.width = Val::Px(200.);
        b.style.height = Val::Px(65.);
        b.style.margin = UiRect::bottom(Val::Px(10.));
    });
}

fn handle_interactions(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (With<MenuButton>, Changed<Interaction>),
    >,
) {
    for (interaction, mut bg, mut border) in query.iter_mut() {
        let (new_bg, new_border) = match *interaction {
            Interaction::Pressed => (BG_PRESSED, BORDER_PRESSED),
            Interaction::Hovered => (BG_HOVERED.darker(0.1), BORDER_HOVERED),
            Interaction::None => (BG_NONE, BORDER_NONE),
        };
        *bg = new_bg.into();
        *border = new_border.into();
    }
}
