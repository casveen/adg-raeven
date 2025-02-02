use std::{any::Any, time::Duration};

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{game_world::Wall, player::states::cordycept::CordyCeptMovement};

#[derive(Component)]
pub struct Ant;

#[derive(Component)]
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
    fn decrement(&mut self) {
        self.current_num_ants -= 1
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
            timer: Timer::from_seconds(1.5, TimerMode::Once),
        },));
    }
}

pub(super) fn tick_spawn_timers(mut query: Query<&mut AntRespawnTimer>, time: Res<Time>) {
    for mut timer in query.iter_mut() {
        timer.timer.tick(time.delta());
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
            transform: *transform,
        });
    }
}

pub(super) fn spawn_ant(
    event: Trigger<SpawnAnt>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut t = event.event().transform;
    t.scale = Vec3::ONE * 1.1;
    warn!("WHAT TRANSFORM IS WRONG??: {:?}", t);
    let new_ant = commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::from_size(t.scale))),
            MeshMaterial3d(materials.add(Color::srgb_u8(190, 0, 180))),
            t,
            Collider::cuboid(t.scale.x, t.scale.y, t.scale.z),
            Ant,
            CollidingEntities::default(),
        ))
        .id();
    commands.entity(event.entity()).add_child(new_ant);
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
                debug!("ant wall collision: {}, {:?}", entity, colliding_entities);
                commands.entity(entity).trigger(KillAnt);
            }
        }
    }
}

pub(super) fn kill_ant(
    event: Trigger<KillAnt>,
    q_parent: Query<&Parent>,
    mut q_spawners: Query<&mut AntSpawner>,
    mut commands: Commands,
) {
    let Ok(parent) = q_parent.get(event.entity()) else {
        return;
    };
    let Ok(mut spawner) = q_spawners.get_mut(parent.get()) else {
        return;
    };
    spawner.decrement();
    commands.entity(event.entity()).remove_parent().despawn();
}
