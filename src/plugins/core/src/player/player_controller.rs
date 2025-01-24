use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use crate::{
    input_manager::{self, motion, InputManager},
    isometric_camera::CameraManager,
};

use super::graphical_component::GraphicalModulePlugin;

pub struct PlayerControllerPlugin;
impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(GraphicalModulePlugin)
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
    mut camera: ResMut<CameraManager>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let motion = im.get_motion(MOVEMENT);
    let cam_direction = camera.get_camera_rotation();
    // let pos = player.translation;
}

