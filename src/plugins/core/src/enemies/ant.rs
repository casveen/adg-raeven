use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    game_world::{
        puffy_shrooms::PuffyShroomCollision,
        spore_cloud::{SpawnSporeCloud, SporeCloud},
        Wall,
    },
    player::states::cordycept::{CordyCeptMovement, CordyCeptedComponent},
};

pub struct AntPlugin;
impl Plugin for AntPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.register_type::<AntRespawnTimer>()
            .add_systems(
                Update,
                (
                    wall_collision,
                    spore_cloud_collision,
                    anthill_entry_collision,
                    spawner_evaluate_spawning,
                    tick_spawn_timers,
                    recent_movement_in_anthill_cooldown_timer
                        .run_if(recent_movement_in_anthill_cooldown_timer_run_if),
                    respawn_timer.run_if(respawn_timer_run_if),
                ),
            )
            .add_observer(spawn_ant)
            .add_observer(kill_ant)
            .add_observer(cordyceptmovement)
            .add_observer(teleport_ant);
    }
}

#[derive(Component)]
struct Ant;

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
struct AntRespawnTimer {
    timer: Timer,
}

#[derive(Event)]
struct SpawnAnt {
    transform: Transform,
}

#[derive(Event)]
struct KillAnt;

fn spawner_evaluate_spawning(
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

fn tick_spawn_timers(mut query: Query<&mut AntRespawnTimer>, time: Res<Time>) {
    for mut timer in query.iter_mut() {
        timer.timer.tick(time.delta());
    }
}

fn respawn_timer_run_if(query: Query<(), With<AntRespawnTimer>>) -> bool {
    !query.is_empty()
}

fn respawn_timer(query: Query<(Entity, &Transform, &AntRespawnTimer)>, mut commands: Commands) {
    for (entity, transform, _) in query.iter().filter(|(_, _, t)| t.timer.finished()) {
        commands.entity(entity).remove::<AntRespawnTimer>();

        commands.entity(entity).trigger(SpawnAnt {
            transform: *transform,
        });
    }
}

fn spawn_ant(
    event: Trigger<SpawnAnt>, // TODO; add rutine here
    mut commands: Commands,
    // tmp visuals
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let t = event.event().transform.with_scale(Vec3::ONE * 1.6);
    let new_ant = commands
        .spawn((
            Ant,
            // transform is inherited from parent
            Transform::default(),
            Collider::cuboid(t.scale.x, t.scale.y, t.scale.z),
            CollidingEntities::default(),
            PuffyShroomCollision, // todo, should be removed for cordycepted ant?
            //
            Mesh3d(meshes.add(Cuboid::from_size(t.scale))),
            MeshMaterial3d(materials.add(Color::srgb_u8(190, 0, 180))),
        ))
        .id();
    commands.entity(event.entity()).add_child(new_ant);
}

fn cordyceptmovement(
    event: Trigger<CordyCeptMovement>,
    mut cordycepted_ants: Query<(&Parent, &mut Transform), (With<Ant>, With<CordyCeptedComponent>)>,
    q_spawner: Query<&Transform, (With<AntSpawner>, Without<Ant>)>,
) {
    let movement = event.0;
    for (parent, mut ant) in cordycepted_ants.iter_mut() {
        let Ok(spawner) = q_spawner.get(parent.get()) else {
            continue;
        };
        ant.translation += spawner.rotation.inverse() * movement;
    }
}

fn wall_collision(
    ant_query: Query<(Entity, &CollidingEntities), With<Ant>>,
    wall_query: Query<(), With<Wall>>,
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

fn spore_cloud_collision(
    q_ant: Query<(Entity, &CollidingEntities), (With<Ant>, Without<CordyCeptedComponent>)>,
    q_spore_cloud: Query<Entity, With<SporeCloud>>,
    mut commands: Commands,
) {
    for (entity, colliding_entities) in &q_ant {
        for colliding_entity in colliding_entities.iter() {
            if let Ok(spore_cloud) = q_spore_cloud.get(*colliding_entity) {
                debug!("ant spore_cloud collision: {}, {:?}", entity, spore_cloud);
                commands.entity(entity).insert(CordyCeptedComponent);
                commands.entity(spore_cloud).despawn();
            }
        }
    }
}

fn kill_ant(
    event: Trigger<KillAnt>,
    q_parent: Query<(&Parent, &GlobalTransform)>,
    q_cordycepted: Query<&CordyCeptedComponent>,
    mut q_spawners: Query<&mut AntSpawner>,
    mut commands: Commands,
) {
    let Ok((parent, global_transform)) = q_parent.get(event.entity()) else {
        return;
    };
    let Ok(mut spawner) = q_spawners.get_mut(parent.get()) else {
        return;
    };

    if let Ok(_) = q_cordycepted.get(event.entity()) {
        info!("spawn spore cloud");
        commands.trigger(SpawnSporeCloud(global_transform.compute_transform()));
    }

    // destroy ant
    spawner.decrement();
    commands.entity(event.entity()).remove_parent().despawn();
}

// Composed of two entries
#[derive(Component)]
#[require(Transform)]
pub struct AntHillPipe;
#[derive(Component)]
#[require(Transform)]
pub struct AntHillEntry {
    pub other_entry: Entity,
}
#[derive(Event)]
struct AntHillMovement {
    hill_entry_global_transform: GlobalTransform,
}
#[derive(Component)]
struct RecentMovementInAntHill {
    cooldown_timer: Timer,
}
impl Default for RecentMovementInAntHill {
    fn default() -> Self {
        Self {
            cooldown_timer: Timer::from_seconds(2., TimerMode::Once),
        }
    }
}

fn recent_movement_in_anthill_cooldown_timer_run_if(q: Query<&RecentMovementInAntHill>) -> bool {
    !q.is_empty()
}

fn recent_movement_in_anthill_cooldown_timer(
    mut timers: Query<(Entity, &mut RecentMovementInAntHill)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut timer) in timers.iter_mut() {
        timer.cooldown_timer.tick(time.delta());
        if timer.cooldown_timer.finished() {
            commands.entity(entity).remove::<RecentMovementInAntHill>();
        }
    }
}

fn anthill_entry_collision(
    ant_query: Query<(Entity, &CollidingEntities), (With<Ant>, Without<RecentMovementInAntHill>)>,
    hill_query: Query<(Entity, &GlobalTransform, &AntHillEntry), With<AntHillEntry>>,
    mut commands: Commands,
) {
    for (entity, colliding_entities) in &ant_query {
        for colliding_entity in colliding_entities.iter() {
            let Ok((_, _, hill_entry)) = hill_query.get(*colliding_entity) else {
                continue;
            };
            let Ok((_, other_hill_transform, _)) = hill_query.get(hill_entry.other_entry) else {
                continue;
            };

            commands
                .entity(entity)
                .insert(RecentMovementInAntHill::default())
                .trigger(AntHillMovement {
                    hill_entry_global_transform: *other_hill_transform,
                });
        }
    }
}

fn teleport_ant(
    movement: Trigger<AntHillMovement>,
    mut query: Query<(&mut Transform, &Parent), With<RecentMovementInAntHill>>,
    q_parents: Query<&Transform, (With<AntSpawner>, Without<RecentMovementInAntHill>)>,
) {
    let (mut ant_transform, parent) = query.get_mut(movement.entity()).unwrap();
    let spawner_transform = q_parents.get(parent.get()).unwrap();
    let entry_transform = movement.event().hill_entry_global_transform;

    ant_transform.translation = entry_transform.translation() - spawner_transform.translation;
    ant_transform.rotation = entry_transform.rotation() * spawner_transform.rotation.conjugate();
}

/// Ant Rutine
/// move between locations,
/// once reached it waits for a short period of time
struct AntRutinePoint {
    /// Entity whose transform point is tied to
    entity: Entity,

    // transform in ant_space that point is at
    transform: Transform,

    // TODO
    // Guided path towards a point, e.g. using a* ant will try to move
    // according to the path
    // guided_path:
    //
    /// how long, in seconds, the ant will wait at a given point
    wait_time: f32,
}
#[derive(Component)]
struct AntRutineCollection {
    points: Vec<AntRutinePoint>,
}
impl AntRutineCollection {
    pub fn get_next_point(&self, current_point: &AntRutinePoint) -> &AntRutinePoint {
        for (i, point) in self.points.iter().enumerate() {
            if point.entity == current_point.entity {
                if let Some(point) = self.points.get((i + 1) % self.points.len()) {
                    return point;
                }
            }
        }
        unreachable!()
    }
}

#[derive(Component)]
struct AntRutine {
    point: &'static AntRutinePoint,
}

#[derive(Component)]
struct AntRutineMoveToPoint(pub &'static AntRutinePoint);
impl AntRutineMoveToPoint {
    fn interpolate(&self, transform: &mut Transform, speed: f32) {
        // TODO follow a path
        // linear interpolation for now
        let dir = (self.0.transform.translation - transform.translation).normalize();
        transform.translation += dir * speed
    }

    fn reached_end(&self, transform: &Transform) -> bool {
        let length = (self.0.transform.translation - transform.translation).length();
        const ACCEPTED_LENGTH: f32 = 2.;
        length < ACCEPTED_LENGTH
    }
}

#[derive(Component)]
struct AntRutineWaitAtPoint(Timer);
impl AntRutineWaitAtPoint {
    fn new(time: f32) -> Self {
        Self(Timer::from_seconds(time, TimerMode::Once))
    }
}

fn antrutine_move(
    query: Query<
        (Entity, &Parent, &AntRutine, &AntRutineMoveToPoint),
        (With<AntRutine>, With<AntRutineMoveToPoint>),
    >,
    mut q_transform: Query<&mut Transform, With<Ant>>,
    mut commands: Commands,
) {
    const ANT_MOVEMENT_SPEED: f32 = 1.;

    for (entity, parent, rutine, rutine_movement) in query.iter() {
        if let Ok(mut transform) = q_transform.get_mut(parent.get()) {
            rutine_movement.interpolate(&mut transform, ANT_MOVEMENT_SPEED);
            if rutine_movement.reached_end(&transform) {
                commands.entity(entity).remove::<AntRutineMoveToPoint>();
                commands
                    .entity(entity)
                    .insert(AntRutineWaitAtPoint::new(rutine.point.wait_time));
            }
        }
    }
}

fn antrutine_wait(
    mut query: Query<
        (Entity, &Parent, &mut AntRutine, &mut AntRutineWaitAtPoint),
        (With<AntRutine>, With<AntRutineWaitAtPoint>),
    >,
    q_rutine_collection: Query<&AntRutineCollection>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, parent, mut rutine, mut rutine_wait) in query.iter_mut() {
        rutine_wait.0.tick(time.delta());
        if rutine_wait.0.finished() {
            if let Ok(collection) = q_rutine_collection.get(parent.get()) {
                rutine.point = collection.get_next_point(rutine.point);
                commands.entity(entity).remove::<AntRutineWaitAtPoint>();
                commands
                    .entity(entity)
                    .insert(AntRutineMoveToPoint(rutine.point));
            }
        }
    }
}
