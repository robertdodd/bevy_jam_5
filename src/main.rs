// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{asset::AssetMetaCheck, prelude::*};

mod menu;
use menu::*;

mod game_state;
use game_state::*;

mod planet;
use planet::*;

mod enemy;
use enemy::*;

mod enemy_spawner;
use enemy_spawner::*;

mod player;
use player::*;

mod camera;
use camera::*;

mod foilage;
use foilage::*;

mod game_resources;
use game_resources::*;

mod weapon;
use weapon::*;

mod projectile;
use projectile::*;

mod health;
use health::*;

mod velocity;
use velocity::*;

mod collision;
use collision::*;

mod lifetime;
use lifetime::*;

// Un-comment to enable debug rendering and key presses, and add `DebugPlugin` to app.
// mod debug;
// use debug::*;

mod hud_ui;
use hud_ui::*;

mod ui_widgets;
use ui_widgets::*;

mod point;
use point::*;

mod attractor;
use attractor::*;

mod level_up;
use level_up::*;

mod orb;
use orb::*;

mod constants;
mod math;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins((
            PlanetPlugin,
            EnemyPlugin,
            EnemySpawnerPlugin,
            PlayerPlugin,
            CameraPlugin,
            FoilagePlugin,
            WeaponPlugin,
            ProjectilePlugin,
            VelocityPlugin,
            HealthPlugin,
            CollisionPlugin,
            LifetimePlugin,
            PointPlugin,
            AttractorPlugin,
            GameResourcesPlugin,
        ))
        .add_plugins((LevelUpPlugin, OrbPlugin))
        .add_plugins((MenuPlugin, UiWidgetsPlugin, HudUIPlugin))
        .init_state::<AppState>()
        .init_state::<GameState>()
        .enable_state_scoped_entities::<AppState>()
        .enable_state_scoped_entities::<GameState>()
        .run();
}
