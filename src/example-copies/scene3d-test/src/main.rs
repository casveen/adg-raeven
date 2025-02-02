use avian3d::prelude::*;
use bevy::math::ops::{cos, sin};
use bevy::{picking::pointer::PointerInteraction, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use core::enemies::ant::AntSpawn;
use core::game_world::{Ground, Wall};
use core::input::input_manager::{
    button, motion, Action, InputManager, InputModeChanged, InputType,
};
use core::player::controller::PlayerSpawn;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            core::CorePlugin,
            MeshPickingPlugin,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, (setup, setup_walls, spawn_ant, register_input))
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.trigger(PlayerSpawn {
        transform: Transform::from_xyz(0., 0., 0.),
    });

    // ground
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        Ground,
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

pub fn setup_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut t = Transform::from_xyz(3.0, 1.0, 4.0);
    t.scale = Vec3::new(2.0, 1.0, 2.0);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(t.scale))),
        MeshMaterial3d(materials.add(Color::srgb_u8(190, 255, 220))),
        t,
        Collider::cuboid(t.scale.x, t.scale.y, t.scale.z),
        RigidBody::Static,
        Wall,
    ));

    // test rigidbody
    let mut t = Transform::from_xyz(1.0, 4.0, 2.0);
    let an = 60.0_f32.to_radians();
    let a = sin(an / 2.);
    let d = Vec3::new(1., 2., 1.).normalize();
    t.rotation = Quat::from_xyzw(a * d.x, a * d.y, a * d.z, cos(an / 2.));
    t.scale = Vec3::new(1.0, 1.0, 1.0);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(t.scale))),
        MeshMaterial3d(materials.add(Color::srgb_u8(190, 255, 220))),
        t,
        Collider::cuboid(t.scale.x, t.scale.y, t.scale.z),
        RigidBody::Dynamic,
        CollidingEntities::default(),
    ));
}

pub fn spawn_ant(mut commands: Commands) {
    commands.trigger(AntSpawn {
        transform: Transform::from_xyz(0., 1.0, 0.),
    });
}

fn draw_cursor(
    pointers: Query<&PointerInteraction>,
    mut gizmos: Gizmos,
    ground_entity: Query<Entity, With<Ground>>,
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
        if ground_entity.contains(*entity) {
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
