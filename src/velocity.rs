use bevy::prelude::*;

use crate::*;

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_velocity.run_if(in_game_not_paused));
    }
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct Velocity {
    pub pos: f32,
    pub axis: Vec3,
    pub speed: f32,
}

fn tick_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        let rot = Quat::from_axis_angle(velocity.axis, time.delta_seconds());
        transform.rotate_around(Vec3::ZERO, rot);
        // IMPORTANT: Ensure constant height above ground, otherwise it decreases over time
        transform.translation = transform.translation.normalize() * velocity.pos;
    }
}
