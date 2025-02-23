use bevy::prelude::*;

pub struct BlenvyCheckerPlugin;
impl Plugin for BlenvyCheckerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameplaySceneLoaded::default())
            .add_systems(
                Update,
                gameplay_scene_loaded.run_if(is_gameplay_scene_not_loaded),
            );
    }
}

///
/// Used as magic name when spawning anything from
/// "${project_root}/levels/**.glb"
/// Give all those blueprints this name to look after events of when it is
/// finally loaded
pub const LEVEL_STRING: &str = "CurrentGameplayLevel";

#[derive(Event)]
pub struct GameplaySceneLoadedEvent;

#[derive(Resource, Default)]
struct GameplaySceneLoaded {
    loaded: bool,
}

fn is_gameplay_scene_not_loaded(res: Res<GameplaySceneLoaded>) -> bool {
    !res.loaded
}

fn gameplay_scene_loaded(
    mut bp_events: EventReader<blenvy::BlueprintEvent>,
    mut commands: Commands,
    mut res: ResMut<GameplaySceneLoaded>,
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
            info!("{} fully loaded", LEVEL_STRING);
            commands.trigger(GameplaySceneLoadedEvent);
            res.loaded = true;
        }
    }
}
