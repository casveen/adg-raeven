use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use blenvy::{BlueprintInfo, GameWorldTag, HideUntilReady, SpawnBlueprint};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            core::CorePlugin,
        ))
        .add_systems(Startup, setup_game)
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo {
            path: "levels/AntDemo.glb".to_string(),
            name: core::utils::blenvy_checker::LEVEL_STRING.to_string(),
        },
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}
