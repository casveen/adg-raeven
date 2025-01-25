use std::time::Duration;

use bevy::prelude::*;

use super::player_controller::{Player, PlayerMovementEvent};

pub(super) struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_player_mesh)
            .add_observer(setup_once_loaded)
            .add_observer(animation_motion);
    }
}

#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}
#[derive(Component)]
pub struct Boxy;
const BOXY_PATH: &str = "models/boxy.glb";

fn spawn_player_mesh(
    _: Trigger<OnAdd, Player>,
    player: Single<Entity, With<Player>>,
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
    commands.entity(*player).insert((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(BOXY_PATH))),
        Boxy,
    ));
}

fn setup_once_loaded(
    _: Trigger<OnInsert, AnimationPlayer>,
    player: Single<Entity, With<Player>>,
    mut commands: Commands,
    animations: Res<Animations>,
    mut anim_players: Query<(Entity, &mut AnimationPlayer), With<AnimationPlayer>>,
) {
    println!("AnimationPlayer loaded...");
    let (entity, mut anim_player) = anim_players.single_mut();
    println!("player {:?}", *player);
    println!("entt {:?}", entity);
    let mut transitions = AnimationTransitions::new();
    transitions
        .play(&mut anim_player, animations.animations[0], Duration::ZERO)
        .repeat();
    commands
        .entity(entity)
        .insert(AnimationGraphHandle(animations.graph.clone()))
        .insert(transitions);
    commands.entity(*player).insert_children(0, &[entity]);
}

const ANIM_IDLE: usize = 0;
const ANIM_RUN: usize = 1;

const ANIMSPEED_RUN: f32 = 244.0;

fn animation_motion(
    movement_event: Trigger<PlayerMovementEvent>,
    _: Query<&Parent, With<Player>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
) {
    let (mut player, mut transitions) = animation_players.single_mut();

    match movement_event.event().position_delta {
        Some(delta) => {
            if !player.is_playing_animation(animations.animations[ANIM_RUN]) {
                transitions
                    .play(&mut player, animations.animations[ANIM_RUN], Duration::ZERO)
                    .repeat();
            }
            for (_, anim) in player.playing_animations_mut() {
                anim.set_speed(delta.length_squared() * ANIMSPEED_RUN);
            }
        }
        None => {
            transitions
                .play(
                    &mut player,
                    animations.animations[ANIM_IDLE],
                    Duration::ZERO,
                )
                .repeat();
        }
    }
}
