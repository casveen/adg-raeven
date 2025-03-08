use core::player::controller::KillPlayerEvent;

use bevy::{input::keyboard::KeyboardInput, prelude::*};
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
        .add_systems(Update, process_input)
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo {
            path: "levels/AntDemo.glb".to_string(),
            name: core::utils::gameplayscene_loadstatus::LEVEL_NAME.to_string(),
        },
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}

// for testing
fn process_input(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    for key in keyboard.get_just_pressed() {
        if key == &KeyCode::KeyZ {
            commands.trigger(KillPlayerEvent {
                description: "KeyCode::KeyZ".to_string(),
            });
        }
    }
}
