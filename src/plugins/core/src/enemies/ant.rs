use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{game_world::Wall, player::states::cordycept::CordyCeptMovement};

#[derive(Component)]
pub struct Ant;

#[derive(Event)]
pub struct AntSpawn {
    pub transform: Transform,
}

pub fn spawn_ant(
    event: Trigger<AntSpawn>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let t = event.event().transform;
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(t.scale))),
        MeshMaterial3d(materials.add(Color::srgb_u8(190, 0, 180))),
        event.event().transform,
        Collider::cuboid(t.scale.x, t.scale.y, t.scale.z),
        Ant,
        CollidingEntities::default(),
    ));
}

pub fn cordyceptmovement(
    event: Trigger<CordyCeptMovement>,
    mut cordycepted_ants: Query<&mut Transform, With<Ant>>,
) {
    for mut ant in cordycepted_ants.iter_mut() {
        ant.translation += event.event().0;
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct CollisionGizmo;

pub fn wall_collision(
    ant_query: Query<(Entity, &CollidingEntities), With<Ant>>,
    wall_query: Query<Entity, With<Wall>>,
    mut commands: Commands,
) {
    for (entity, colliding_entities) in &ant_query {
        for colliding_entity in colliding_entities.iter() {
            if wall_query.contains(*colliding_entity) {
                println!("ant wall collision: {}, {:?}", entity, colliding_entities);
                kill_ant(entity, &mut commands);
            }
        }
    }
}

fn kill_ant(antity: Entity, commands: &mut Commands) {
    commands.entity(antity).despawn();

    // todo: make dedicated spawner
    commands.trigger(AntSpawn {
        transform: Transform::from_xyz(0., 1.0, 0.),
    });
}
