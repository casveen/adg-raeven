use bevy::prelude::*;

use super::puffy_shrooms::SpawnPuffyShroomOnEntity;

pub(super) struct FertileGroundPlugin;
impl Plugin for FertileGroundPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FertileGround>()
            .add_observer(get_closest_fertile_ground);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct FertileGround;

#[derive(Event)]
pub struct SpawnFertileGround {
    // Player transform basically, or whoever triggers this
    pub request_instigator_transform: GlobalTransform,
}

pub const FERTILE_GROUND_SPAWN_DISTANCE: f32 = 1.0;

// Spawn PuffyShrooms on fertile ground and remove component
fn get_closest_fertile_ground(
    trigger: Trigger<SpawnFertileGround>,
    q_fertile_grounds: Query<(Entity, &GlobalTransform), With<FertileGround>>,
    mut commands: Commands,
) {
    info!("Get closest fertile ground");
    let mut distance = f32::MAX;
    let mut fertile_ground_entity: Entity = Entity::PLACEHOLDER;
    let mut fertile_ground_transform = &GlobalTransform::IDENTITY;
    let instigator_transform = trigger.event().request_instigator_transform;
    for (ent, gtransform) in q_fertile_grounds.iter() {
        let v = gtransform.compute_transform().translation - instigator_transform.translation();
        let new_dist = v.length_squared();
        if new_dist < distance {
            distance = new_dist;
            fertile_ground_entity = ent;
            fertile_ground_transform = gtransform;
        }
    }

    // Ensure player is close enough to actually spawn
    // TODO? change once grid system is running?
    if distance < FERTILE_GROUND_SPAWN_DISTANCE && fertile_ground_entity != Entity::PLACEHOLDER {
        info!("Spawning fertile ground");
        commands
            .entity(fertile_ground_entity)
            .remove::<FertileGround>();
        commands
            .entity(fertile_ground_entity)
            .trigger(SpawnPuffyShroomOnEntity(
                fertile_ground_entity,
                fertile_ground_transform.compute_transform(),
            ));
    }
}
