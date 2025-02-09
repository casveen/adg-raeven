pub mod puffy_shrooms;
pub mod spore_cloud;

use bevy::prelude::*;

pub struct GameWorldPlugin;
impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spore_cloud::SporeCloudPlugin,
            puffy_shrooms::PuffyShroomsPlugin,
        ));
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Tree;
