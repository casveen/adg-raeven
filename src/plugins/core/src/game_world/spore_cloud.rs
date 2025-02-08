use avian3d::prelude::*;
use bevy::prelude::*;

pub struct SporeCloudPlugin;
impl Plugin for SporeCloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_spore_cloud)
            .add_observer(spawn_spore_cloud);
    }
}

const LIFETIME: f32 = 5.0;
const SIZE: f32 = 2.0;

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
pub struct SpawnSporeCloud(pub Transform);

fn spawn_spore_cloud(
    trigger: Trigger<SpawnSporeCloud>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        SporeCloud::default(),
        trigger.event().0,
        RigidBody::Static,
        Collider::cuboid(SIZE, SIZE, SIZE), // should be sphere?
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::ONE * SIZE))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.9, 0.7))),
    ));
}
