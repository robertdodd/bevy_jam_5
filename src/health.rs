use bevy::prelude::*;

use crate::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeathEvent>()
            .add_systems(Update, handle_health_changed.run_if(in_game));
    }
}

#[derive(Event, Debug, Reflect)]
#[reflect(Debug)]
pub struct DeathEvent(pub Entity);

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct Health {
    pub current: f32,
    pub max_health: f32,
}

impl Health {
    pub fn percent(&self) -> f32 {
        self.current / self.max_health * 100.
    }
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.,
            max_health: 100.,
        }
    }
}

fn handle_health_changed(
    query: Query<(Entity, &Health), Changed<Health>>,
    mut death_writer: EventWriter<DeathEvent>,
) {
    for (entity, health) in query.iter() {
        if health.current <= 0. {
            death_writer.send(DeathEvent(entity));
        }
    }
}
