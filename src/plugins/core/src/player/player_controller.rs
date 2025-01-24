use bevy::prelude::*;

use crate::{
    camera::isometric_camera::CameraYaw,
    input::input_manager::{self, motion, InputManager},
};

use super::render_plugin::RenderPlugin;

const PLAYER_SPEED: f32 = 10.0;

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
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
#[require(Transform(|| Transform::from_xyz(0., 0., 0.)))]
pub struct Player;

#[derive(Event)]
pub struct PlayerMovementEvent {
    pub position_delta: Option<Vec3>,
}

#[derive(Resource, Default)]
pub struct PlayerSpawn {
    pub transform: Transform,
}

static MOVEMENT: input_manager::Action = input_manager::Action("movement");

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
}

fn spawn_player(mut commands: Commands, player_spawn: Res<PlayerSpawn>) {
    commands.spawn((Player, player_spawn.transform));
}

fn player_movement(
    im: Res<InputManager>,
    yaw: Res<CameraYaw>,
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
    mut moved_last_frame: Local<bool>,
) {
    let Some(direction) = im.get_motion(MOVEMENT).get_motion_opt_y(yaw.get()) else {
        if *moved_last_frame {
            commands.trigger(PlayerMovementEvent {
                position_delta: None,
            });
        }
        *moved_last_frame = false;
        return;
    };
    let movement = direction * PLAYER_SPEED * time.delta_secs();
    player.translation += movement;
    commands.trigger(PlayerMovementEvent {
        position_delta: Some(movement),
    });
    *moved_last_frame = true;

    let direction = direction.normalize(); // gamepad motion can be < 1.
    let forward = -Vec3::Y.cross(direction);
    let right = direction;
    let matrix = Mat3::from_cols(right, Vec3::Y, forward);
    let quat = Quat::from_mat3(&matrix);
    player.rotation = quat;
}
