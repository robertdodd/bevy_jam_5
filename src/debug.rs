#![allow(dead_code)]
use bevy::{color::palettes::css, prelude::*};

use crate::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (debug_axes, handle_input).run_if(in_state(AppState::Game)),
        );
    }
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut enemy_stats: ResMut<EnemyStats>,
    mut player_stats: ResMut<PlayerStats>,
) {
    if keys.just_pressed(KeyCode::KeyQ) {
        enemy_stats.movement_speed -= 5.;
    }
    if keys.just_pressed(KeyCode::KeyW) {
        enemy_stats.movement_speed += 5.;
    }
    if keys.just_pressed(KeyCode::KeyA) {
        player_stats.attack_size -= 5.;
    }
    if keys.just_pressed(KeyCode::KeyS) {
        player_stats.attack_size += 5.;
    }
    if keys.just_pressed(KeyCode::KeyZ) && player_stats.attack_amount > 1 {
        player_stats.attack_amount -= 1;
    }
    if keys.just_pressed(KeyCode::KeyX) {
        player_stats.attack_amount += 1;
    }
}

fn debug_axes(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<GameCamera>>,
    player_query: Query<&Player>,
) {
    let transform = query.single();
    let player = player_query.single();

    let camera_up = player.up.normalize();
    let towards_camera = transform.translation.normalize();
    let half_length = constants::PLANET_RADIUS * 1.5;
    let camera_right = camera_up.cross(towards_camera).normalize();

    // draw camera up
    gizmos.line(
        Vec3::ZERO,
        // -up * half_length,
        camera_up * half_length,
        css::RED,
    );

    // draw towards camera
    gizmos.line(
        Vec3::ZERO,
        // -towards_camera * half_length,
        towards_camera * half_length,
        css::GREEN,
    );

    // draw line in direction of cross product
    gizmos.line(
        Vec3::ZERO,
        // -axis * half_length,
        camera_right * half_length,
        css::BLUE,
    );
}
