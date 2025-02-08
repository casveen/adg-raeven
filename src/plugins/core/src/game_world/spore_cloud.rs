use bevy::prelude::*;
use avian3d::prelude::*;

pub struct SporeCloudPlugin;
impl Plugin for SporeCloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_spore_cloud).add_observer(spawn_spore_cloud);
    }
}

const LIFETIME:f32 = 1.0;
const SIZE: Vec3 = Vec3::new(1.0, 1.0, 1.0);

#[derive(Component)]
pub struct SporeCloud(Timer);
impl Default for SporeCloud {
    fn default() -> Self {
        Self(Timer::from_seconds(LIFETIME, TimerMode::Once))
    }
}

fn tick_spore_cloud(
    mut query: Query<(Entity, &mut SporeCloud)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut spore_cloud) in query.iter_mut() {
        spore_cloud.0.tick(time.delta());
        if spore_cloud.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Event)]
pub struct SpawnSporeCloud(pub GlobalTransform);

fn spawn_spore_cloud(
    trigger: Trigger<SpawnSporeCloud>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        SporeCloud::default(),
        trigger.event().0.compute_transform(),
        RigidBody::Static,
        Collider::cuboid(SIZE.x, SIZE.y, SIZE.z),
        // CollidingEntities::default(),
        Mesh3d(meshes.add(Cuboid::from_size(SIZE))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.9, 0.7))),
    ));
}

