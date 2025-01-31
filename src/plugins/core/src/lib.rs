pub mod camera;
pub mod exit_game;
pub mod fsm;
pub mod input;
pub mod player;
mod settings;

pub struct CorePlugin;
impl bevy::prelude::Plugin for CorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            exit_game::ExitGamePlugin,
            input::input_manager::InputManagerPlugin,
            camera::isometric_camera::IsometricCameraPlugin,
            player::player_controller::PlayerControllerPlugin,
            settings::plugins::VendorPlugin,
        ));
    }
}
