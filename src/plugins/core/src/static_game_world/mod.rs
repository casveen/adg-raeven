use bevy::prelude::*;

pub mod wall;

pub struct StaticGameWorldPlugin;
impl Plugin for StaticGameWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, wall::setup_walls);
    }
}
