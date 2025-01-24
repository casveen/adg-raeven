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
) {
    let movement =
        im.get_motion(MOVEMENT).get_motion_y(yaw.get()) * PLAYER_SPEED * time.delta_secs();
    player.translation += movement;
}
