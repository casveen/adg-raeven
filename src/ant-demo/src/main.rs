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
        BlueprintInfo::from_path("levels/AntDemo.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}
