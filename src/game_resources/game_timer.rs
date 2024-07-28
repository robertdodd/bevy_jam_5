use bevy::{prelude::*, time::Stopwatch};

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct GameTimer(pub Stopwatch);

impl GameTimer {
    pub fn to_time_string(&self) -> String {
        let elapsed_secs = self.0.elapsed().as_secs();
        let seconds = elapsed_secs % 60;
        let minutes = (elapsed_secs / 60) % 60;
        let hours = (elapsed_secs / 60) / 60;

        if hours > 0 {
            format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
        } else {
            format!("{:0>2}:{:0>2}", minutes, seconds)
        }
    }
}
