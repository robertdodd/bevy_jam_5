use bevy::prelude::*;

#[derive(Component, Debug, Reflect, PartialEq, Eq, Hash)]
#[reflect(Component, Debug, PartialEq, Hash)]
pub enum MenuButtonAction {
    Play,
    Quit,
    MainMenu,
    Resume,
}
