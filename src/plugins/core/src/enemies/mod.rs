use bevy::prelude::*;

pub mod ant;
pub mod creature;
pub mod handlers;

use ant::AntPlugin;
use creature::CreaturePlugin;

pub struct EnemiesPlugin;
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
        .add_plugins(AntPlugin)
        .add_plugins(CreaturePlugin);
    }
}
