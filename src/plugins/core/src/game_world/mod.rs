pub mod creature;
pub mod environment;
pub mod exit_gate;
pub mod fertile_ground;
pub mod grid;
pub mod mushroom;
pub mod puffy_shrooms;
pub mod spore_cloud;
pub mod water;
pub mod win_screen;

use bevy::prelude::*;

pub struct GameWorldPlugin;
impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            environment::EnvironmentPlugin,
            exit_gate::ExitGatePlugin,
            fertile_ground::FertileGroundPlugin,
            grid::WorldGridPlugin,
            mushroom::MushroomPlugin,
            puffy_shrooms::PuffyShroomsPlugin,
            spore_cloud::SporeCloudPlugin,
            water::WaterPlugin,
            win_screen::WinScreenPlugin,
        ))
        .register_type::<Wall>();
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Tree;
