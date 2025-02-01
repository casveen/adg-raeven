use bevy::prelude::*;

use crate::player::states::cordycept::CordyCeptMovement;

#[derive(Component)]
pub struct Ant;

#[derive(Event)]
pub struct AntSpawn {
    transform: Transform,
}

/**
 * tmp method
 * impl this in main.rs for each application
 */
pub fn startup(mut commands: Commands) {
    commands.trigger(AntSpawn {
        transform: Transform::from_xyz(0., 1.0, 0.),
    });
}

pub fn spawn_ant(
    event: Trigger<AntSpawn>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(190, 0, 180))),
        event.event().transform,
        Ant,
        // PickingBehavior::IGNORE,
    ));
}

pub fn observe_cordyceptmovement(
    event: Trigger<CordyCeptMovement>,
    mut cordycepted_ants: Query<&mut Transform, With<Ant>>,
) {
    for mut ant in cordycepted_ants.iter_mut() {
        ant.translation += event.event().0;
    }
}
