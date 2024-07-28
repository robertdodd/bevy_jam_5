use bevy::prelude::*;

use crate::*;

use self::math::get_angle_for_arc_length;

pub struct AttractorPlugin;

impl Plugin for AttractorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_attractors.run_if(in_game_not_paused));
    }
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct Attractor {
    pub radius: f32,
}

impl Attractor {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct Attractable;

fn handle_attractors(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Attractable>>,
    attractor_query: Query<(&GlobalTransform, &Attractor)>,
) {
    for mut transform in query.iter_mut() {
        let closest_attractor = attractor_query
            .iter()
            .filter_map(|(t, attractor)| {
                let pos = t.translation();
                let dist = pos.distance(transform.translation);
                if dist <= attractor.radius {
                    Some((dist, pos))
                } else {
                    None
                }
            })
            .reduce(|acc, e| if e.0 < acc.0 { e } else { acc });

        if let Some((_, pos)) = closest_attractor {
            // get rotation towards the attractor
            let target_rot =
                Quat::from_rotation_arc(transform.translation.normalize(), pos.normalize());
            let (axis, angle) = target_rot.to_axis_angle();

            // compute angle for movement speed
            let speed_angle = get_angle_for_arc_length(
                constants::ATTRACTOR_SPEED,
                constants::PLANET_RADIUS + constants::ENEMY_SIZE / 2.,
            ) * time.delta_seconds();

            let rot = Quat::from_axis_angle(axis, speed_angle * angle.signum() * 0.5);
            transform.rotate_around(Vec3::ZERO, rot)
        }
    }
}
