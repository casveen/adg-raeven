use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use super::player_controller::Player;

pub(super) struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_player_mesh)
            .add_observer(setup_once_loaded)
            .add_systems(Update, keyboard_animation_control);
    }
}

#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}
#[derive(Component)]
struct Boxy;
const BOXY_PATH: &str = "models/boxy.glb";

fn spawn_player_mesh(
    _: Trigger<OnAdd, Player>,
    player: Single<&Transform, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // boxy
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(BOXY_PATH)),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(BOXY_PATH)),
    ]);
    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        animations: node_indices,
        graph: graph_handle,
    });
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(BOXY_PATH))),
        **player,
        Boxy,
    ));
}

fn setup_once_loaded(
    _: Trigger<OnInsert, AnimationPlayer>,
    mut commands: Commands,
    animations: Res<Animations>,
    mut anim_players: Query<(Entity, &mut AnimationPlayer), With<AnimationPlayer>>,
) {
    println!("AnimationPlayer loaded...");
    for (entity, mut anim_player) in &mut anim_players {
        let mut transitions = AnimationTransitions::new();
        transitions
            .play(&mut anim_player, animations.animations[0], Duration::ZERO)
            .repeat();
        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}

// Reference for changing animation
//
fn keyboard_animation_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {
    for (mut player, mut transitions) in &mut animation_players {
        if keyboard_input.just_pressed(KeyCode::Enter) {
            *current_animation = (*current_animation + 1) % animations.animations.len();

            transitions
                .play(
                    &mut player,
                    animations.animations[*current_animation],
                    Duration::ZERO,
                )
                .repeat();
        }
    }
}
