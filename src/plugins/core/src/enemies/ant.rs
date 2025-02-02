use avian3d::prelude::*;
use bevy::prelude::*;

use crate::utils::collision_data::get_contactdata_global_position;
use crate::{player::states::cordycept::CordyCeptMovement, static_game_world::wall::Wall};

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

pub fn observe_cordyceptmovement(
    event: Trigger<CordyCeptMovement>,
    mut cordycepted_ants: Query<&mut Transform, With<Ant>>,
) {
    for mut ant in cordycepted_ants.iter_mut() {
        ant.translation += event.event().0;
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct CollisionGizmo;

pub fn ant_collision(
    query: Query<(Entity, &CollidingEntities, &Transform), With<Ant>>,
    transform_query: Query<&Transform>,
    collisions: Res<Collisions>,
    mut gizmos: Gizmos<CollisionGizmo>,
) {
    for (entity, colliding_entities, transform) in &query {
        println!("ant_collision: {}, {:?}", entity, colliding_entities);
        for colliding_entity in colliding_entities.iter() {
            let Some(contacts) = collisions.get(entity, *colliding_entity) else {
                continue;
            };
            for manifold in contacts.manifolds.iter() {
                let Some(contact_data) = manifold.find_deepest_contact() else {
                    return;
                };
                gizmos.cross(
                    Isometry3d::from_translation(get_contactdata_global_position(
                        entity,
                        transform,
                        contacts,
                        contact_data,
                    )),
                    3.0,
                    Color::srgb(1., 0., 0.),
                );

                let Ok(transform) = transform_query.get(*colliding_entity) else {
                    return;
                };
                gizmos.cross(
                    Isometry3d::from_translation(get_contactdata_global_position(
                        *colliding_entity,
                        transform,
                        contacts,
                        contact_data,
                    )),
                    3.0,
                    Color::srgb(0., 0., 1.),
                );
            }
        }
    }
}
