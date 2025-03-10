use bevy::prelude::*;
use crate::utils::grid::Direction;
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
struct WorldGrid {
    grid_size: f32
}

#[derive(Resource, PartialEq)]
enum WorldMovementState {
    Stopped, 
    Moving,
    // Free? free movement when not in puzzle mode
}

impl Default for WorldGrid {
    fn default() -> Self {
        WorldGrid{grid_size:2.0}
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
#[derive(Event)]
struct OnPlayerMove {
    direction: Direction
}

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

// Component for entities that are moving. Used to keep track of WHICH, entities are moving 
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Moving {
    direction: Direction, 
    speed: f32
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
    player_moved: Trigger<OnPlayerMove>,
    mut commands: Commands,
    mut creature_query: Query<(Entity, &MovingCreature)>,
    mut player_query: Query<&Player>,
    mut world_state: ResMut<WorldMovementState>,
) {
    //let player=player_query.get_;
    let player_direction: &Direction = &player_moved.direction;
    for (entity, creature) in creature_query.iter_mut() {
        //set movement TODO do in initial movement
        let new_direction: Direction = match creature {
            MovingCreature::Player => player_direction.clone(),
            MovingCreature::Ant => player_direction.clone(),
            MovingCreature::Spider => player_direction.clone().opposite(),
            MovingCreature::Rolypoly => player_direction.clone(),
            //Creature::Snake => player_direction,
            //Creature::Wasp => player_direction,
            //Creature::Tick => player_direction,
        };
        commands.entity(entity).insert(
            Moving{direction: new_direction, speed: 2.0}
        );
    }
    let mut w = world_state.into_inner();
    *w = WorldMovementState::Moving;
}

// let teh creatures move! 
fn moving_creatures_system(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &MovingCreature, &Moving, &mut Transform)>,
    world_grid: Res<WorldGrid>,
) {
    for (entity, creature, moving, mut transform) in creature_query.iter_mut() {

        // move the creature, and check if it stops


        let grid_size = world_grid.grid_size;
        let mut translation = transform.translation;
    
        //closest grid point in moving direction
        let closest_grid_point = (translation/grid_size).round()*grid_size;
        let diff_to_closest_grid_point_before_movement = translation-closest_grid_point;
    
        //move the creature one FRAME with movement
        let Moving{direction, speed} = moving;
        translation += Vec3::from(direction.clone())*speed; // *dt?
    
        let diff_to_closest_grid_point_after_movement = translation-closest_grid_point;
    
        //if we've just now passed through the closest grid point, one of the diffs have changed sign
        if diff_to_closest_grid_point_before_movement.signum() != diff_to_closest_grid_point_after_movement.signum() {
            //snap to grid, and stop moving
            translation = closest_grid_point;
            commands.entity(entity).remove::<Moving>(); // TODO handle safely
            transform.translation=translation; // TODO this might not be ideal... can we get a reference, or is it technically primitive?
        }
    }
}

// let teh creatures move! 
fn look_for_stopped_world_system(
    commands: Commands,
    moving_creatures_query: Query<&Moving>,
    mut world_state: ResMut<WorldMovementState>,
) {
    // if the world state was moving, anf the query is empty, the world state will be stopped
    let world_state = world_state.as_mut();
    if moving_creatures_query.is_empty() && (*world_state == WorldMovementState::Moving) {
        //let w = world_state.as_mut();
        //w = WorldMovementState::Stopped.into();
        //let mut w = world_state.into_inner();
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
