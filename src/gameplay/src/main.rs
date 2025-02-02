use bevy::prelude::*;
use blenvy::{BlenvyPlugin, BlueprintInfo, GameWorldTag, HideUntilReady, SpawnBlueprint};

mod mushroom;
use mushroom::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct Player {
    strength: f32,
    perception: f32,
    endurance: f32,
    charisma: f32,
    intelligence: f32,
    agility: f32,
    luck: f32,
}

/*#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct Bloomable {
    bloomed: bool,
    bloom_tween: f32, //[0,1]
    to_bloom: String,

}*/

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin::default()),
            // our custom plugins
            MushroomPlugin, // Showcases different type of components /structs
            BlenvyPlugin::default(),
        ))
        //.register_type::<Player>()
        .add_systems(Startup, setup_game)
        .run();
}

fn setup_game(mut commands: Commands) {
    // here we actually spawn our game world/level
    commands.spawn((
        BlueprintInfo::from_path("levels/World.glb"), // all we need is a Blueprint info...
        SpawnBlueprint, // and spawnblueprint to tell blenvy to spawn the blueprint now
        HideUntilReady, // only reveal the level once it is ready
        GameWorldTag,
    ));
}



/*
    BlueprintInfo::from_path("Health_Pickup.glb"), // mandatory !!
    // or the alterive: BlueprintInfo{name:"health pickup1".into(), path:"Health_Pickup.glb".into()}
    SpawnBlueprint, // mandatory !!
    
    TransformBundle::from_transform(Transform::from_xyz(x, 2.0, y)), // optional
*/