use std::f32::consts::TAU;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::*;

use self::math::get_angle_for_arc_length;

pub struct OrbPlugin;

impl Plugin for OrbPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OrbResources>().add_systems(
            Update,
            (
                setup_new_orbs,
                handle_collision_events.run_if(on_event::<CollisionEvent>()),
                (tick_orb_hits, update_orb_transform).run_if(not_paused),
                update_orbs_on_stats_change.run_if(resource_exists_and_changed::<PlayerStats>),
            )
                .run_if(in_game),
        );
    }
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct Orb {
    pub damage: f32,
    pub angle: f32,
    pub hits: HashMap<Entity, Timer>,
}

impl Orb {
    pub fn new(damage: f32, angle: f32) -> Self {
        assert!(damage >= 0.);
        Self {
            damage,
            angle,
            hits: HashMap::<Entity, Timer>::new(),
        }
    }
}

#[derive(Bundle)]
pub struct OrbBundle {
    pub name: Name,
    pub orb: Orb,
    pub state_scoped: StateScoped<AppState>,
    pub transform: Transform,
    pub collider: Collider,
    pub collision_group: CollisionGroups,
}

impl OrbBundle {
    pub fn new(transform: Transform, damage: f32, angle: f32) -> Self {
        Self {
            name: Name::new("Orb"),
            orb: Orb::new(damage, angle),
            state_scoped: StateScoped(AppState::Game),
            transform,
            collider: Collider::Sphere(constants::ORB_RADIUS),
            collision_group: CollisionGroups::new(GROUP_PROJECTILE, GROUP_ENEMY),
        }
    }
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct OrbResources {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
}

impl OrbResources {
    pub fn get_or_create_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        if let Some(ref material) = self.material {
            material.clone()
        } else {
            let material = materials.add(StandardMaterial {
                emissive: LinearRgba::rgb(0.0, 13.99, 0.0),
                ..default()
            });
            self.material = Some(material.clone());
            material
        }
    }

    pub fn get_or_create_mesh(&mut self, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        if let Some(ref mesh) = self.mesh {
            mesh.clone()
        } else {
            let mesh = meshes.add(Sphere::new(1.));
            self.mesh = Some(mesh.clone());
            mesh
        }
    }
}

fn setup_new_orbs(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Added<Orb>>,
    mut orb_resources: ResMut<OrbResources>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, transform) in query.iter() {
        let material = orb_resources.get_or_create_material(&mut materials);
        let mesh = orb_resources.get_or_create_mesh(&mut meshes);

        commands.entity(entity).insert((
            Name::new("Orb"),
            MaterialMeshBundle {
                material,
                mesh,
                transform: *transform,
                ..default()
            },
        ));
    }
}

fn handle_collision_events(
    mut events: EventReader<CollisionEvent>,
    mut orb_query: Query<&mut Orb>,
    mut health_query: Query<&mut Health>,
) {
    for event in events.read() {
        let entity_pairs = [(event.e1, event.e2), (event.e2, event.e1)];
        for (health_entity, orb_entity) in entity_pairs {
            if let (Ok(mut health), Ok(mut orb)) = (
                health_query.get_mut(health_entity),
                orb_query.get_mut(orb_entity),
            ) {
                // ignore if projectile has already hit this entity
                if orb.hits.contains_key(&health_entity) {
                    continue;
                }
                health.current -= orb.damage;
                orb.hits.insert(
                    health_entity,
                    Timer::from_seconds(constants::ORB_COOLDOWN_SECS, TimerMode::Once),
                );
            }
        }
    }
}

fn tick_orb_hits(time: Res<Time>, mut query: Query<&mut Orb>) {
    for mut orb in query.iter_mut() {
        // tick the orb hits
        let mut to_remove = HashSet::<Entity>::new();
        for (e, timer) in orb.hits.iter_mut() {
            timer.tick(time.delta());
            if timer.finished() {
                to_remove.insert(*e);
            }
        }

        // remove hits where the timer has finished
        for e in to_remove {
            orb.hits.remove(&e);
        }
    }
}

/// System that rotates orbs around the player
fn update_orb_transform(
    time: Res<Time>,
    stats: Res<PlayerStats>,
    mut query: Query<(&mut Orb, &mut Transform)>,
    player_query: Query<(&Player, &GlobalTransform)>,
) {
    let speed = stats.get_attack_speed(constants::ORB_MOVEMENT_SPEED);

    for (mut orb, mut transform) in query.iter_mut() {
        // update the angle for correct movement speed
        let move_angle =
            get_angle_for_arc_length(speed, constants::ORB_ORBIT_RADIUS) * time.delta_seconds();
        orb.angle += move_angle;

        // get the players position and axis so we can rotate around it
        let (player, player_transform) = player_query.single();

        // set the orb's new transform
        *transform = get_orb_transform(
            player_transform.translation(),
            player.up.normalize(),
            orb.angle,
            stats.get_attack_size(constants::ORB_RADIUS),
        );
    }
}

fn update_orbs_on_stats_change(
    mut commands: Commands,
    stats: Res<PlayerStats>,
    mut orb_query: Query<(Entity, &mut Collider, &mut Transform, &mut Orb)>,
    player_query: Query<(&Player, &GlobalTransform)>,
) {
    let orb_count = orb_query.iter().count();
    let expected_count = stats.get_amount(stats.extra_orbs + constants::ORB_BASE_AMOUNT);

    if (orb_count as u32) < expected_count {
        // de-spawn existing orbs so we can spawn new ones the correct distance apart
        for e in orb_query.iter().map(|(e, _, _, _)| e) {
            commands.entity(e).despawn_recursive();
        }

        // spawn new orbs
        let angle_spacer = TAU / (expected_count as f32);
        for index in 0..expected_count {
            // get the players position and axis so we can position the orb appropriately
            let (player, player_transform) = player_query.single();
            let camera_up = player.up.normalize();

            // spawn a new orb
            let orb_angle = (index as f32) * angle_spacer;
            commands.spawn(OrbBundle::new(
                get_orb_transform(
                    player_transform.translation(),
                    camera_up,
                    orb_angle,
                    stats.get_attack_size(constants::ORB_RADIUS),
                ),
                stats.get_damage(constants::ORB_BASE_DAMAGE),
                orb_angle,
            ));
        }
    } else {
        // update existing orbs and count them
        for (_, mut collider, mut transform, mut orb) in orb_query.iter_mut() {
            // update the collider size
            *collider = Collider::Sphere(stats.get_attack_size(constants::ORB_RADIUS));
            // update the transform scale
            transform.scale = Vec3::splat(stats.get_attack_size(constants::ORB_RADIUS));
            // update the orb
            orb.damage = stats.get_damage(constants::ORB_BASE_DAMAGE);
        }
    }
}

fn get_orb_transform(player_pos: Vec3, camera_up: Vec3, angle: f32, radius: f32) -> Transform {
    // define the initial transform
    let mut new_transform =
        Transform::from_translation(player_pos + camera_up * constants::ORB_ORBIT_RADIUS);

    // rotate the orb around the player's axis
    let towards_camera = player_pos.normalize();
    new_transform.rotate_around(player_pos, Quat::from_axis_angle(towards_camera, angle));

    // normalize the position so its the correct distance from the center from the world
    new_transform.translation =
        new_transform.translation.normalize() * (constants::PLANET_RADIUS + radius);

    // set the scale
    new_transform.scale = Vec3::splat(radius);

    new_transform
}
