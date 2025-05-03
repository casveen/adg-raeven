pub mod creature;
pub mod environment;
pub mod fertile_ground;
pub mod grid;
pub mod mushroom;
pub mod puffy_shrooms;
pub mod spore_cloud;
pub mod water;

use bevy::prelude::*;

pub struct GameWorldPlugin;
impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            environment::EnvironmentPlugin,
            fertile_ground::FertileGroundPlugin,
            grid::WorldGridPlugin,
            mushroom::MushroomPlugin,
            puffy_shrooms::PuffyShroomsPlugin,
            spore_cloud::SporeCloudPlugin,
            water::WaterPlugin,
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
