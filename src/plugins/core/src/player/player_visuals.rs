use std::time::Duration;

use bevy::prelude::*;

use super::player_controller::{Player, PlayerEvent};

pub(super) struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_player_mesh)
            .add_observer(setup_once_loaded)
            .add_observer(observe_player_event);
    }
}

#[derive(Resource)]
pub struct Animations {
    pub animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}
#[derive(Component)]
pub struct AnimationComponent {
    pub fsm: fsm::Fsm,
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
    // anim index matching .gltf
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(3).from_asset(BOXY_PATH)),
        asset_server.load(GltfAssetLabel::Animation(2).from_asset(BOXY_PATH)),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(BOXY_PATH)),
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(BOXY_PATH)),
    ]);
    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        animations: node_indices,
        graph: graph_handle,
    });
    commands.entity(*player).insert((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(BOXY_PATH))),
        Boxy,
        AnimationComponent {
            fsm: fsm::Fsm::new(),
        },
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
        .play(
            &mut anim_player,
            animations.animations[fsm::ANIM_IDLE],
            Duration::ZERO,
        )
        .repeat();
    commands
        .entity(entity)
        .insert(AnimationGraphHandle(animations.graph.clone()))
        .insert(transitions);
    commands.entity(*player).insert_children(0, &[entity]);
}

fn observe_player_event(
    event: Trigger<PlayerEvent>,
    _: Query<&Parent, With<Player>>,
    mut fsm: Single<&mut AnimationComponent, With<Player>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
) {
    let (mut player, mut transitions) = animation_players.single_mut();
    fsm.fsm.process_event(
        &event.event(),
        &mut fsm::AnimUpdateAggregate(&animations, &mut player, &mut transitions),
    );
}

mod fsm {
    use std::time::Duration;

    use bevy::prelude::*;

    use crate::player::player_controller::{PlayerEvent, PlayerMovementEvent};
    use crate::player::player_visuals::Animations;

    // These match index of array in Animation graph, not .gltf file
    pub const ANIM_IDLE: usize = 1;
    pub const ANIM_RUN: usize = 0;
    pub const ANIM_FLOATY_HAT: usize = 2;
    pub const ANIM_BLAST: usize = 3;

    const ANIMSPEED_RUN: f32 = 8.0;

    pub struct AnimUpdateAggregate<'a>(
        pub &'a Animations,
        pub &'a mut AnimationPlayer,
        pub &'a mut AnimationTransitions,
    );
    pub struct StateMovement;
    pub struct StateFloaty;
    pub struct Fsm {
        current_state: Box<dyn TState>,
    }
    impl Fsm {
        pub fn new() -> Self {
            Self {
                current_state: Box::new(StateMovement),
            }
        }

        pub fn process_event(&mut self, event: &PlayerEvent, anim_update: &mut AnimUpdateAggregate) {
            let new_state = self.current_state.process_event(event, anim_update);

            let Some(new_state) = new_state else {
                return;
            };

            self.current_state = new_state;
            self.current_state.enter_state(event, anim_update);
        }
    }

    pub trait TState: Send + Sync {
        fn enter_state(&self, event: &PlayerEvent, anim_update: &mut AnimUpdateAggregate);
        fn process_event(
            &self,
            event: &PlayerEvent,
            anim_update: &mut AnimUpdateAggregate,
        ) -> Option<Box<dyn TState>>;
    }

    impl TState for StateMovement {
        fn enter_state(&self, event: &PlayerEvent, anim_update: &mut AnimUpdateAggregate) {
            if let PlayerEvent::Movement(event) = event {
                self.handle_movement(event, anim_update);
            } else {
                self.handle_movement(&PlayerMovementEvent::empty(), anim_update);
            }
        }

        fn process_event(
            &self,
            event: &PlayerEvent,
            anim_update: &mut AnimUpdateAggregate,
        ) -> Option<Box<dyn TState>> {
            if let PlayerEvent::Movement(event) = event {
                self.handle_movement(event, anim_update);
                return None;
            }

            match event {
                PlayerEvent::Floaty(_) => Some(Box::new(StateFloaty)),
                _ => None,
            }
        }
    }
    impl TState for StateFloaty {
        fn enter_state(&self, event: &PlayerEvent, anim_update: &mut AnimUpdateAggregate) {
            let PlayerEvent::Floaty(event) = event else {
                return;
            };

            if event.active {
                let AnimUpdateAggregate(animations, anim_player, anim_transitions) =
                    &mut *anim_update;
                anim_transitions
                    .play(
                        anim_player,
                        animations.animations[ANIM_FLOATY_HAT],
                        Duration::ZERO,
                    )
                    .repeat();
            }
        }

        fn process_event(
            &self,
            event: &PlayerEvent,
            _anim_update: &mut AnimUpdateAggregate,
        ) -> Option<Box<dyn TState>> {
            match event {
                PlayerEvent::Floaty(event) => {
                    if !event.active {
                        return Some(Box::new(StateMovement));
                    }
                    None
                }
                _ => None,
            }
        }
    }

    impl StateMovement {
        fn handle_movement(
            &self,
            movement_event: &PlayerMovementEvent,
            anim_update: &mut AnimUpdateAggregate,
        ) {
            let AnimUpdateAggregate(animations, anim_player, anim_transitions) = &mut *anim_update;

            match movement_event.motion {
                Some(delta) => {
                    if !anim_player.is_playing_animation(animations.animations[ANIM_RUN]) {
                        anim_transitions
                            .play(anim_player, animations.animations[ANIM_RUN], Duration::ZERO)
                            .repeat();
                    }
                    for (_, anim) in anim_player.playing_animations_mut() {
                        anim.set_speed(delta.length() * ANIMSPEED_RUN);
                    }
                }
                None => {
                    anim_transitions
                        .play(
                            anim_player,
                            animations.animations[ANIM_IDLE],
                            Duration::ZERO,
                        )
                        .repeat();
                }
            }
        }
    }
}
