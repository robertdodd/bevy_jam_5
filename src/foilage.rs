use bevy::{color::palettes::css, prelude::*};
use rand::{distributions::Uniform, Rng};

use crate::{constants, game_state::*};

pub struct FoilagePlugin;

impl Plugin for FoilagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FoilageResources>()
            .add_systems(OnEnter(AppState::Game), spawn_foilage);
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Debug)]
pub struct Foilage;

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct FoilageResources {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
}

impl FoilageResources {
    pub fn get_or_create_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        if let Some(ref material) = self.material {
            material.clone()
        } else {
            let material = materials.add(StandardMaterial {
                base_color: css::GREEN.into(),
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
            let mesh = meshes.add(Cuboid::from_size(Vec3::new(
                1.,
                1.,
                constants::FOILAGE_HEIGHT,
            )));
            self.mesh = Some(mesh.clone());
            mesh
        }
    }
}

fn spawn_foilage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut foilage_resources: ResMut<FoilageResources>,
) {
    let start = Vec3::new(0., 0., constants::PLANET_RADIUS);
    let mesh = foilage_resources.get_or_create_mesh(&mut meshes);
    let material = foilage_resources.get_or_create_material(&mut materials);

    let between = Uniform::from(0..=100);
    let mut rng = rand::thread_rng();

    for _ in 0..constants::FOILAGE_COUNT {
        let mut pos = Vec3::new(
            (rng.sample(between) - 50) as f32 / 100.,
            (rng.sample(between) - 50) as f32 / 100.,
            (rng.sample(between) - 50) as f32 / 100.,
        );
        pos = pos.normalize() * constants::PLANET_RADIUS;

        commands.spawn((
            Name::new("Foilage"),
            StateScoped(AppState::Game),
            Foilage,
            MaterialMeshBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(pos)
                    .with_rotation(Quat::from_rotation_arc(start.normalize(), pos.normalize())),
                ..default()
            },
        ));
    }
}
