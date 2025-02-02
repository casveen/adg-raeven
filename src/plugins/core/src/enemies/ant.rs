use std::{any::Any, time::Duration};

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{game_world::Wall, player::states::cordycept::CordyCeptMovement};

#[derive(Component)]
pub struct Ant;

#[derive(Component)]
#[require(Transform)]
pub struct AntSpawner {
    max_ants: u8,
    current_num_ants: u8,
}
impl AntSpawner {
    pub fn new(max_ants: u8) -> Self {
        Self {
            max_ants,
            current_num_ants: 0,
        }
    }

    fn increment(&mut self) {
        self.current_num_ants += 1
    }

    fn should_spawn(&self) -> bool {
        self.current_num_ants < self.max_ants
    }
}

#[derive(Component, Reflect)]
pub(super) struct AntRespawnTimer {
    timer: Timer,
}

#[derive(Event)]
pub(super) struct SpawnAnt {
    pub transform: Transform,
}

#[derive(Event)]
pub struct KillAnt;

pub(super) fn spawner_evaluate_spawning(
    mut query: Query<(Entity, &mut AntSpawner), Without<AntRespawnTimer>>,
    mut commands: Commands,
) {
    for (entity, mut spawner) in query.iter_mut().filter(|(_, s)| s.should_spawn()) {
        spawner.increment();
        commands.entity(entity).insert((AntRespawnTimer {
            timer: Timer::new(Duration::from_millis(1500), TimerMode::Once),
        },));
    }
}

pub(super) fn respawn_timer_run_if(query: Query<(), With<AntRespawnTimer>>) -> bool {
    !query.is_empty()
}

pub(super) fn respawn_timer(
    query: Query<(Entity, &Transform, &AntRespawnTimer)>,
    mut commands: Commands,
) {
    for (entity, transform, _) in query.iter().filter(|(_, _, t)| t.timer.finished()) {
        commands.entity(entity).remove::<AntRespawnTimer>();

        commands.entity(entity).trigger(SpawnAnt {
            transform: transform.with_scale(Vec3::ONE * 1.1),
        });
    }
}

pub(super) fn spawn_ant(
    event: Trigger<SpawnAnt>,
    spawner: Single<Entity>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let t = event.event().transform;
    let new_ant = commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::from_size(t.scale))),
            MeshMaterial3d(materials.add(Color::srgb_u8(190, 0, 180))),
            event.event().transform,
            Collider::cuboid(t.scale.x, t.scale.y, t.scale.z),
            Ant,
            CollidingEntities::default(),
        ))
        .id();
    commands.entity(*spawner).add_child(new_ant);
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
                commands.entity(entity).trigger(KillAnt);
            }
        }
    }
}

pub fn kill_ant(
    _: Trigger<KillAnt>,
    // spawner: Single<&Parent>,
    antity: Single<Entity>,
    mut commands: Commands,
) {
    commands.entity(*antity).remove_parent();
    // commands.entity(spawner.get()).insert((
    //     RespawnInstance {
    //         timer: Timer::new(Duration::from_millis(1500), TimerMode::Once),
    //     },
    //     AntSpawnInstance,
    // ));
}
