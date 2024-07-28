use bevy::prelude::*;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct PlayerScore {
    /// The players current level
    pub level: u32,

    /// The number of points needed to reach the next level
    pub next_level: u32,

    /// The players current number of points counting towards the next level
    pub current_points: u32,

    /// Total number of points collected throughout the run
    pub total_points: u32,

    /// Total number of enemies killed
    pub enemies_killed: u32,
}

impl Default for PlayerScore {
    fn default() -> Self {
        Self {
            level: 1,
            // TODO: RESTORE:
            // next_level: 10,
            next_level: 1,
            current_points: 0,
            total_points: 0,
            enemies_killed: 0,
        }
    }
}

impl PlayerScore {
    /// Adds points
    pub fn add_points(&mut self, points: u32) {
        self.total_points += points;
        self.current_points += points;
    }

    pub fn add_enemy_killed(&mut self) {
        self.enemies_killed += 1;
    }

    /// Levels up if the current points have reached a new level.
    /// Returns `true` if a new level was reached, `false` otherwise.
    pub fn get_level_up(&mut self) -> bool {
        if self.current_points >= self.next_level {
            self.current_points -= self.next_level;
            self.level += 1;
            self.next_level *= 2;
            true
        } else {
            false
        }
    }
}
