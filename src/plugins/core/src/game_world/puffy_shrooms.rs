// Shroom traps

use avian3d::prelude::{Collider, CollidingEntities};
use bevy::prelude::*;

use super::spore_cloud::SpawnSporeCloud;

pub(super) struct PuffyShroomsPlugin;
impl Plugin for PuffyShroomsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PuffyShroom>()
            .register_type::<PuffyShroomCollision>()
            .add_systems(Update, puffy_shroom_collision)
            .add_observer(spawn_puffy_shroom)
            .add_observer(destroy_puffy_shroom);
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PuffyShroom;

#[derive(Event)]
pub struct SpawnPuffyShroomOnEntity(pub Entity, pub Transform);

fn spawn_puffy_shroom(
    trigger: Trigger<SpawnPuffyShroomOnEntity>,
    mut commands: Commands,
    // tmp visuals
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let SpawnPuffyShroomOnEntity(entity, transform) = trigger.event();
    let child = commands
        .spawn((
            PuffyShroom,
            *transform,
            Collider::cuboid(transform.scale.x, transform.scale.y, transform.scale.z),
            CollidingEntities::default(),
            //
            Mesh3d(meshes.add(Cuboid::from_size(transform.scale))),
            MeshMaterial3d(materials.add(Color::srgb(1., 0., 0.))),
        ))
        .id();
    commands.entity(*entity).add_child(child);
}

#[derive(Event)]
struct DestroyPuffyShroom;

fn destroy_puffy_shroom(
    trigger: Trigger<DestroyPuffyShroom>,
    q_global_transforms: Query<&GlobalTransform, With<PuffyShroom>>,
    mut commands: Commands,
) {
    let Ok(global_transform) = q_global_transforms.get(trigger.entity()) else {
        return;
    };

    commands.trigger(SpawnSporeCloud(global_transform.compute_transform()));
    commands.entity(trigger.entity()).remove_parent().despawn();
}

/// For entities colliding with PuffyShrooms
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PuffyShroomCollision;

fn puffy_shroom_collision(
    q_puffy_shroom: Query<(Entity, &CollidingEntities), With<PuffyShroom>>,
    q_puffy_shroom_collider: Query<(), With<PuffyShroomCollision>>,
    mut commands: Commands,
) {
    for (entity, colliding_entities) in &q_puffy_shroom {
        for colliding_entity in colliding_entities.iter() {
            if q_puffy_shroom_collider.contains(*colliding_entity) {
                debug!("puffy shroom collision {}, {}", entity, colliding_entity);
                commands.entity(entity).trigger(DestroyPuffyShroom);
            }
        }
    }
}
