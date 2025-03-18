use std::collections::HashMap;

use bevy::math::I8Vec3;
use bevy::prelude::*;
use bevy::reflect::Map;
use bevy::utils::HashSet;
use crate::player::controller::{PlayerEvent, PlayerMovementEvent};
use crate::utils::grid::Direction;
use crate::player::states::cordycept::CordyCeptedComponent;

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
 *************/

#[derive(Resource)]
struct WorldGrid (
    HashMap<Coordinate, HashSet<Entity>>,
    f32
);

impl WorldGrid {
    fn grid_size(WorldGrid(_,grid_size) :Self) -> f32 {
        grid_size
    }

    /*fn get_creatures_on_coordinate(self, coord: Coordinate) -> Option<&Vec<Entity>> {
        WorldGrid::grid(self)
        .get(&coord)
    }*/
}

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

/**********
 * EVENTS *
 **********/
 /* correponds to the event where the player STARTS moving */
//#[derive(Event)]
//struct OnPlayerMove {
//    direction: Direction
//}

/**************
 * COMPONENTS *
 **************/
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
enum MovingCreature {
    Player,
    Ant,
    Spider,
    Rolypoly,
    //Snake,
    //Wasp,
    //Tick,
}

#[derive(Reflect, Debug, PartialEq)]
enum MovementType {
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
    moving_from: Vec3, //the translation when the movement started, used to not stop immideately
    //movement_type: MovementType
}

/*fn handle_collision(creature_1: Creature, creature_2:Creature) {
    match (creature_1, creature_2) {
        (Ant, Ant)                        => 0,
        (Ant, Spider) | (Spider, Ant)     => 0,
        (Ant, Rolypoly) | (Rolypoly, Ant) => 0,
        (Ant, Snake) | (Snake, Ant)       => 0,
        (Ant, Wasp) | (Wasp, Ant)         => 0,
        (Ant, Tick) | (Tick, Ant)         => 0,
        (Spider, Spider)                  => 0,
        (Spider, Snake) | (Snake, Spider) => 0,
        (Spider, Rolypoly) | (Rolypoly, Spider) => 0,
        (Spider, Snake) | (Snake, Spider) => 0,
        (Spider, Wasp) | (Wasp, Spider)   => 0,
        (Spider, Tick) | (Tick, Spider)   => 0,
        (Rolypoly, Rolypoly)              => 0,
        (Rolypoly, Snake) | (Snake, Rolypoly) => 0,
        (Rolypoly, Wasp) | (Wasp, Rolypoly) => 0,
        (Rolypoly, Tick) | (Tick, Rolypoly) => 0,
        (Snake, Snake)                      => 0,
        (Snake, Wasp) | (Wasp, Snake)       => 0,
        (Snake, Tick) | (Tick, Snake)       => 0,
        (Wasp, Wasp)                        => 0,
        (Wasp, Tick) | (Tick, Wasp)         => 0,
        (Tick, Tick)                        => 0,
    }
}*/

//used for collision checking
#[derive(Component, Reflect, Debug, Eq, Hash, PartialEq, Clone)]
#[reflect(Component)]
struct Coordinate(IVec3);


#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Infected;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Obstacle;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Walkable;

/************
 * HANDLERS *
 ************
 * 
 * 
 * 
 * 
 */

/***********
 * SYSTEMS *
 ***********
 * 
 * 
 * 
 * 
 */
fn on_world_start_moving(
    player_event: Trigger<PlayerEvent>,
    mut commands: Commands,
    mut creature_query: Query<(Entity, &MovingCreature, &Transform, &Coordinate, Option<&CordyCeptedComponent>)>,
    obstacle_query: Query<(Entity, &Obstacle)>,
    walkable_query: Query<(Entity, &Walkable)>,
    world_grid: Res<WorldGrid>,
    world_state: ResMut<WorldMovementState>,
)   {
    if let WorldMovementState::Stopped = *world_state {
        info!("world is stopped:(");
        if let PlayerEvent::Movement(PlayerMovementEvent{motion: Some(player_direction)}) = player_event.event() {
            info!("Player moved!");
            //info!("enteties moving: {}", creature_query.)
            for (entity, creature, transform, coordinate, infection) in creature_query.iter_mut() {
                info!("DING!");
                // get the desired direction of movement
                let is_infected = match infection {Some(_) => true, _ => false};
                let new_direction: Direction = if is_infected {
                    decide_infected_creature_movement(creature, player_direction)
                } else {
                    decide_creature_movement(creature, player_direction) //, routine)
                };
                

                // TODO: next step handler for creatures here, some creatures might move differently. fex jumping two squares
                let desired_coordinate = Coordinate(coordinate.0+Vec3::from(new_direction.clone()).as_ivec3());
                //let below_desired_coordinate = Coordinate(coordinate.0+Vec3::from(new_direction.clone()).as_ivec3()+IVec3::Y);

                if creature_can_move(creature, desired_coordinate, &obstacle_query, &walkable_query, &world_grid) {
                    info!("OK: I {} was able to move!", entity);
                    // start moving! (actual movement done elsewhere)
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
            let w = world_state.into_inner();
            *w = WorldMovementState::Moving;
        }
    }
}

//move according to player
fn decide_infected_creature_movement(creature: &MovingCreature, player_direction: &Direction) -> Direction {
    match creature {
        MovingCreature::Player => player_direction.clone(),
        MovingCreature::Ant => player_direction.clone(),
        MovingCreature::Spider => player_direction.clone().opposite(),
        MovingCreature::Rolypoly => player_direction.clone(),
        //Creature::Snake => player_direction,
        //Creature::Wasp => player_direction,
        //Creature::Tick => player_direction,
    }
}

//move according to routine
fn decide_creature_movement(creature: &MovingCreature, player_direction: &Direction) -> Direction { // , routine: &Routine) -> Direction {
    match creature {
        MovingCreature::Player => player_direction.clone(),
        MovingCreature::Ant => player_direction.clone(),
        MovingCreature::Spider => player_direction.clone().opposite(),
        MovingCreature::Rolypoly => player_direction.clone(),
        //Creature::Snake => player_direction,
        //Creature::Wasp => player_direction,
        //Creature::Tick => player_direction,
    }
}

fn handle_movement_type(creature: &MovingCreature) -> MovementType {
    match creature {
        MovingCreature::Player => MovementType::OneStep,
        MovingCreature::Ant => MovementType::OneStep,
        MovingCreature::Spider => MovementType::OneStep,
        MovingCreature::Rolypoly => MovementType::UntilCollision,
    }
}
    //creature, player_direction, is_infected);
/**  
 * Fly my babies!
 *
 * Let creatures move, the direction depending on the creatures behaviour
 * 
 * Note how the direction is not specified here. That is decided at the movement START (on_world_start_moving)
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
        for (entity, moving, stepping, mut transform, mut coordinate) in creature_query.iter_mut() {
            info!("I am moving!");
            let p_translation = transform.translation;
            let grid_size = world_grid.1;
            let old_coordinate = coordinate.clone();
            let closest_grid_point = (p_translation/grid_size).round()*grid_size;
            let diff_to_closest_grid_point_before_movement = p_translation-closest_grid_point;

            //move the creature one FRAME with movement
            let Stepping{moving_from} = stepping;
            let Moving{direction, speed, movement_type: _} = moving;

            //move
            transform.translation += Vec3::from(direction.clone())*speed*time.delta_secs();

            //
            let diff_to_closest_grid_point_after_movement = transform.translation-closest_grid_point;
            //if we've just now passed through the closest grid point, one of the diffs have changed sign
            //dont count this when it happens close to the initial position the creature moved from
            if diff_to_closest_grid_point_before_movement.signum() != diff_to_closest_grid_point_after_movement.signum() &&
                *moving_from != p_translation {
                info!("Hit grid!");
                //snap to grid, and stop moving
                transform.translation = closest_grid_point;
                coordinate.0 = (p_translation/grid_size).round().as_ivec3();
                //update the coordinate in the world grid
                
                //if *movement_type != MovementType::UntilCollision {
                commands.entity(entity).remove::<Stepping>();

                // UPDATE GRID POSITION, remove from old, add to new
                world_grid.0
                .entry(old_coordinate)      
                .and_modify(|creatures| { creatures.remove(&entity);});
                world_grid.0
                .entry(coordinate.clone())
                .and_modify(|creatures| {creatures.insert(entity);})
                .or_insert(HashSet::default()).insert(entity);
                
                //
                //let creatures_in_my_space = world_grid.0.get(&coordinate);


                //} else {
                    //TODO check stopping conditions
                //}

                transform.translation=p_translation; // TODO this might not be ideal... can we get a reference, or is it technically primitive?
            }
        }
    }
}

// let teh creatures move! 
fn look_for_stepping_finished(
    stepping_creatures_query: Query<&Stepping>,
    mut world_state: ResMut<WorldMovementState>,
) {
    // if the world state was moving, anf the query is empty, the world state will be stopped
    let world_state = world_state.as_mut();
    if (*world_state == WorldMovementState::Moving) && stepping_creatures_query.is_empty() {
        //let w = world_state.as_mut();
        //w = WorldMovementState::Stopped.into();
        //let mut w = world_state.into_inner();
        info!("All stopped, set to interacting state");
        *world_state = WorldMovementState::Interaction;
    }
}

/**
 * THis should actually be an observer
 */
fn interacting_creatures_system(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &mut Transform, &Coordinate, &Moving, &MovingCreature)>,
    mut moving_creatures_query: Query<Entity, With<MovingCreature>>,
    obstacle_query: Query<(Entity, &Obstacle)>,
    walkable_query: Query<(Entity, &Walkable)>,
    world_grid: Res<WorldGrid>,
    mut world_state: ResMut<WorldMovementState>,

) {
    if let WorldMovementState::Interaction = *world_state {
        info!("interaction state");
        for (entity, 
            mut transform, 
            coordinate, 
            Moving{direction: direction, speed: _ , movement_type: movement_type},
            creature
        ) in creature_query.iter_mut() {
            // THE GIRLS ARE INTERACTING AAA
            let creatures_in_my_space = world_grid.0.get(&coordinate);
            let grid_position = coordinate;

            // THE GIRLS MIGHT CONTIUE MOVING AAA

            //some movement types allows for continuing to move as long as its possible
            if let MovementType::UntilCollision = movement_type {
                let desired_coordinate = Coordinate(coordinate.0+Vec3::from(direction.clone()).as_ivec3());
                if creature_can_move(creature, desired_coordinate, &obstacle_query, &walkable_query, &world_grid) {
                    info!("OK: I {} was able to move!", entity);
                    // start moving! (actual movement done elsewhere)
                    commands.entity(entity).insert(
                        Moving{
                            direction: direction.clone(), 
                            speed: 2.0, 
                            movement_type: handle_movement_type(creature)
                        }
                    ).insert(
                        Stepping{
                            moving_from: transform.translation, 
                        }
                    );
                } else {
                    //the creature wants to move, but cannot, stop moving
                }
            }
        }

        //if after interaction, there are any moving creatures left, keep moving
        //*world_state = WorldMovementState::Moving;
        //otherwise, stop and await player 
        if moving_creatures_query.is_empty() {
            *world_state = WorldMovementState::Stopped;
        } else {
            *world_state = WorldMovementState::Moving;
        }


        
    }



}

fn creature_can_move(
    creature: &MovingCreature, // TODO might need this later
    desired_coordinate: Coordinate,
    obstacle_query: &Query<(Entity, &Obstacle)>,
    walkable_query: &Query<(Entity, &Walkable)>,
    world_grid: &Res<WorldGrid>,
) -> bool {
    let mut able_to_move = false;
    //let desired_coordinate = Coordinate(coordinate.0+Vec3::from(new_direction.clone()).as_ivec3());
    let below_desired_coordinate = Coordinate(desired_coordinate.0-IVec3::Y);

    // There has to be walkable terrain bellow where I want to go
    if let Some(entities) = world_grid.0.get(&below_desired_coordinate) {
        //there is SOMETHING there, but is it walkable
        if walkable_query.iter_many(entities).count()>0 {
            able_to_move = true;
            info!("there is walkable terrain here!");
        }
    }

    // am I allowed to move in the given direction? (this assumes SINGLE step)                
    if let Some(entities) = world_grid.0.get(&desired_coordinate) {
        //there is SOMETHING there, but are there any obstacles?
        if obstacle_query.iter_many(entities).count()>0 {
            able_to_move = false;
            info!("I was unable to move!");
        }
    }

    able_to_move
}

fn init_coordinates(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &mut Transform), (Added<Transform>, Without<Coordinate>)>, //Todo remove movingcreature, register all!
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


pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<MovingCreature>()
        .register_type::<Coordinate>()
        .register_type::<Infected>()
        .register_type::<Moving>()
        .register_type::<Obstacle>()
        .init_resource::<WorldMovementState>()
        .init_resource::<WorldGrid>()
        .add_systems(Update, init_coordinates)
        .add_systems(Update, (moving_creatures_system, look_for_stepping_finished, interacting_creatures_system))
        .add_observer(on_world_start_moving);
    }      
}
