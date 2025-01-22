use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    input_manager::{self, motion, InputManager},
    isometric_camera::CameraManager,
};

#[derive(Component)]
#[require(Transform(|| Transform::from_xyz(0., 0., 0.)))]
pub struct Player;

#[derive(Resource, Default)]
pub struct PlayerSpawn {
    pub transform: Transform,
}

// tmp visual elements stuff, place in separate module
#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}
#[derive(Component)]
struct Boxy;
const BOXY_PATH: &str = "models/boxy.glb";

static MOVEMENT: input_manager::Action = input_manager::Action("movement");

pub struct PlayerControllerPlugin;
impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                register_input,
                spawn_player,
                spawn_player_mesh,
                // spawn_mesh as separate system
            )
                .chain(),
        )
        .add_systems(Update, player_movement);
    }
}

fn spawn_player(mut commands: Commands, player_spawn: Res<PlayerSpawn>) {
    commands.spawn((Player, player_spawn.transform));
}

fn spawn_player_mesh(
    player: Single<&Transform, With<Player>>,
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
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

fn player_movement(
    im: Res<InputManager>,
    mut camera: ResMut<CameraManager>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let motion = im.get_motion(MOVEMENT);
    let cam_direction = camera.get_camera_rotation();
    // let pos = player.translation;
}
