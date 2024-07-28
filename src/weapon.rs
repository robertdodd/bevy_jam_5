use std::time::Duration;

use bevy::{color::palettes::css, prelude::*};

use crate::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeaponResources>()
            .add_event::<FireWeapon>()
            .add_systems(
                Update,
                (
                    setup_new_weapons,
                    (fire_weapons.run_if(not_paused), handle_fire_events).chain(),
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(Event, Debug)]
pub struct FireWeapon;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, Reflect)]
#[reflect(Default, Debug, PartialEq, Hash)]
pub enum WeaponState {
    Attack,
    #[default]
    Cooldown,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Debug)]
pub struct Weapon {
    /// State
    pub state: WeaponState,
    /// Cooldown timer
    pub cooldown_timer: Timer,
    /// Fire timer
    pub fire_timer: Timer,
    /// Fire count
    pub fire_count: u32,
}

#[derive(Bundle)]
pub struct WeaponBundle {
    pub name: Name,
    pub weapon: Weapon,
    pub state_scoped: StateScoped<AppState>,
    pub spatial_bundle: SpatialBundle,
}

impl WeaponBundle {
    pub fn new() -> Self {
        // define the weapon position. NOTE: it is parented to a player.
        let pos = Vec3::new(
            constants::PLAYER_SIZE,
            constants::PLAYER_SIZE,
            constants::PLAYER_SIZE,
        );
        Self {
            name: Name::new("Projectile"),
            weapon: Weapon {
                cooldown_timer: Timer::from_seconds(
                    constants::PLAYER_DEFAULT_COOLDOWN,
                    TimerMode::Repeating,
                ),
                fire_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                fire_count: 0,
                state: WeaponState::Cooldown,
            },
            state_scoped: StateScoped(AppState::Game),
            spatial_bundle: SpatialBundle::from_transform(Transform::from_translation(pos)),
        }
    }
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct WeaponResources {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
}

impl WeaponResources {
    pub fn get_or_create_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        if let Some(ref material) = self.material {
            material.clone()
        } else {
            let material = materials.add(StandardMaterial {
                base_color: css::DARK_GRAY.into(),
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
                constants::WEAPON_THICKNESS,
                constants::WEAPON_LENGTH,
                constants::WEAPON_THICKNESS,
            )));
            self.mesh = Some(mesh.clone());
            mesh
        }
    }
}

fn setup_new_weapons(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut Weapon), Added<Weapon>>,
    mut weapon_resource: ResMut<WeaponResources>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    stats: Res<PlayerStats>,
) {
    for (entity, transform, mut weapon) in query.iter_mut() {
        // Update weapon stats according to player stats
        weapon
            .cooldown_timer
            .set_duration(Duration::from_secs_f32(stats.attack_cooldown));

        let material = weapon_resource.get_or_create_material(&mut materials);
        let mesh = weapon_resource.get_or_create_mesh(&mut meshes);

        commands.entity(entity).insert((
            Name::new("Weapon"),
            MaterialMeshBundle {
                material,
                mesh,
                transform: *transform,
                ..default()
            },
        ));
    }
}

fn fire_weapons(
    time: Res<Time>,
    mut query: Query<&mut Weapon>,
    mut fire_writer: EventWriter<FireWeapon>,
    stats: Res<PlayerStats>,
) {
    for mut weapon in query.iter_mut() {
        match weapon.state {
            WeaponState::Attack => {
                if weapon.fire_count == 0 || weapon.fire_timer.finished() {
                    weapon.fire_count += 1;
                    weapon.fire_timer.reset();
                    fire_writer.send(FireWeapon);
                    if weapon.fire_count >= stats.attack_amount {
                        weapon.state = WeaponState::Cooldown;
                        weapon.fire_timer.reset();
                        weapon.fire_count = 0;
                    }
                } else {
                    weapon.fire_timer.tick(time.delta());
                }
            }
            WeaponState::Cooldown => {
                weapon.cooldown_timer.tick(time.delta());
                if weapon.cooldown_timer.finished() {
                    weapon.state = WeaponState::Attack;
                }
            }
        }
    }
}

fn handle_fire_events(
    mut events: EventReader<FireWeapon>,
    mut commands: Commands,
    player_query: Query<(&Player, &GlobalTransform)>,
    stats: Res<PlayerStats>,
) {
    for _ in events.read() {
        let (player, player_transform) = player_query.single();

        let camera_up = player.up.normalize();
        let towards_camera = player_transform.translation().normalize();
        let camera_right = camera_up.cross(towards_camera).normalize();

        let axes = [
            (camera_up + camera_right) / 2.,
            (-camera_up + camera_right) / 2.,
            (camera_up - camera_right) / 2.,
            (-camera_up - camera_right) / 2.,
        ];
        for axis in axes {
            commands.spawn(ProjectileBundle::new(
                player_transform.translation(),
                axis,
                1.,
                stats.attack_damage,
                stats.attack_size_frac(),
            ));
        }
    }
}
