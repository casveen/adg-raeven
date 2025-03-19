use std::collections::HashMap;
use bevy::prelude::*;
use bevy::utils::HashSet;
use crate::player::controller::{PlayerEvent, PlayerMovementEvent};
use crate::utils::grid::Direction;
use crate::player::states::cordycept::CordyCeptedComponent;
use super::handlers::{
    decide_creature_movement, 
    handle_movement_type, 
    decide_infected_creature_movement,
    creature_can_move
};

/***
 * Resources:
 * 
 * Events:
 * OnPlayerMove -> should trigger when player starts moving
 * 
 * 
 * 
 * commands.trigger(OnPlayerMove { direction: Direction });
 * for every creature, set an observer
 * .observe(OnPlayerMover);
 * and creatureCollision
 * 
 * Components:
 * - Moving: for entities that are moving. Moving depends on creature, some move one step, others move until collision etc.
 *           not to be confused with step. The Moving component decides how/how long the creature will step.
 * - Stepping: for entities that are stepping. The step component is what ACTUALLY moves the creature, and is removed as soon as the creature snaps to the next grid 
 * 
 * 
 * 
 * 
 * 
 * TODO_
 * replace wall with obstacle
 * 
 * 
 * 
 */



/*************
 * RESOURCES *
 *************
 *
 * - WorldGrid: resource representing the game world. A grid where entities place themselves, and the size of the grid in 3d space 
 * - WorldMovementState: the current movement state of the world.
 */
#[derive(Resource)]
pub struct WorldGrid (
    pub HashMap<Coordinate, HashSet<Entity>>,
    pub f32
);

#[derive(Resource, PartialEq)]
enum WorldMovementState {
    Stopped, 
    Moving,
    Interaction,
    // Free? free movement when not in puzzle mode
}

impl Default for WorldGrid {
    fn default() -> Self {
        WorldGrid(default(), 1.0)
    }
}

impl Default for WorldMovementState {
    fn default() -> Self {
        WorldMovementState::Stopped
    }
}

/**************
 * COMPONENTS *
 **************/
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub enum MovingCreature {
    Player,
    Ant,
    Spider,
    Rolypoly,
    //Snake,
    //Wasp,
    //Tick,
}

#[derive(Reflect, Debug, PartialEq)]
pub enum MovementType {
    OneStep,
    UntilCollision 
}

// Component for entities that are moving. Used to keep track of WHICH, entities are moving 
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Moving {
    direction: Direction, 
    speed: f32,
    movement_type: MovementType
}

// Component for entities that are moving. Used to keep track of WHICH, entities are moving 
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Stepping {
    moving_from: Vec3, //the translation (in 3d space, not grid) when the movement started, used to not stop immideately
    //movement_type: MovementType
}

//used for collision checking
#[derive(Component, Reflect, Debug, Eq, Hash, PartialEq, Clone)]
#[reflect(Component)]
pub struct Coordinate(pub IVec3);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Infected;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Obstacle;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Walkable;

/************
 * TRIGGERS *
 ***********/

/**
 * A trigger that triggers when the player makes a  first move from a stopped state. 
 * This is when the game world will start to move, and will not stop before every creature has finished its movement(some movements consists of several steps).
 */
fn on_world_start_moving(
    player_event: Trigger<PlayerEvent>,
    mut commands: Commands,
    mut creature_query: Query<(Entity, &MovingCreature, &Transform, &Coordinate, Option<&CordyCeptedComponent>)>,
    obstacle_query: Query<(Entity, &Obstacle)>,
    walkable_query: Query<(Entity, &Walkable)>,
    world_grid: Res<WorldGrid>,
    mut world_state: ResMut<WorldMovementState>,
)   {
    if let WorldMovementState::Stopped = *world_state {
        if let PlayerEvent::Movement(PlayerMovementEvent{motion: Some(player_direction)}) = player_event.event() {
            //the player has given input, lets move ALL creatures
            for (entity, creature, transform, coordinate, infection) in creature_query.iter_mut() {
                // get the desired direction of movement. Depends on creature and infection
                let is_infected = match infection {Some(_) => true, _ => false};
                let new_direction: Direction = if is_infected {
                    decide_infected_creature_movement(creature, player_direction)
                } else {
                    decide_creature_movement(creature, player_direction) //, routine)
                };
                
                // TODO: next step handler for creatures here, some creatures might move differently. fex jumping two squares
                let desired_coordinate = Coordinate(coordinate.0+Vec3::from(new_direction.clone()).as_ivec3());

                /* if the creature is able to move from here, add the Moving component
                * Any creature that just started moving will also get the stepping component, and actually start moving.
                * Adding Stepping could've been done in a separate system, avoiding som code duplication, but done here for clarity */
                if creature_can_move(creature, desired_coordinate, &obstacle_query, &walkable_query, &world_grid) {
                    commands.entity(entity).insert(
                        Moving{
                            direction: new_direction.clone(), 
                            speed: 2.0, 
                            movement_type: handle_movement_type(creature)
                        }
                    ).insert(
                        Stepping{
                            moving_from: transform.translation, 
                        }
                    );
                }
            }

            // The components have been set, and creatures are free to move in their desired directions.
            *world_state = WorldMovementState::Moving;
        }
    }
}

/***********
 * SYSTEMS *
 **********/

/**  
 * Fly my babies!
 *
 * Let creatures move, the direction depending on the creatures behaviour. Snaps creature to grid and removes Stepping component when snapped
 * Note that, while the creature will stop stepping when it hits its desired grid point, it might not stop moving, depending on the "movement_type".
 * 
 * find the closest grid point, measure diff to it before and after movement. If the signs in the two diffs differ in any way, 
 * we have crossed the grid point and can snap to it(and stop moving)
 */ 
fn moving_creatures_system(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &Moving, &Stepping, &mut Transform, &mut Coordinate), With<MovingCreature>>,
    mut world_grid: ResMut<WorldGrid>,
    world_state: Res<WorldMovementState>,
    time: Res<Time>,

) {
    if let WorldMovementState::Moving = *world_state {
        let grid_size = world_grid.1;
        for (entity, moving, stepping, mut transform, mut coordinate) in creature_query.iter_mut() {
            //store some data for the previous position, getting diff to closest gridpoint
            let previous_translation = transform.translation;
            let previous_coordinate = coordinate.clone();
            let closest_grid_point = (previous_translation/grid_size).round()*grid_size;
            let diff_to_closest_grid_point_before_movement = previous_translation-closest_grid_point;

            //move the creature ONE STEP with movement
            let Stepping{moving_from} = stepping;
            let Moving{direction, speed, movement_type: _} = moving;
            transform.translation += Vec3::from(direction.clone())*speed*time.delta_secs();
            let diff_to_closest_grid_point_after_movement = transform.translation-closest_grid_point;

            //if we've just now passed through the closest grid point, one of the diffs have changed sign
            //dont count this when it happens close to the initial position the creature moved from
            if diff_to_closest_grid_point_before_movement.signum() != diff_to_closest_grid_point_after_movement.signum() &&
                *moving_from != previous_translation { // TODO: this last check is unstable, and subject to rounding errors as we are comparing floats
                //snap to grid, stop stepping and update coordinate to world_grid
                transform.translation = closest_grid_point;
                commands.entity(entity).remove::<Stepping>();

                // UPDATE GRID POSITION, remove from old, add to new
                coordinate.0 = (previous_translation/grid_size).round().as_ivec3();
                world_grid.0
                .entry(previous_coordinate)      
                .and_modify(|creatures| { creatures.remove(&entity);});
                world_grid.0
                .entry(coordinate.clone())
                .and_modify(|creatures| {creatures.insert(entity);})
                .or_insert(HashSet::default()).insert(entity);
            }
        }
    }
}

/** 
 * Look for when all creatures have stopped stepping from the moving state
 * 
 * This should be an observer
*/
fn look_for_stepping_finished_system(
    stepping_creatures_query: Query<&Stepping>,
    mut world_state: ResMut<WorldMovementState>,
) {
    // if the world state was moving, and the stepping(!) query is empty, the world state will go into interaction
    let world_state = world_state.as_mut();
    if (*world_state == WorldMovementState::Moving) && stepping_creatures_query.is_empty() {
        *world_state = WorldMovementState::Interaction;
    }
}

/**
 * Let creatures interact
 * 
 * This should actually be an observer, too
 */
fn interacting_creatures_system(
    mut commands: Commands,
    moving_creature_query: Query<(Entity, &Transform, &Coordinate, &Moving, &MovingCreature)>,
    moving_creatures_query: Query<Entity, (With<MovingCreature>, With<Moving>)>,
    obstacle_query: Query<(Entity, &Obstacle)>,
    walkable_query: Query<(Entity, &Walkable)>,
    world_grid: Res<WorldGrid>,
    mut world_state: ResMut<WorldMovementState>,

) {
    if let WorldMovementState::Interaction = *world_state {
        // Go through all creatures that have just stepped
        for (entity, 
            transform, 
            coordinate, 
            Moving{direction, speed: _, movement_type},
            creature
        ) in moving_creature_query.iter() {
            //some movement types allows for continuing to move as long as its possible
            match movement_type {
                MovementType::UntilCollision => {
                    let desired_coordinate = Coordinate(coordinate.0+Vec3::from(direction.clone()).as_ivec3());
                    if creature_can_move(creature, desired_coordinate, &obstacle_query, &walkable_query, &world_grid) {
                        // continue stepping! (actual movement done elsewhere)
                        commands.entity(entity).insert(
                            Stepping{
                                moving_from: transform.translation, 
                            }
                        );
                    } else {
                        //the creature wants to move, but cannot, stop moving
                        commands.entity(entity).remove::<Moving>();
                    }
                }
                MovementType::OneStep => {
                    commands.entity(entity).remove::<Moving>();
                }
            }
        }

        //if after interaction, there are any moving creatures left, keep moving
        //otherwise, stop and await player 
        if moving_creatures_query.is_empty() {
            *world_state = WorldMovementState::Stopped;
        } else {
            *world_state = WorldMovementState::Moving;
        }
    }   
}

fn init_coordinates(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &mut Transform), (Added<Transform>, Without<Coordinate>)>,
    mut world_grid: ResMut<WorldGrid>,
) {
    for (entity, transform) in creature_query.iter_mut() {
        let grid_size =  world_grid.1;
        let coordinate = Coordinate(
            (transform.translation/grid_size).round().as_ivec3()
        );
        commands.entity(entity).insert(coordinate.clone());

        world_grid.0
        .entry(coordinate)
        .and_modify(|creatures| {creatures.insert(entity);})
        .or_insert(HashSet::default()).insert(entity);
    }
}

pub struct CreatureMovementPlugin;
impl Plugin for CreatureMovementPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<MovingCreature>()
        .register_type::<Coordinate>()
        .register_type::<Infected>()
        .register_type::<Moving>()
        .register_type::<Obstacle>()
        .register_type::<Walkable>()
        .init_resource::<WorldMovementState>()
        .init_resource::<WorldGrid>()
        .add_systems(Update, (
            init_coordinates, 
            moving_creatures_system, 
            look_for_stepping_finished_system, 
            interacting_creatures_system)
        )
        .add_observer(on_world_start_moving);
    }      
}