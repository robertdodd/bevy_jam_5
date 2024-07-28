use bevy::prelude::*;

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
}

impl Projectile {
    pub fn new(damage: f32, radius: f32) -> Self {
        assert!(damage >= 0.);
        assert!(radius >= 0.);
        Self { damage, radius }
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
    pub fn new(pos: Vec3, axis: Vec3, speed: f32, damage: f32, size_frac: f32) -> Self {
        let height = constants::PLANET_RADIUS
            + constants::PROJECTILE_HEIGHT
            + constants::PROJECTILE_RADIUS * size_frac;
        let normalized_pos = pos.normalize() * height;

        Self {
            name: Name::new("Projectile"),
            projectile: Projectile::new(damage, constants::PROJECTILE_RADIUS * size_frac),
            velocity: Velocity {
                pos: height,
                axis,
                speed,
            },
            state_scoped: StateScoped(AppState::Game),
            transform: Transform::from_translation(normalized_pos)
                .with_scale(Vec3::splat(size_frac)),
            collider: Collider::Sphere(constants::PROJECTILE_RADIUS * size_frac),
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
            let mesh = meshes.add(Sphere::new(constants::PROJECTILE_RADIUS));
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
    mut events: EventReader<CollisionEvent>,
    projectile_query: Query<&Projectile, With<Projectile>>,
    mut health_query: Query<&mut Health>,
) {
    for event in events.read() {
        let entity_pairs = [(event.e1, event.e2), (event.e2, event.e1)];
        for (e1, e2) in entity_pairs {
            if let (Ok(mut health), Ok(projectile)) =
                (health_query.get_mut(e1), projectile_query.get(e2))
            {
                health.current -= projectile.damage;
            }
        }
    }
}
