use bevy::prelude::*;

pub mod ant;

use ant::AntPlugin;

pub struct EnemiesPlugin;
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(AntPlugin);
    }
}
