use bevy::app::{Plugin, Startup};

pub mod ant;

pub struct EnemiesPlugin;
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, ant::startup)
            .add_observer(ant::spawn_ant)
            .add_observer(ant::observe_cordyceptmovement);
    }
}
