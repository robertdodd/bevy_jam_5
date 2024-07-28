use bevy::{prelude::*, utils::HashSet};

use crate::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProjectileResources>().add_systems(
            Update,
            (
                setup_new_projectiles,
                handle_collision_events.run_if(on_event::<CollisionEvent>()),
            )
                .run_if(in_game),
        );
    }
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct Projectile {
    pub damage: f32,
    pub radius: f32,
    pub passthrough_count: u32,
    pub max_passthrough: u32,
    pub hits: HashSet<Entity>,
}

impl Projectile {
    pub fn new(damage: f32, radius: f32, max_passthrough: u32) -> Self {
        assert!(damage >= 0.);
        assert!(radius >= 0.);
        Self {
            damage,
            radius,
            max_passthrough,
            passthrough_count: 0,
            hits: HashSet::<Entity>::new(),
        }
    }
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub name: Name,
    pub projectile: Projectile,
    pub velocity: Velocity,
    pub state_scoped: StateScoped<AppState>,
    pub transform: Transform,
    pub collider: Collider,
    pub collision_group: CollisionGroups,
    pub lifetime: Lifetime,
}

impl ProjectileBundle {
    pub fn new(
        pos: Vec3,
        axis: Vec3,
        speed: f32,
        damage: f32,
        radius: f32,
        max_passthrough: u32,
    ) -> Self {
        let height = constants::PLANET_RADIUS + constants::PROJECTILE_HEIGHT + radius;
        let normalized_pos = pos.normalize() * height;

        Self {
            name: Name::new("Projectile"),
            projectile: Projectile::new(damage, radius, max_passthrough),
            velocity: Velocity {
                pos: height,
                axis,
                speed,
            },
            state_scoped: StateScoped(AppState::Game),
            transform: Transform::from_translation(normalized_pos).with_scale(Vec3::splat(radius)),
            collider: Collider::Sphere(radius),
            collision_group: CollisionGroups::new(GROUP_PROJECTILE, GROUP_ENEMY),
            lifetime: Lifetime::from_seconds(1.),
        }
    }
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct ProjectileResources {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
}

impl ProjectileResources {
    pub fn get_or_create_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        if let Some(ref material) = self.material {
            material.clone()
        } else {
            let material = materials.add(StandardMaterial {
                emissive: LinearRgba::rgb(13.99, 0.0, 0.0),
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

fn setup_new_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Added<Projectile>>,
    mut weapon_resource: ResMut<ProjectileResources>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, transform) in query.iter() {
        let material = weapon_resource.get_or_create_material(&mut materials);
        let mesh = weapon_resource.get_or_create_mesh(&mut meshes);

        commands.entity(entity).insert((
            Name::new("Projectile"),
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
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut projectile_query: Query<&mut Projectile, With<Projectile>>,
    mut health_query: Query<&mut Health>,
) {
    let mut to_despawn = HashSet::<Entity>::new();
    for event in events.read() {
        let entity_pairs = [(event.e1, event.e2), (event.e2, event.e1)];
        for (health_entity, projectile_entity) in entity_pairs {
            if let (Ok(mut health), Ok(mut projectile)) = (
                health_query.get_mut(health_entity),
                projectile_query.get_mut(projectile_entity),
            ) {
                // ignore if marked to de-spawn
                if to_despawn.contains(&projectile_entity) {
                    continue;
                }
                // ignore if projectile has already hit this entity
                if projectile.hits.contains(&health_entity) {
                    continue;
                }
                health.current -= projectile.damage;
                projectile.passthrough_count += 1;
                projectile.hits.insert(health_entity);
                if projectile.passthrough_count >= projectile.max_passthrough {
                    to_despawn.insert(projectile_entity);
                }
            }
        }
    }

    for entity in to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
