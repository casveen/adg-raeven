/************
 * HANDLERS *
 ************
 * 
 * 
 * 
 * 
 */

use bevy::{ecs::{entity::Entity, system::{Query, Res}}, math::IVec3};

use crate::{game_world::{environment::{Obstacle, Walkable}, grid::{Coordinate, WorldGrid}}, utils::grid::Direction};
use super::creature_movement::{MovementType, MovingCreature};

//move according to routine
pub fn decide_creature_movement(creature: &MovingCreature, player_direction: &Direction) -> Direction { // , routine: &Routine) -> Direction {
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

pub fn handle_movement_type(creature: &MovingCreature) -> MovementType {
    match creature {
        MovingCreature::Player => MovementType::OneStep,
        MovingCreature::Ant => MovementType::OneStep,
        MovingCreature::Spider => MovementType::OneStep,
        MovingCreature::Rolypoly => MovementType::UntilCollision,
    }
}

//move according to player
pub fn decide_infected_creature_movement(creature: &MovingCreature, player_direction: &Direction) -> Direction {
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

pub fn creature_can_move(
    creature: &MovingCreature, // TODO might need this later
    desired_coordinate: Coordinate,
    obstacle_query: &Query<(Entity, &Obstacle)>,
    walkable_query: &Query<(Entity, &Walkable)>,
    world_grid: &Res<WorldGrid>,
) -> bool {
    let mut able_to_move = false;
    let below_desired_coordinate = Coordinate(desired_coordinate.0-IVec3::Y);

    // There has to be walkable terrain bellow where I want to go
    if let Some(entities) = world_grid.0.get(&below_desired_coordinate) {
        //there is SOMETHING there, but is it walkable?
        if walkable_query.iter_many(entities).count()>0 {
            able_to_move = true;
        }
    }

    // am I allowed to move in the given direction? (this assumes SINGLE step)                
    if let Some(entities) = world_grid.0.get(&desired_coordinate) {
        //there is SOMETHING there, but are there any obstacles?
        if obstacle_query.iter_many(entities).count()>0 {
            able_to_move = false;
        }
    }

    able_to_move
}