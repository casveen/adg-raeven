use bevy::prelude::*;

pub mod ant;

pub struct EnemiesPlugin;
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, ant::startup)
            // .add_systems(Update, ant::read_collision)
            .add_systems(Update, ant::ant_collision)
            .init_gizmo_group::<ant::CollisionGizmo>()
            .add_observer(ant::spawn_ant)
            .add_observer(ant::observe_cordyceptmovement);
    }
}
