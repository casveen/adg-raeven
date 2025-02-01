use bevy::prelude::*;

use crate::{
    camera::isometric_camera::CameraYaw,
    input::input_manager::{self, button, motion, InputManager},
};

use super::player_visuals::RenderPlugin;
use super::states;

pub struct PlayerControllerPlugin;
impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderPlugin)
            .add_systems(
                Startup,
                (
                    register_input,
                    spawn_player,
                    //
                ),
            )
            .add_systems(Update, process_input);
    }
}

#[derive(Component)]
#[require(Transform(|| Transform::from_xyz(0., 0., 0.)))]
pub struct Player;

// Method to change states once player has been spawned
#[macro_export]
macro_rules! new_state {
    ($commands:expr, $fsm:expr, $children:expr, $next_state:expr) => {{
        for c in *$children {
            $commands.entity(*c).remove_parent().despawn();
        }
        let new_state = $commands.add_observer($next_state).id();
        $commands.entity(*$fsm).insert_children(0, &[new_state]);
    }};
}
pub(super) use new_state;
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

#[derive(Resource, Default)]
pub struct PlayerSpawn {
    pub transform: Transform,
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

fn spawn_player(mut commands: Commands, player_spawn: Res<PlayerSpawn>) {
    // Spawning of Player Fsm, use new_state! after this
    let state = commands.add_observer(states::idle_run::process_event).id();
    let fsm_entity = commands.spawn(PlayerFsm).insert_children(0, &[state]).id();

    commands
        .spawn((Player, player_spawn.transform))
        .insert_children(0, &[fsm_entity]);
}

fn process_input(
    im: Res<InputManager>,
    yaw: Res<CameraYaw>,
    mut commands: Commands,
    mut moved_last_frame: Local<bool>,
) {
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
