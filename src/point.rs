use bevy::prelude::*;

use crate::*;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PointResources>().add_systems(
            Update,
            (
                setup_new_points,
                handle_collision_events.run_if(on_event::<CollisionEvent>()),
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct PointResources {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
}

impl PointResources {
    pub fn get_or_create_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        if let Some(ref material) = self.material {
            material.clone()
        } else {
            let material = materials.add(StandardMaterial {
                emissive: LinearRgba::rgb(1., 1., 13.99),
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
            let mesh = meshes.add(Sphere::new(constants::POINT_RADIUS));
            self.mesh = Some(mesh.clone());
            mesh
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Debug)]
pub struct Point;

#[derive(Bundle)]
pub struct PointBundle {
    pub name: Name,
    pub point: Point,
    pub state_scoped: StateScoped<AppState>,
    pub transform: Transform,
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    pub attractable: Attractable,
}

impl PointBundle {
    pub fn new(pos: Vec3) -> Self {
        let position = pos.normalize() * (constants::PLANET_RADIUS + constants::POINT_RADIUS);
        Self {
            name: Name::new("Point"),
            point: Point,
            state_scoped: StateScoped(AppState::Game),
            transform: Transform::from_translation(position),
            collider: Collider::Sphere(constants::POINT_RADIUS),
            collision_groups: CollisionGroups::new(GROUP_POINT, GROUP_PLAYER),
            attractable: Attractable,
        }
    }
}

fn setup_new_points(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Added<Point>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player_resources: ResMut<PointResources>,
) {
    for (entity, transform) in query.iter() {
        commands.entity(entity).insert(MaterialMeshBundle {
            mesh: player_resources.get_or_create_mesh(&mut meshes),
            material: player_resources.get_or_create_material(&mut materials),
            transform: *transform,
            ..default()
        });
    }
}

fn handle_collision_events(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    point_query: Query<&Point>,
    player_query: Query<&mut Player>,
    mut score: ResMut<PlayerScore>,
) {
    for event in events.read() {
        let entity_pairs = [(event.e1, event.e2), (event.e2, event.e1)];
        for (player_entity, point_entity) in entity_pairs {
            if let (Ok(_), Ok(_)) = (
                player_query.get(player_entity),
                point_query.get(point_entity),
            ) {
                score.add_points(1);
                commands.entity(point_entity).despawn_recursive();
            }
        }
    }
}
