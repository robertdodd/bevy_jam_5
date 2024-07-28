use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::ClassBuilder;

/// Spawns a [`NodeBundle`] in a [`ChildBuilder`].
///
/// # Example:
///
/// ```rust,ignore
/// commands.spawn(NodeBundle::default()).with_children(|p| {
///     // Un-styled:
///     node(p, (), ());
///
///     // Using a custom style:
///     node(p, (), |b: &mut NodeBundle| {
///         b.background_color = Color::RED.into();
///     });
///
///     // Using [`ClassBuilder`] functions for the style:
///     node(p, (), (c_border_1, c_border_red));
///
///     // With children:
///     node(p, (), (c_border_1, c_border_red)).with_children(|p| {
///         node(p, (), ());
///     });
/// });
/// ```
pub fn node<'a>(
    parent: &'a mut ChildBuilder,
    extras: impl Bundle,
    class: impl ClassBuilder<NodeBundle>,
) -> EntityCommands<'a> {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle);
    parent.spawn((bundle, extras))
}

/// Spawns a root [`NodeBundle`].
///
/// # Examples:
///
/// ```rust,ignore
/// // Un-styled:
/// root(&mut commands, (), ());
///
/// // Using a custom style:
/// root(&mut commands, (), |b: &mut NodeBundle| {
///     b.background_color = Color::RED.into();
/// });
///
/// // Using [`ClassBuilder`] functions for the style:
/// root(&mut commands, (), (c_border_1, c_border_red));
///
/// // With children:
/// root(&mut commands, (), (c_border_1, c_border_red)).with_children(|p| {
///     node(p, (), ());
/// });
/// ```
pub fn root<'a>(
    commands: &'a mut Commands,
    extras: impl Bundle,
    class: impl ClassBuilder<NodeBundle>,
) -> EntityCommands<'a> {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle);
    commands.spawn((bundle, extras))
}

/// Spawns a [`TextBundle`] in a [`ChildBuilder`].
///
/// # Example:
///
/// ```rust,ignore
/// commands.spawn(NodeBundle::default()).with_children(|p| {
///     text("My Title", p, TextStyle::default());
/// });
/// ```
pub fn text(
    parent: &mut ChildBuilder,
    txt: impl Into<String>,
    extras: impl Bundle,
    style: TextStyle,
) -> Entity {
    parent
        .spawn((TextBundle::from_section(txt, style), extras))
        .id()
}

/// Spawns a centered [`TextBundle`] in a [`ChildBuilder`].
///
/// # Example:
///
/// ```rust,ignore
/// commands.spawn(NodeBundle::default()).with_children(|p| {
///     text_center("My Title", p, TextStyle::default());
/// });
/// ```
pub fn text_centered(
    parent: &mut ChildBuilder,
    txt: impl Into<String>,
    extras: impl Bundle,
    style: TextStyle,
) -> Entity {
    parent
        .spawn((
            TextBundle::from_section(txt, style).with_text_justify(JustifyText::Center),
            extras,
        ))
        .id()
}
