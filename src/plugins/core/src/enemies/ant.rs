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
                    ant_rutine,
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
    collection: Entity,
    first_rutine_point: Entity,
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

fn respawn_timer(
    query: Query<(Entity, &Transform, &AntRespawnTimer, &Children)>,
    q_rutine_collections: Query<Entity, With<AntRutineCollection>>,
    q_rutine_points: Query<(Entity, &Parent), With<AntRutinePoint>>,
    mut commands: Commands,
) {
    for (entity, transform, _, children) in query.iter().filter(|(_, _, t, _)| t.timer.finished()) {
        commands.entity(entity).remove::<AntRespawnTimer>();

        let collection = children
            .iter()
            .find(|c| q_rutine_collections.get(**c).is_ok())
            .unwrap();
        let (point, _) = q_rutine_points
            .iter()
            .find(|(_, p)| p.get() == *collection)
            .unwrap();

        commands.entity(entity).trigger(SpawnAnt {
            transform: *transform,
            collection: *collection,
            first_rutine_point: point,
        });
    }
}

fn spawn_ant(
    event: Trigger<SpawnAnt>,
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
            AntRutineComponent {
                collection: event.collection,
                current_point: event.first_rutine_point,
                action: AntRutineAction::Move(event.first_rutine_point),
            },
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

///
/// Collection of rutine points ants will move to, placed on ant spawner
#[derive(Component)]
#[require(Transform(|| Transform::default()))]
pub struct AntRutineCollection;
impl AntRutineCollection {
    fn get_next_point(&self, current: Entity, children: &Children) -> Entity {
        // assumes all children are AntRutinePoint
        for (i, child) in children.iter().enumerate() {
            if *child == current {
                let c = *children.get((i + 1) % children.len()).unwrap();
                info!("child {} {:?}", (i + 1) % children.len(), c);
                return c;
            }
        }
        unreachable!()
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct AntRutinePoint;

///
/// Ants rutine component
#[derive(Component)]
struct AntRutineComponent {
    collection: Entity,
    current_point: Entity,
    action: AntRutineAction,
}
enum AntRutineAction {
    Move(Entity /* target point */),
    Wait(Entity /* timer entity */),
}
impl AntRutineComponent {
    fn interpolate(&self, transform: &mut Transform, target_position: &Vec3, delta: f32) {
        const ANT_MOVESPEED: f32 = 2.;

        // TODO, actual pathfinding. Smoothbrain linear interpolation for now
        let dir = (target_position - transform.translation).normalize();
        transform.translation += dir * ANT_MOVESPEED * delta;
    }

    fn reached_point(&self, position: &Vec3, target_positon: &Vec3) -> bool {
        const POSITION_MARGIN: f32 = 1.;
        (target_positon - position).length() < POSITION_MARGIN
    }
}
#[derive(Component)]
struct AntRutineWaitTimer(Timer);

fn ant_rutine(
    mut q_antrutine: Query<(&mut AntRutineComponent, &mut Transform), With<Ant>>,
    q_rutine_collections: Query<(&Children, &AntRutineCollection), Without<Ant>>,
    q_rutine_points: Query<(&AntRutinePoint, &Transform), Without<Ant>>,
    mut q_rutine_wait_timers: Query<&mut AntRutineWaitTimer>,
    time: Res<Time>,
    mut commands: Commands,
) {
    const ANT_RUTINE_WAIT: f32 = 1.;

    for (mut ant_rutine, mut transform) in q_antrutine.iter_mut() {
        match ant_rutine.action {
            AntRutineAction::Move(e_point) => {
                let (_, point_transform) = q_rutine_points.get(e_point).unwrap();
                ant_rutine.interpolate(
                    &mut transform,
                    &point_transform.translation,
                    time.delta_secs(),
                );
                if ant_rutine.reached_point(&transform.translation, &point_transform.translation) {
                    info!("Ant reached rutine position");
                    let timer = commands
                        .spawn(AntRutineWaitTimer(Timer::from_seconds(
                            ANT_RUTINE_WAIT,
                            TimerMode::Once,
                        )))
                        .id();
                    ant_rutine.action = AntRutineAction::Wait(timer);
                }
            }
            AntRutineAction::Wait(e_timer) => {
                let mut timer = q_rutine_wait_timers.get_mut(e_timer).unwrap();
                timer.0.tick(time.delta());
                if !timer.0.finished() {
                    continue;
                }
                info!("Ant waited... will now move to next point");
                commands.entity(e_timer).despawn();

                let (children, collection) =
                    q_rutine_collections.get(ant_rutine.collection).unwrap();
                let new_point = collection.get_next_point(ant_rutine.current_point, children);
                ant_rutine.current_point = new_point;
                ant_rutine.action = AntRutineAction::Move(new_point);
            }
        }
    }
}
