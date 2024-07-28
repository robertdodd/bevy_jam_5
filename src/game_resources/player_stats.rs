use std::fmt;

use bevy::prelude::*;
use rand::{distributions::Uniform, Rng};

use crate::constants;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct PlayerStats {
    pub max_health: f32,
    pub recovery: f32,
    pub armor: f32,

    pub move_speed: f32,

    pub damage_percent: f32,
    pub attack_size_percent: f32,
    pub attack_cooldown: f32,
    pub attack_amount_extra: u32,
    pub projectile_passthrough: u32,
    pub projectile_speed: f32,

    pub extra_orbs: u32,

    pub pickup_radius: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            max_health: 100.,
            recovery: constants::PLAYER_DEFAULT_RECOVERY,
            armor: 10.,
            move_speed: constants::PLAYER_DEFAULT_SPEED,
            damage_percent: 100.,
            attack_size_percent: 100.,
            attack_cooldown: constants::PLAYER_DEFAULT_COOLDOWN,
            attack_amount_extra: 0,
            pickup_radius: constants::PLAYER_DEFAULT_ATTRACTOR_RADIUS,
            projectile_passthrough: 1,
            extra_orbs: 0,
            projectile_speed: 100.,
        }
    }
}

impl PlayerStats {
    pub fn add_power_up(&mut self, power_up: &PowerUp) {
        match power_up.stat {
            Stat::MaxHealth => {
                self.max_health = power_up.value.add_f32(self.max_health);
            }
            Stat::Recovery => {
                self.recovery = power_up.value.add_f32(self.recovery);
            }
            Stat::Armor => {
                self.armor = power_up.value.add_f32(self.armor);
            }
            Stat::MoveSpeed => {
                self.move_speed = power_up.value.add_f32(self.move_speed);
            }
            Stat::AttachDamage => {
                self.damage_percent = power_up.value.add_f32(self.damage_percent);
            }
            Stat::AtackSize => {
                self.attack_size_percent = power_up.value.add_f32(self.attack_size_percent);
            }
            Stat::AttackCooldown => {
                self.attack_cooldown = power_up.value.add_f32(self.attack_cooldown);
            }
            Stat::AttackAmount => {
                self.attack_amount_extra = power_up.value.add_u32(self.attack_amount_extra);
            }
            Stat::PickupRadius => {
                self.pickup_radius = power_up.value.add_f32(self.pickup_radius);
            }
            Stat::ProjectilePassthrough => {
                self.projectile_passthrough = power_up.value.add_u32(self.projectile_passthrough);
            }
            Stat::OrbCount => {
                self.extra_orbs = power_up.value.add_u32(self.extra_orbs);
            }
            Stat::ProjectileSpeed => {
                self.projectile_speed = power_up.value.add_f32(self.projectile_speed);
            }
        }
    }

    pub fn get_attack_size(&self, size: f32) -> f32 {
        size * self.attack_size_percent / 100.
    }

    pub fn get_damage(&self, damage: f32) -> f32 {
        damage * self.damage_percent / 100.
    }

    pub fn get_amount(&self, amount: u32) -> u32 {
        amount + self.attack_amount_extra
    }

    pub fn get_attack_speed(&self, speed: f32) -> f32 {
        speed * self.projectile_speed / 100.
    }
}

#[derive(Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct PowerUp {
    pub stat: Stat,
    pub value: PowerUpValue,
}

impl PowerUp {
    pub fn description(&self) -> String {
        let prefix = if self.value.value() > 0 {
            "Increase"
        } else {
            "Decrease"
        };
        format!("{} {} by {}", prefix, self.stat, self.value)
    }

    pub fn new_random() -> Self {
        let stat = Stat::new_random();
        let options = stat.get_random_range();
        let value = if options.len() == 1 {
            options[0]
        } else {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..options.len());
            options[index]
        };
        Self { stat, value }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Reflect)]
#[reflect(Debug, PartialEq)]
pub enum PowerUpValue {
    Percent(i32),
    Amount(i32),
}

impl PowerUpValue {
    pub fn value(&self) -> i32 {
        match self {
            PowerUpValue::Percent(val) => *val,
            PowerUpValue::Amount(val) => *val,
        }
    }

    pub fn add_f32(&self, value: f32) -> f32 {
        match self {
            PowerUpValue::Percent(val) => value + (value * *val as f32 / 100.),
            PowerUpValue::Amount(val) => value + *val as f32,
        }
    }

    pub fn add_u32(&self, value: u32) -> u32 {
        match self {
            PowerUpValue::Percent(val) => value + (value as f32 * *val as f32 / 100.) as u32,
            PowerUpValue::Amount(val) => value + *val as u32,
        }
    }
}

impl fmt::Display for PowerUpValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerUpValue::Percent(val) => write!(f, "{}%", *val),
            PowerUpValue::Amount(val) => write!(f, "{}", *val),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub enum Stat {
    MaxHealth,
    Recovery,
    Armor,
    MoveSpeed,
    AttachDamage,
    AtackSize,
    AttackCooldown,
    AttackAmount,
    PickupRadius,
    ProjectilePassthrough,
    OrbCount,
    ProjectileSpeed,
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stat::MaxHealth => write!(f, "Max Health"),
            Stat::Recovery => write!(f, "Recovery"),
            Stat::Armor => write!(f, "Armor"),
            Stat::MoveSpeed => write!(f, "Move Speed"),
            Stat::AttachDamage => write!(f, "Damage"),
            Stat::AtackSize => write!(f, "Projectile Size"),
            Stat::AttackCooldown => write!(f, "Weapon Cooldown"),
            Stat::AttackAmount => write!(f, "Attack Amount"),
            Stat::PickupRadius => write!(f, "Pickup Radius"),
            Stat::ProjectilePassthrough => write!(f, "Projectile Passthrough"),
            Stat::OrbCount => write!(f, "Orb"),
            Stat::ProjectileSpeed => write!(f, "Projectile Speed"),
        }
    }
}

impl Stat {
    const ALL: [Self; 12] = [
        Self::MaxHealth,
        Self::Recovery,
        Self::Armor,
        Self::MoveSpeed,
        Self::AttachDamage,
        Self::AtackSize,
        Self::AttackCooldown,
        Self::AttackAmount,
        Self::PickupRadius,
        Self::ProjectilePassthrough,
        Self::OrbCount,
        Self::ProjectileSpeed,
    ];

    pub fn get_random_range(&self) -> Vec<PowerUpValue> {
        match self {
            Stat::MaxHealth => {
                vec![
                    PowerUpValue::Percent(5),
                    PowerUpValue::Percent(10),
                    PowerUpValue::Percent(20),
                ]
            }
            Stat::Recovery => {
                vec![
                    PowerUpValue::Percent(5),
                    PowerUpValue::Percent(10),
                    PowerUpValue::Percent(20),
                ]
            }
            Stat::Armor => {
                vec![
                    PowerUpValue::Percent(5),
                    PowerUpValue::Percent(10),
                    PowerUpValue::Percent(20),
                ]
            }
            Stat::MoveSpeed => {
                vec![PowerUpValue::Percent(5), PowerUpValue::Percent(10)]
            }
            Stat::AttachDamage => {
                vec![
                    PowerUpValue::Percent(5),
                    PowerUpValue::Percent(10),
                    PowerUpValue::Percent(20),
                    PowerUpValue::Amount(10),
                ]
            }
            Stat::AtackSize => {
                vec![
                    PowerUpValue::Percent(5),
                    PowerUpValue::Percent(10),
                    PowerUpValue::Percent(20),
                    PowerUpValue::Amount(10),
                ]
            }
            Stat::AttackCooldown => {
                vec![
                    PowerUpValue::Percent(-5),
                    PowerUpValue::Percent(-10),
                    PowerUpValue::Percent(-20),
                ]
            }
            Stat::AttackAmount => {
                vec![PowerUpValue::Amount(1)]
            }
            Stat::PickupRadius => {
                vec![
                    PowerUpValue::Percent(10),
                    PowerUpValue::Percent(20),
                    PowerUpValue::Percent(30),
                ]
            }
            Stat::ProjectilePassthrough => {
                vec![PowerUpValue::Amount(1)]
            }
            Stat::OrbCount => {
                vec![PowerUpValue::Amount(1)]
            }
            Stat::ProjectileSpeed => {
                vec![
                    PowerUpValue::Percent(5),
                    PowerUpValue::Percent(10),
                    PowerUpValue::Percent(20),
                    PowerUpValue::Percent(40),
                ]
            }
        }
    }

    pub fn new_random() -> Self {
        let between = Uniform::from(0..Self::ALL.len());
        let mut rng = rand::thread_rng();
        Self::ALL[rng.sample(between)]
    }
}
