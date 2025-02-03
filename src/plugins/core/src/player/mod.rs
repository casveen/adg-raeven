/**
 * Module for all player related plugins, types and systems
 *
 * PlayerController as input interpreter and main entry point to the module, which
 * then sends Trigger<_,_> that the other audio/visual systems will add_observer for.
 */
pub mod controller;
pub mod visuals;
pub mod states;

use controller::PlayerControllerPlugin;
use visuals::VisualsPlugin;
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerControllerPlugin, VisualsPlugin));
    }
}
