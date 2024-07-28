use std::f32::consts::{FRAC_PI_4, FRAC_PI_8};

use bevy::{color::palettes::css, prelude::*};

use crate::*;

use self::math::get_angle_for_arc_length;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyResources>()
            .add_event::<SpawnEnemies>()
            .add_systems(
                Update,
                (
                    setup_new_enemies,
                    handle_spawn_events.run_if(on_event::<SpawnEnemies>()),
                    (move_enemies, attack_players).run_if(not_paused),
                    handle_death_events.run_if(on_event::<DeathEvent>()),
                )
                    .run_if(in_game),
            );
    }
}

#[derive(Event, Default, Debug, Reflect)]
#[reflect(Default, Debug)]
pub struct SpawnEnemies;

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct Enemy {
    damage: f32,
    cooldown_timer: Timer,
    speed: f32,
    has_attacked: bool,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            damage: 10.,
            cooldown_timer: Timer::from_seconds(1., TimerMode::Once),
            speed: constants::ENEMY_MOVEMENT_SPEED,
            has_attacked: false,
        }
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub name: Name,
    pub enemy: Enemy,
    pub state_scoped: StateScoped<AppState>,
    pub spatial_bundle: SpatialBundle,
    pub health: Health,
    pub collider: Collider,
    pub collision_group: CollisionGroups,
}

impl EnemyBundle {
    pub fn new(pos: Vec3, movement_speed: f32, damage: f32) -> Self {
        Self {
            name: Name::new("Enemy"),
            enemy: Enemy {
                speed: movement_speed,
                damage,
                ..default()
            },
            state_scoped: StateScoped(AppState::Game),
            spatial_bundle: SpatialBundle::from_transform(Transform::from_translation(pos)),
            health: Health::default(),
            collider: Collider::Cuboid(Vec3::splat(constants::ENEMY_SIZE)),
            collision_group: CollisionGroups::new(GROUP_ENEMY, GROUP_PLAYER | GROUP_PROJECTILE),
        }
    }
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct EnemyResources {
    pub enemy_mesh: Option<Handle<Mesh>>,
    pub enemy_material: Option<Handle<StandardMaterial>>,
}

impl EnemyResources {
    pub fn get_or_create_enemy_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        if let Some(ref material) = self.enemy_material {
            material.clone()
        } else {
            let material = materials.add(StandardMaterial {
                base_color: css::RED.into(),
                ..default()
            });
            self.enemy_material = Some(material.clone());
            material
        }
    }

    pub fn get_or_create_enemy_mesh(&mut self, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        if let Some(ref mesh) = self.enemy_mesh {
            mesh.clone()
        } else {
            let mesh = meshes.add(Cuboid::from_length(constants::ENEMY_SIZE));
            self.enemy_mesh = Some(mesh.clone());
            mesh
        }
    }
}

fn handle_spawn_events(
    mut events: EventReader<SpawnEnemies>,
    mut commands: Commands,
    player_query: Query<(&Player, &Transform)>,
    enemy_stats: Res<EnemyStats>,
) {
    let (player, player_transform) = player_query.single();

    // compute horizontal axis (cross product)
    let camera_up = player.up.normalize();
    let towards_camera = player_transform.translation.normalize();

    // define how far from the center the enemy should spawn
    let enemy_up_pos = constants::PLANET_RADIUS + constants::ENEMY_SIZE / 2.;

    for _ in events.read() {
        // Spawn players at fixed points along the circumference of the circle
        let mut spawn_transform = Transform::from_translation(camera_up * enemy_up_pos);
        for n in 0..8 {
            if n != 0 {
                let rot = Quat::from_axis_angle(towards_camera, (n as f32 * FRAC_PI_4) + FRAC_PI_8);
                spawn_transform.rotate_around(Vec3::ZERO, rot);
            }

            let pos = spawn_transform.translation;

            commands.spawn(EnemyBundle::new(
                pos,
                enemy_stats.movement_speed,
                enemy_stats.damage,
            ));
        }
    }
}

fn setup_new_enemies(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Added<Enemy>>,
    mut enemy_resource: ResMut<EnemyResources>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, transform) in query.iter() {
        let material = enemy_resource.get_or_create_enemy_material(&mut materials);
        let mesh = enemy_resource.get_or_create_enemy_mesh(&mut meshes);

        commands.entity(entity).insert((
            Name::new("Enemy"),
            MaterialMeshBundle {
                material,
                mesh,
                transform: *transform,
                ..default()
            },
        ));
    }
}

fn move_enemies(
    time: Res<Time>,
    mut query: Query<(&Enemy, &mut Transform)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();

    for (enemy, mut transform) in query.iter_mut() {
        // skip if close enough to player
        let dist = transform.translation.distance(player_transform.translation);
        if dist < constants::PLAYER_SIZE / 2. + constants::ENEMY_SIZE / 2. {
            continue;
        }

        // get rotation towards player
        let target_rot = Quat::from_rotation_arc(
            transform.translation.normalize(),
            player_transform.translation.normalize(),
        );
        let (axis, angle) = target_rot.to_axis_angle();

        // compute angle for movement speed
        let speed_angle = get_angle_for_arc_length(
            enemy.speed,
            constants::PLANET_RADIUS + constants::ENEMY_SIZE / 2.,
        ) * time.delta_seconds();

        let rot = Quat::from_axis_angle(axis, speed_angle * angle.signum() * 0.5);
        transform.rotate_around(Vec3::ZERO, rot)
    }
}

fn attack_players(
    time: Res<Time>,
    mut query: Query<(&mut Enemy, &Transform)>,
    mut player_query: Query<(&mut Health, &Transform), (With<Player>, Without<Enemy>)>,
) {
    let (mut player_health, player_transform) = player_query.single_mut();

    for (mut enemy, transform) in query.iter_mut() {
        enemy.cooldown_timer.tick(time.delta());

        // skip if not close enough to player
        let dist = transform.translation.distance(player_transform.translation);
        let attack_dist = constants::PLAYER_SIZE / 2. + constants::ENEMY_SIZE / 2.;
        if dist > attack_dist {
            continue;
        }

        if !enemy.has_attacked || enemy.cooldown_timer.finished() {
            enemy.has_attacked = true;
            enemy.cooldown_timer.reset();
            player_health.current -= enemy.damage;
        }
    }
}

fn handle_death_events(
    mut commands: Commands,
    mut events: EventReader<DeathEvent>,
    query: Query<&GlobalTransform, With<Enemy>>,
    mut score: ResMut<PlayerScore>,
) {
    for event in events.read() {
        if let Ok(transform) = query.get(event.0) {
            // spawn a point bundle
            commands.spawn(PointBundle::new(transform.translation()));
            // track enemies killed in score
            score.add_enemy_killed();
            // de-spawn the enemy
            commands.entity(event.0).despawn_recursive();
        }
    }
}
