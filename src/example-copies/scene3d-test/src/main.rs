use core::player::player_controller::PlayerSpawn;

use bevy::{picking::pointer::PointerInteraction, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use core::input::input_manager::{
    button, motion, Action, InputManager, InputModeChanged, InputType,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            core::CorePlugin,
            MeshPickingPlugin,
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(GroundEntity::default())
        .insert_resource(PlayerSpawn {
            transform: Transform::from_xyz(0., 0., 0.),
        })
        .add_systems(Startup, (setup, register_input))
        .add_systems(Update, draw_cursor)
        .add_observer(get_input_mode_change_trigger)
        .run();
}

static ACTIVATE: Action = Action("activate");
static SPAWN_SHROOM: Action = Action("spawn_shroom");
static MOVEMENT: Action = Action("movement");
static CAMERA: Action = Action("camera");

fn register_input(mut im: ResMut<InputManager>) {
    im.register_action_button(
        ACTIVATE,
        vec![
            button::Variant::Keyboard(KeyCode::KeyE),
            button::Variant::Gamepad(GamepadButton::South),
        ],
    );

    im.register_action_button(
        SPAWN_SHROOM,
        vec![
            button::Variant::Keyboard(KeyCode::Space),
            button::Variant::Gamepad(GamepadButton::North),
        ],
    );

    im.register_action_motion(
        MOVEMENT,
        vec![
            motion::Entry {
                input_type: InputType::Keyboard,
                relations: vec![
                    motion::Relation::KeyCode(KeyCode::KeyW, motion::Axis::PosY),
                    motion::Relation::KeyCode(KeyCode::KeyS, motion::Axis::NegY),
                    motion::Relation::KeyCode(KeyCode::KeyD, motion::Axis::PosX),
                    motion::Relation::KeyCode(KeyCode::KeyA, motion::Axis::NegX),
                ],
            },
            motion::Entry {
                input_type: InputType::Gamepad,
                relations: vec![
                    motion::Relation::GamepadAxis(GamepadAxis::LeftStickY, motion::Axis::Y),
                    motion::Relation::GamepadAxis(GamepadAxis::LeftStickX, motion::Axis::X),
                ],
            },
        ],
    );

    im.register_action_motion(
        CAMERA,
        vec![
            motion::Entry {
                input_type: InputType::Keyboard,
                relations: vec![
                    motion::Relation::KeyCode(KeyCode::KeyK, motion::Axis::PosY),
                    motion::Relation::KeyCode(KeyCode::KeyJ, motion::Axis::NegY),
                    motion::Relation::KeyCode(KeyCode::KeyL, motion::Axis::PosX),
                    motion::Relation::KeyCode(KeyCode::KeyH, motion::Axis::NegX),
                ],
            },
            motion::Entry {
                input_type: InputType::Mouse,
                relations: vec![motion::Relation::Mouse(20.)],
            },
            motion::Entry {
                input_type: InputType::Gamepad,
                relations: vec![
                    motion::Relation::GamepadAxis(GamepadAxis::RightStickY, motion::Axis::Y),
                    motion::Relation::GamepadAxis(GamepadAxis::RightStickX, motion::Axis::X),
                ],
            },
        ],
    );
}

fn get_input_mode_change_trigger(trigger: Trigger<InputModeChanged>) {
    let event = trigger.event();
    println!("TRIGGER input_mode_change: {:?}", event);
}

#[derive(Component)]
struct Ground;

#[derive(Resource)]
struct GroundEntity {
    id: u32,
}
impl GroundEntity {
    fn default() -> GroundEntity {
        GroundEntity { id: 0 }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ground_entity: ResMut<GroundEntity>,
) {
    // ground
    ground_entity.id = commands
        .spawn((
            Mesh3d(meshes.add(Circle::new(4.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            Ground,
        ))
        .id()
        .index();

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn draw_cursor(
    pointers: Query<&PointerInteraction>,
    mut gizmos: Gizmos,
    ground_entity: Res<GroundEntity>,
) {
    // draw circle just above ground plane
    for (entity, point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(entity, hit)| {
            hit.position
                .zip(hit.normal)
                .map(|(position, normal)| (entity, position, normal))
        })
    {
        if entity.index() == ground_entity.id {
            gizmos.circle(
                Isometry3d::new(
                    point + normal * 0.01,
                    Quat::from_rotation_arc(Vec3::Z, normal),
                ),
                0.2,
                Color::WHITE,
            );
        }
    }
}
