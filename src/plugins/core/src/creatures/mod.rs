use bevy::prelude::*;

pub mod ant;
pub mod creature;
pub mod handlers;

use ant::AntPlugin;
use creature::CreatureMovementPlugin;

pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
        .add_plugins(AntPlugin)
        .add_plugins(CreatureMovementPlugin);
    }
}
