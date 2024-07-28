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

    pub attack_damage: f32,
    pub attack_size: f32,
    pub attack_cooldown: f32,
    pub attack_amount: u32,

    pub pickup_radius: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            max_health: 100.,
            recovery: constants::PLAYER_DEFAULT_RECOVERY,
            armor: 10.,
            move_speed: constants::PLAYER_DEFAULT_SPEED,
            attack_damage: constants::PLAYER_DEFAULT_DAMAGE,
            attack_size: 100.,
            attack_cooldown: constants::PLAYER_DEFAULT_COOLDOWN,
            attack_amount: constants::PLAYER_DEFAULT_AMOUNT,
            pickup_radius: constants::PLAYER_DEFAULT_ATTRACTOR_RADIUS,
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
                self.attack_damage = power_up.value.add_f32(self.attack_damage);
            }
            Stat::AtackSize => {
                self.attack_size = power_up.value.add_f32(self.attack_size);
            }
            Stat::AttackCooldown => {
                self.attack_cooldown = power_up.value.add_f32(self.attack_cooldown);
            }
            Stat::AttackAmount => {
                self.attack_amount = power_up.value.add_u32(self.attack_amount);
            }
            Stat::PickupRadius => {
                self.pickup_radius = power_up.value.add_f32(self.pickup_radius);
            }
        }
    }

    pub fn attack_size_frac(&self) -> f32 {
        self.attack_size / 100.
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
        }
    }
}

impl Stat {
    const ALL: [Self; 9] = [
        Self::MaxHealth,
        Self::Recovery,
        Self::Armor,
        Self::MoveSpeed,
        Self::AttachDamage,
        Self::AtackSize,
        Self::AttackCooldown,
        Self::AttackAmount,
        Self::PickupRadius,
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
                    PowerUpValue::Percent(5),
                    PowerUpValue::Percent(10),
                    PowerUpValue::Percent(20),
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
