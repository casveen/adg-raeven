use bevy::prelude::*;

pub struct BlenvyCheckerPlugin;
impl Plugin for BlenvyCheckerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gameplay_scene_loaded);
    }
}

///
/// Used as magic name when spawning anything from
/// "${project_root}/levels/**.glb"
/// Give all those blueprints this name to look after events of when it is
/// finally loaded
pub const LEVEL_STRING: &str = "current_level";

#[derive(Event)]
pub struct GameplaySceneLoadedEvent;

pub fn gameplay_scene_loaded(
    mut bp_events: EventReader<blenvy::BlueprintEvent>,
    mut commands: Commands,
) {
    for bp_event in bp_events.read() {
        let blenvy::BlueprintEvent::InstanceReady {
            entity: _,
            blueprint_name,
            blueprint_path: _,
        } = bp_event
        else {
            warn!("!!! Blueprint event was not InstanceReady");
            return;
        };

        if *blueprint_name == LEVEL_STRING.to_string() {
            info!("!!! Gameplay level fully loaded");
            commands.trigger(GameplaySceneLoadedEvent)
        }
    }
}
