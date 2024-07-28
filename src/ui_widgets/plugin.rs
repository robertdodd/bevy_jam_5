use bevy::prelude::*;

use super::*;

pub struct UiWidgetsPlugin;

impl Plugin for UiWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuButtonPlugin, ProgressBarWidgetPlugin));
    }
}
