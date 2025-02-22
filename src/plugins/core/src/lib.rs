use avian3d::PhysicsPlugins;
use blenvy::BlenvyPlugin;

pub mod camera;
pub mod enemies;
pub mod exit_game;
pub mod game_world;
pub mod input;
pub mod player;
mod settings;
pub mod utils;

pub struct CorePlugin;
impl bevy::prelude::Plugin for CorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            BlenvyPlugin::default(),
            exit_game::ExitGamePlugin,
            game_world::GameWorldPlugin,
            input::input_manager::InputManagerPlugin,
            camera::isometric_camera::IsometricCameraPlugin,
            player::PlayerPlugin,
            enemies::EnemiesPlugin,
            settings::plugins::VendorPlugin,
            PhysicsPlugins::default(), // avian3d
            utils::blenvy_checker::BlenvyCheckerPlugin,
        ));
    }
}
