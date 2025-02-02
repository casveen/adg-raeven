/**
 * Module for all player related plugins, types and systems
 *
 * PlayerController as input interpreter and main entry point to the module, which
 * then sends Trigger<_,_> that the other audio/visual systems will add_observer for.
 */
pub mod controller;
pub mod visuals;
pub mod states;

use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerControllerPlugin, RenderPlugin));
    }
}

pub struct PlayerControllerPlugin;
impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                controller::register_input,
                //
            ),
        )
        .add_systems(Update, controller::process_input)
        .add_observer(controller::spawn_player);
    }
}

struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(visuals::spawn_player_mesh)
            .add_observer(visuals::setup_once_loaded)
            .add_observer(visuals::observe_player_event);
    }
}
