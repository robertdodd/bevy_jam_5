pub const APP_NAME: &str = "Game";

pub const PLANET_RADIUS: f32 = 20.;

pub const CAMERA_DISTANCE: f32 = 40.;

pub const ENEMY_SIZE: f32 = 1.;
pub const ENEMY_MOVEMENT_SPEED: f32 = 5.;
pub const ENEMY_BASE_DAMAGE: f32 = 10.;
pub const ENEMY_DEFAULT_MOB_SIZE: u32 = 2;
pub const ENEMY_DEFAULT_MOB_COUNT: u32 = 3;
pub const ENEMY_DEFAULT_SPAWN_SPEED: f32 = 3.;
pub const ENEMY_DEFAULT_HEALTH: f32 = 10.;

pub const PLAYER_SIZE: f32 = 1.;
pub const PLAYER_DEFAULT_SPEED: f32 = 5.;
pub const PLAYER_DEFAULT_ATTRACTOR_RADIUS: f32 = 5.;
pub const PLAYER_DEFAULT_DAMAGE: f32 = ENEMY_DEFAULT_HEALTH;
pub const PLAYER_DEFAULT_COOLDOWN: f32 = 1.;
pub const PLAYER_DEFAULT_RECOVERY: f32 = 1.;

pub const FOILAGE_COUNT: u16 = 400;
pub const FOILAGE_HEIGHT: f32 = 0.2;

pub const WEAPON_LENGTH: f32 = 0.2;
pub const WEAPON_THICKNESS: f32 = 0.5;
pub const WEAPON_BASE_AMOUNT: u32 = 3;

/// Radius of projectiles
pub const PROJECTILE_RADIUS: f32 = 0.2;
/// Height above the ground
pub const PROJECTILE_HEIGHT: f32 = 0.3;
/// Base speed
pub const PROJECTILE_BASE_SPEED: f32 = 1.;

/// Radius of orbs
pub const ORB_RADIUS: f32 = 0.5;
/// Orb cooldown before it can hit the same entity again
pub const ORB_COOLDOWN_SECS: f32 = 0.5;
/// Orbit speed
pub const ORB_MOVEMENT_SPEED: f32 = 10.;
/// How far from the player the orb orbits
pub const ORB_ORBIT_RADIUS: f32 = 4.;
/// Orb base damage
pub const ORB_BASE_DAMAGE: f32 = ENEMY_DEFAULT_HEALTH * 2.;
/// Orb base amount
pub const ORB_BASE_AMOUNT: u32 = 1;

/// Radius of points
pub const POINT_RADIUS: f32 = 0.25;

/// Speed of objects towards an attractor
pub const ATTRACTOR_SPEED: f32 = 20.;
