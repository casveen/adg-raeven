use bevy::prelude::*;

pub mod ant;
pub mod movement;
use ant::AntPlugin;
use movement::creature_movement::CreatureMovementPlugin;

pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
        .add_plugins(AntPlugin)
        .add_plugins(CreatureMovementPlugin);
    }
}
