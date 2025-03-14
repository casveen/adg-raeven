use std::collections::HashMap;

use bevy::math::I8Vec3;
use bevy::prelude::*;
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
 */



/*************
 * RESOURCES *
 *************/

#[derive(Resource)]
struct WorldGrid (
    Vec<Vec<Vec<
        Vec<(Entity, MovingCreature)>
    >>>,
    f32
);

impl WorldGrid {
    fn grid_size(WorldGrid(_,grid_size) :Self) -> f32 {
        grid_size
    }

    fn grid(WorldGrid(grid, _) :Self) -> Vec<Vec<Vec<Vec<(Entity, MovingCreature)>>>> {
        grid
    }

    fn get_creatures_on_coordinate(self, coord: IVec3) -> Option<Vec<(Entity, MovingCreature)>> {
        let creatures = WorldGrid::grid(self)
        .get(0)
        .and_then( |yz| 
            yz
            .get(1)
            .and_then( |z| 
                z
                .get(2)
            )
        );
            
        creatures
    }
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
    moving_from: Vec3, //the translation when the movement started, used to not stop immideately
    movement_type: MovementType
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
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Coordinate(IVec3);


//used for collision checking
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Infected;

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
    mut creature_query: Query<(Entity, &MovingCreature, &Transform, &Routine, Option<&CordyCeptedComponent>)>,
    world_state: ResMut<WorldMovementState>,
) {
    if let WorldMovementState::Stopped = *world_state {
        if let PlayerEvent::Movement(PlayerMovementEvent{motion: Some(player_direction)}) = player_event.event() {
        for (entity, creature, transform, routine, infection) in creature_query.iter_mut() {
            
            let is_infected = match infection {Some(_) => true, _ => false};
            let new_direction: Direction = if is_infected {
                decide_infected_creature_movement(creature, player_direction)
            } else {
                decide_creature_movement(creature, player_direction, routine)
            };
            
            
            commands.entity(entity).insert(
                Moving{
                    direction: new_direction, 
                    speed: 2.0, 
                    moving_from: transform.translation, 
                    movement_type: handle_movement_type(creature)
                }
            );
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
fn decide_creature_movement(creature: &MovingCreature, player_direction: &Direction, routine: &Routine) -> Direction {
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

fn handle_movmenet_type(creature: &MovingCreature) -> MovementType {
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
    mut creature_query: Query<(Entity, &Moving, &mut Transform, &mut Coordinate), With<MovingCreature>>,
    world_grid: Res<WorldGrid>,
    world_state: Res<WorldMovementState>,
    time: Res<Time>,

) {
    if let WorldMovementState::Moving = *world_state {
        for (entity, moving, mut transform, mut coordinate) in creature_query.iter_mut() {
            let p_translation = transform.translation;
            let grid_size = world_grid.1;
        
            let closest_grid_point = (p_translation/grid_size).round()*grid_size;
            let diff_to_closest_grid_point_before_movement = p_translation-closest_grid_point;

            //move the creature one FRAME with movement
            let Moving{direction, speed, moving_from, movement_type} = moving;
            transform.translation += Vec3::from(direction.clone())*speed*time.delta_secs();

            let diff_to_closest_grid_point_after_movement = transform.translation-closest_grid_point;
            
            //if we've just now passed through the closest grid point, one of the diffs have changed sign
            //dont count this when it happens close to the initial position the creature moved from
            if diff_to_closest_grid_point_before_movement.signum() != diff_to_closest_grid_point_after_movement.signum() &&
                *moving_from != p_translation {
                //snap to grid, and stop moving
                transform.translation = closest_grid_point;
                coordinate.0 = (p_translation/grid_size).round().as_ivec3();
                //update the coordinate in the world grid
                


                //if *movement_type != MovementType::UntilCollision {
                commands.entity(entity).remove::<Moving>();

                // UPDATE GRID POSITION

                //} else {
                    //TODO check stopping conditions
                //}

                transform.translation=p_translation; // TODO this might not be ideal... can we get a reference, or is it technically primitive?
            }
        }
    }
}

// let teh creatures move! 
fn look_for_stopped_world_system(
    moving_creatures_query: Query<&Moving>,
    mut world_state: ResMut<WorldMovementState>,
) {
    // if the world state was moving, anf the query is empty, the world state will be stopped
    let world_state = world_state.as_mut();
    if moving_creatures_query.is_empty() && (*world_state == WorldMovementState::Moving) {
        //let w = world_state.as_mut();
        //w = WorldMovementState::Stopped.into();
        //let mut w = world_state.into_inner();
        *world_state = WorldMovementState::Interaction;
    }
}

fn interacting_creatures_system(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &mut Transform, &Coordinate), With<MovingCreature>>,
    world_grid: Res<WorldGrid>,
    world_state: Res<WorldMovementState>,
    time: Res<Time>,

) {
    if let WorldMovementState::Interaction = *world_state {
        for (entity, mut transform) in creature_query.iter_mut() {
            grid_position = coordinate
        }
        //if after interaction, there are any moving creatures left, keep moving
        //*world_state = WorldMovementState::Moving;
        //otherwise, stop and await player movement
        *world_state = WorldMovementState::Stopped;
    }
}



pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<MovingCreature>()
        .init_resource::<WorldMovementState>()
        .init_resource::<WorldGrid>()
        .add_systems(Update, (moving_creatures_system, look_for_stopped_world_system))
        .add_observer(on_world_start_moving);
    }      
}
