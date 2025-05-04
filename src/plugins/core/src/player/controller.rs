use avian3d::prelude::{Collider, CollidingEntities, RigidBody, RigidBodyQueryItem};
use bevy::prelude::*;

use crate::{
    camera::isometric_camera::CameraYaw,
    creatures::movement::creature_movement::MovingCreature,
    game_world::{exit_gate::DestroyExitGate, fertile_ground::SpawnFertileGround},
    input::input_manager::{self, button, motion, InputManager},
    utils::{gameplayscene_loadstatus::GameplaySceneLoadedEvent, grid::Direction},
};

use super::states;

pub struct PlayerControllerPlugin;
impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerSpawn>()
            .add_systems(
                Startup,
                (
                    //
                    register_input,
                ),
            )
            .add_systems(
                Update,
                (
                    //
                    process_input,
                    evaluate_player_respawn_timer,
                ),
            )
            .add_observer(start_player_spawn_timer_on_loading_scene)
            .add_observer(spawn_player_on_respawn_event)
            .add_observer(spawn_player)
            .add_observer(kill_player);
    }
}

#[derive(Reflect)]
enum PlayerState {
    NotSpawned,
    Dead,
    Alive,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Transform(|| Transform::from_xyz(0., 0., 0.)))]
pub struct Player {
    state: PlayerState,
}

#[derive(Component)]
pub struct PlayerFsm;

#[derive(Event)]
pub struct PlayerMovementEvent {
    pub motion: Option<Direction>,
}
impl PlayerMovementEvent {
    pub fn empty() -> Self {
        Self { motion: None }
    }
}

#[derive(Event)]
pub struct PlayerFloatyEvent {
    pub active: bool,
}

#[derive(Event)]
pub struct PlayerCordyCeptEvent {
    pub active: bool,
}

#[derive(Event)]
pub enum PlayerEvent {
    Movement(PlayerMovementEvent),
    Floaty(PlayerFloatyEvent),
    CordyCept(PlayerCordyCeptEvent),
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct PlayerSpawn;

#[derive(Event, Default)]
struct PlayerSpawnEvent {
    transform: Transform,
}

/**
 * To kill the player, simply trigger the KillPlayer event. Globally or on the Entity.
 * Player death deletes its entity and spawns another entity holding a timer
 * which launches PlayerRespawnEvent
 */
#[derive(Event)]
pub struct KillPlayerEvent {
    pub description: String,
}

#[derive(Component)]
struct PlayerRespawnTimer {
    respawn_timer: Timer,
}
impl Default for PlayerRespawnTimer {
    fn default() -> Self {
        Self {
            respawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

#[derive(Event)]
struct PlayerRespawnEvent;

fn kill_player(
    trigger: Trigger<KillPlayerEvent>,
    player: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    if player.is_empty() {
        // unreachable!("Misfire of KillPlayer event. Attempted to kill a non-existant player.")
        return;
    }
    let player = player.single();

    info!("KillPlayerEvent[ {} ]", trigger.event().description);

    commands.entity(player).despawn_recursive();
    commands.spawn(PlayerRespawnTimer::default());
}

fn evaluate_player_respawn_timer(
    mut player_respawner: Query<(Entity, &mut PlayerRespawnTimer)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if player_respawner.is_empty() {
        return;
    }
    let mut player_respawner = player_respawner.single_mut();
    player_respawner.1.respawn_timer.tick(time.delta());
    if player_respawner.1.respawn_timer.finished() {
        commands.entity(player_respawner.0).despawn();
        commands.trigger(PlayerRespawnEvent);
    }
}

// Actually spawning player
fn start_player_spawn_timer_on_loading_scene(
    _: Trigger<GameplaySceneLoadedEvent>,
    mut commands: Commands,
) {
    commands.spawn(PlayerRespawnTimer::default());
}

fn spawn_player_on_respawn_event(
    _: Trigger<PlayerRespawnEvent>,
    player_spawn: Query<&GlobalTransform, With<PlayerSpawn>>,
    mut commands: Commands,
) {
    if player_spawn.is_empty() {
        return;
    }
    let player_spawn = player_spawn.single();

    commands.trigger(PlayerSpawnEvent {
        transform: player_spawn.compute_transform(),
    });
}

static MOVEMENT: input_manager::Action = input_manager::Action("movement");
static ABILITY_FLOATY: input_manager::Action = input_manager::Action("ability_floaty");
static ABILITY_CORDYCEPT: input_manager::Action = input_manager::Action("ability_cordycept");
static ABILITY_SPAWN_FERTILE_GROUND: input_manager::Action =
    input_manager::Action("spawn_fertile_ground");

fn register_input(mut im: ResMut<input_manager::InputManager>) {
    im.register_action_motion(
        MOVEMENT,
        vec![
            motion::Entry {
                input_type: input_manager::InputType::Keyboard,
                relations: vec![
                    motion::Relation::KeyCode(KeyCode::KeyW, motion::Axis::PosY),
                    motion::Relation::KeyCode(KeyCode::KeyS, motion::Axis::NegY),
                    motion::Relation::KeyCode(KeyCode::KeyD, motion::Axis::PosX),
                    motion::Relation::KeyCode(KeyCode::KeyA, motion::Axis::NegX),
                ],
            },
            motion::Entry {
                input_type: input_manager::InputType::Gamepad,
                relations: vec![
                    motion::Relation::GamepadAxis(GamepadAxis::LeftStickY, motion::Axis::Y),
                    motion::Relation::GamepadAxis(GamepadAxis::LeftStickX, motion::Axis::X),
                ],
            },
        ],
    );

    im.register_action_button(
        ABILITY_FLOATY,
        vec![
            button::Variant::Keyboard(KeyCode::KeyK),
            button::Variant::Gamepad(GamepadButton::North),
        ],
    );
    im.register_action_button(
        ABILITY_CORDYCEPT,
        vec![
            button::Variant::Keyboard(KeyCode::KeyJ),
            button::Variant::Gamepad(GamepadButton::East),
        ],
    );
    im.register_action_button(
        ABILITY_SPAWN_FERTILE_GROUND,
        vec![
            button::Variant::Keyboard(KeyCode::Space),
            button::Variant::Gamepad(GamepadButton::South),
        ],
    );
}

fn spawn_player(
    player_spawn: Trigger<PlayerSpawnEvent>,
    player: Query<(Entity, &Player)>,
    mut commands: Commands,
) {
    // lazy assertion
    if !player.is_empty() {
        unreachable!("Trying to spawn a new player entity. STOP!");
    }

    // Spawning of Player Fsm, use new_state! after this
    let state = commands.add_observer(states::idle_run::process_event).id();
    // let fsm_entity = commands.spawn(PlayerFsm).insert_children(0, &[state]).id();

    commands
        .spawn((
            Player {
                state: PlayerState::Alive,
            },
            player_spawn.transform,
            MovingCreature::Player,
            Collider::sphere(0.5),
            CollidingEntities::default(),
        ))
        // .insert_children(0, &[fsm_entity]);
        .with_children(|player| {
            player.spawn(PlayerFsm).insert_children(0, &[state]);
        });
}

fn process_input(
    q_player: Query<&Player>,
    im: Res<InputManager>,
    yaw: Res<CameraYaw>,
    mut commands: Commands,
    mut moved_last_frame: Local<bool>,
    q_global_transform: Query<&GlobalTransform, With<Player>>,
) {
    if q_player.is_empty() {
        // Log something here?
        // For when Player entity does not exist...
        return;
    }

    if im.is_action_just_pressed(ABILITY_FLOATY) {
        commands.trigger(PlayerEvent::Floaty(PlayerFloatyEvent { active: true }));
    } else if im.is_action_just_released(ABILITY_FLOATY) {
        commands.trigger(PlayerEvent::Floaty(PlayerFloatyEvent { active: false }));
    }

    if im.is_action_just_pressed(ABILITY_CORDYCEPT) {
        commands.trigger(PlayerEvent::CordyCept(PlayerCordyCeptEvent {
            active: true,
        }));
    } else if im.is_action_just_released(ABILITY_CORDYCEPT) {
        commands.trigger(PlayerEvent::CordyCept(PlayerCordyCeptEvent {
            active: false,
        }));
    } else if im.is_action_just_pressed(ABILITY_SPAWN_FERTILE_GROUND) {
        commands.trigger(SpawnFertileGround {
            request_instigator_transform: q_global_transform.single().clone(),
        })
    }

    let Some(direction_vector) = im.get_motion(MOVEMENT).get_motion_opt_y(yaw.get()) else {
        if *moved_last_frame {
            commands.trigger(PlayerEvent::Movement(PlayerMovementEvent { motion: None }));
        }
        *moved_last_frame = false;
        return;
    };
    *moved_last_frame = true;
    info!("direction vector {:?}", direction_vector);
    commands.trigger(PlayerEvent::Movement(PlayerMovementEvent {
        motion: Some(Direction::from(direction_vector)),
    }));
}
