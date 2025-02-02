use avian3d::PhysicsPlugins;

pub mod camera;
pub mod enemies;
pub mod exit_game;
pub mod input;
pub mod player;
mod settings;
pub mod static_game_world;
mod utils;

pub struct CorePlugin;
impl bevy::prelude::Plugin for CorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            exit_game::ExitGamePlugin,
            input::input_manager::InputManagerPlugin,
            camera::isometric_camera::IsometricCameraPlugin,
            player::player_controller::PlayerControllerPlugin,
            enemies::EnemiesPlugin,
            settings::plugins::VendorPlugin,
            static_game_world::StaticGameWorldPlugin,
            PhysicsPlugins::default(), // avian3d
        ));
    }
}
