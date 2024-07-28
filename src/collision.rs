use bevy::{
    math::bounding::{Aabb3d, BoundingSphere, IntersectsVolume},
    prelude::*,
    utils::HashSet,
};

use crate::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(Update, check_collisions.run_if(in_game_not_paused));
    }
}

pub const GROUP_PLAYER: u16 = 1 << 0;
pub const GROUP_PROJECTILE: u16 = 1 << 1;
pub const GROUP_ENEMY: u16 = 1 << 2;
pub const GROUP_POINT: u16 = 1 << 3;

/// All of the groups.
#[allow(dead_code)]
pub const GROUP_ALL: u16 = u16::MAX;

/// None of the groups.
#[allow(dead_code)]
pub const GROUP_NONE: u16 = 0;

#[derive(Debug, Eq, PartialEq, Hash)]
struct CollisionPair {
    e1: Entity,
    e2: Entity,
}

impl CollisionPair {
    pub fn new(e1: Entity, e2: Entity) -> Self {
        if e1.to_bits() < e2.to_bits() {
            Self { e1, e2 }
        } else {
            Self { e1: e2, e2: e1 }
        }
    }
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct CollisionGroups {
    pub memberships: u16,
    pub filters: u16,
}

impl CollisionGroups {
    pub fn new(memberships: u16, filters: u16) -> Self {
        Self {
            memberships,
            filters,
        }
    }

    pub fn intersects(&self, rhs: &Self) -> bool {
        (self.memberships & rhs.filters) != 0 && (rhs.memberships & self.filters) != 0
    }
}

#[derive(Event)]
pub struct CollisionEvent {
    pub e1: Entity,
    pub e2: Entity,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub enum Collider {
    Sphere(f32),
    Cuboid(Vec3),
}

impl Default for Collider {
    fn default() -> Self {
        Self::Cuboid(Vec3::ONE)
    }
}

fn check_collisions(
    query: Query<(Entity, &Collider, &GlobalTransform, &CollisionGroups)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let mut handled = HashSet::<CollisionPair>::new();
    for (e1, c1, t1, g1) in query.iter() {
        for (e2, c2, t2, g2) in query.iter() {
            // ignore if same entity
            if e1 == e2 {
                continue;
            }

            // ignore if collision groups don't overlap
            if !g1.intersects(g2) {
                continue;
            }

            // define the collision pair
            let collision_pair = CollisionPair::new(e1, e2);

            // skip if collision pair already handled
            if handled.contains(&collision_pair) {
                continue;
            }

            let is_collision = match (c1, c2) {
                (Collider::Sphere(r1), Collider::Sphere(r2)) => {
                    let b1 = BoundingSphere::new(t1.translation(), *r1);
                    let b2 = BoundingSphere::new(t2.translation(), *r2);
                    b1.intersects(&b2)
                }
                (Collider::Sphere(r1), Collider::Cuboid(s2)) => {
                    let b1 = BoundingSphere::new(t1.translation(), *r1);
                    let b2 = Aabb3d::new(t2.translation(), *s2 / 2.);
                    b1.intersects(&b2)
                }
                (Collider::Cuboid(s1), Collider::Sphere(r2)) => {
                    let b1 = Aabb3d::new(t1.translation(), *s1 / 2.);
                    let b2 = BoundingSphere::new(t2.translation(), *r2);
                    b1.intersects(&b2)
                }
                (Collider::Cuboid(s1), Collider::Cuboid(s2)) => {
                    let b1 = Aabb3d::new(t1.translation(), *s1 / 2.);
                    let b2 = Aabb3d::new(t2.translation(), *s2 / 2.);
                    b1.intersects(&b2)
                }
            };
            if is_collision {
                handled.insert(collision_pair);
                collision_events.send(CollisionEvent { e1, e2 });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collision_groups_work() {
        struct TestCase {
            a: CollisionGroups,
            b: CollisionGroups,
            expected: bool,
        }
        let test_cases = [
            // Player projectiles and enenies should collide
            TestCase {
                a: CollisionGroups::new(GROUP_PROJECTILE, GROUP_ENEMY),
                b: CollisionGroups::new(GROUP_ENEMY, GROUP_PROJECTILE | GROUP_PLAYER),
                expected: true,
            },
            // Player projectiles and players should not collide
            TestCase {
                a: CollisionGroups::new(GROUP_PROJECTILE, GROUP_ENEMY),
                b: CollisionGroups::new(GROUP_PLAYER, GROUP_ENEMY),
                expected: false,
            },
            // Player projectiles should not collide with each other
            TestCase {
                a: CollisionGroups::new(GROUP_PROJECTILE, GROUP_ENEMY),
                b: CollisionGroups::new(GROUP_PROJECTILE, GROUP_ENEMY),
                expected: false,
            },
            // Player should collide with enemy
            TestCase {
                a: CollisionGroups::new(GROUP_PLAYER, GROUP_ENEMY),
                b: CollisionGroups::new(GROUP_ENEMY, GROUP_PROJECTILE | GROUP_PLAYER),
                expected: true,
            },
            // Enemies should not collide with each other
            TestCase {
                a: CollisionGroups::new(GROUP_ENEMY, GROUP_PROJECTILE | GROUP_PLAYER),
                b: CollisionGroups::new(GROUP_ENEMY, GROUP_PROJECTILE | GROUP_PLAYER),
                expected: false,
            },
        ];
        for test in test_cases {
            // test a with b
            let r1 = test.a.intersects(&test.b);
            assert_eq!(r1, test.expected);
            // test b with a gives same result
            let r2 = test.b.intersects(&test.a);
            assert_eq!(r2, test.expected);
        }
    }

    #[test]
    fn collision_pairs_work() {
        struct TestCase {
            name: String,
            a: CollisionPair,
            b: CollisionPair,
            equal: bool,
        }
        let test_cases = [
            TestCase {
                name: "same entities, same order".to_string(),
                a: CollisionPair::new(Entity::from_raw(1), Entity::from_raw(2)),
                b: CollisionPair::new(Entity::from_raw(1), Entity::from_raw(2)),
                equal: true,
            },
            TestCase {
                name: "same entities, different order".to_string(),
                a: CollisionPair::new(Entity::from_raw(1), Entity::from_raw(2)),
                b: CollisionPair::new(Entity::from_raw(2), Entity::from_raw(1)),
                equal: true,
            },
            TestCase {
                name: "totally different entities".to_string(),
                a: CollisionPair::new(Entity::from_raw(1), Entity::from_raw(2)),
                b: CollisionPair::new(Entity::from_raw(3), Entity::from_raw(4)),
                equal: false,
            },
            TestCase {
                name: "partially different entities".to_string(),
                a: CollisionPair::new(Entity::from_raw(1), Entity::from_raw(2)),
                b: CollisionPair::new(Entity::from_raw(1), Entity::from_raw(4)),
                equal: false,
            },
        ];
        for test in test_cases {
            let result = test.a == test.b;
            assert_eq!(result, test.equal, "{}", test.name);
        }
    }
}
