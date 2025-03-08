use bevy::prelude::*;

use crate::{
    camera::isometric_camera::CameraYaw,
    input::input_manager::{self, button, motion, InputManager},
    utils::gameplayscene_loadstatus::GameplaySceneLoadedEvent,
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
                    evaluate_player_spawn_timer,
                ),
            )
            .add_observer(start_player_spawn_timer)
            .add_observer(spawn_player);
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
    pub motion: Option<Vec3>,
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

#[derive(Component)]
struct PlayerSpawnTimer {
    timer: Timer,
}

#[derive(Event, Default)]
struct PlayerSpawnEvent {
    transform: Transform,
}

fn start_player_spawn_timer(
    _: Trigger<GameplaySceneLoadedEvent>,
    player_spawn: Query<(Entity, &PlayerSpawn), Without<PlayerSpawnTimer>>, // Sketchy with Single here? Make sure blenvy only makes a single copy of this component
    mut commands: Commands,
) {
    if player_spawn.is_empty() {
        return;
    }
    commands
        .entity(player_spawn.single().0)
        .insert(PlayerSpawnTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        });
}

fn evaluate_player_spawn_timer(
    mut q_player_spawn: Query<(Entity, &mut PlayerSpawnTimer, &GlobalTransform)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if q_player_spawn.is_empty() {
        return;
    }
    let mut q_player_spawn = q_player_spawn.single_mut();

    q_player_spawn.1.timer.tick(time.delta());
    if q_player_spawn.1.timer.finished() {
        commands
            .entity(q_player_spawn.0)
            .remove::<PlayerSpawnTimer>();
        commands.trigger(PlayerSpawnEvent {
            transform: q_player_spawn.2.compute_transform(),
        });
    }
}

static MOVEMENT: input_manager::Action = input_manager::Action("movement");
static ABILITY_FLOATY: input_manager::Action = input_manager::Action("ability_floaty");
static ABILITY_CORDYCEPT: input_manager::Action = input_manager::Action("ability_cordycept");

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
}

fn spawn_player(
    player_spawn: Trigger<PlayerSpawnEvent>,
    query: Query<&Player>,
    mut commands: Commands,
) {
    // lazy assertion
    if !query.is_empty() {
        unreachable!("Trying to spawn a new player entity. STOP!");
    }

    // Spawning of Player Fsm, use new_state! after this
    let state = commands.add_observer(states::idle_run::process_event).id();
    let fsm_entity = commands.spawn(PlayerFsm).insert_children(0, &[state]).id();

    commands
        .spawn((
            Player {
                state: PlayerState::Alive,
            },
            player_spawn.transform,
        ))
        .insert_children(0, &[fsm_entity]);
}

fn process_input(
    q_player: Query<&Player>,
    im: Res<InputManager>,
    yaw: Res<CameraYaw>,
    mut commands: Commands,
    mut moved_last_frame: Local<bool>,
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
    }

    let Some(direction) = im.get_motion(MOVEMENT).get_motion_opt_y(yaw.get()) else {
        if *moved_last_frame {
            commands.trigger(PlayerEvent::Movement(PlayerMovementEvent { motion: None }));
        }
        *moved_last_frame = false;
        return;
    };
    *moved_last_frame = true;

    commands.trigger(PlayerEvent::Movement(PlayerMovementEvent {
        motion: Some(direction),
    }));
}
