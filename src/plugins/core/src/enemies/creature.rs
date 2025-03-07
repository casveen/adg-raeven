use bevy::prelude::*;
use crate::player::controller::Player;

#[derive(Resource)]
struct WorldGrid {
    grid_size: f32
}

#[derive(Resource)]
enum WorldMovementState {
    Stopped, 
    Moving,
    // Free?
}


#[derive(Event)]
struct OnPlayerMove {
    direction: Direction
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
enum Creature {
    Ant,
    Spider,
    Rolypoly,
    //Snake,
    //Wasp,
    //Tick,
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

#[derive(Reflect, Debug, Clone)]
enum Direction {
    North, 
    East, 
    South, 
    West, 
}

impl Direction { //Opposite for Direction
    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East  => Direction::West,
            Direction::South => Direction::North,
            Direction::West  => Direction::East
        }
    }
}

/*impl Into<Vec3> for Direction {
    fn into(self) -> Vec3 {
        match self {
            Direction::North => Vec3::Y,
            Direction::East => Vec3::X,
            Direction::South => -Vec3::Y,
            Direction::West => -Vec3::X,
        }
    }
}*/

impl From<Direction> for Vec3 {
    fn from(direction: Direction) -> Vec3 {
        match direction {
            Direction::North => Vec3::Y,
            Direction::East =>  Vec3::X,
            Direction::South => -Vec3::Y,
            Direction::West =>  -Vec3::X,
        }
    }
}

impl Into<Vec2> for Direction {
    fn into(self) -> Vec2 {
        let v3 : Vec3 = self.into();
        v3.xy()
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Moving {
    direction: Direction, 
    speed: f32
}




/************
 * HANDLERS *
 ************
 * 
 * 
 * 
 * 
 */

/**
 * The creature is assumed to be able to move, stopping the movement is handles elsewhere 
 */
fn move_creature_handler(
    world_grid: Res<WorldGrid>,
    mut commands: Commands,
    entity:     Entity,
    //creature: &Creature, 
    mut transform: Transform, 
    moving:     &Moving) {

    let grid_size = world_grid.grid_size;
    let mut translation = transform.translation;

    //closest grid point in moving direction
    let closest_grid_point = (translation/grid_size).round()*grid_size;
    let diff_to_closest_grid_point_before_movement = translation-closest_grid_point;

    //move the creature one FRAME with movement
    let Moving{direction, speed} = moving;
    translation += Vec3::from(direction.clone())*speed; // *dt?

    let diff_to_closest_grid_point_after_movement = translation-closest_grid_point;

    //if we've passed through the closest grid point, one of the diffs have changed sign
    if diff_to_closest_grid_point_before_movement.signum() != diff_to_closest_grid_point_after_movement.signum() {
        //snap to grid, and stop moving
        translation = closest_grid_point;
        commands.entity(entity);
        transform.translation=translation; // TODO this might not be ideal... can we get a reference, or is it technically primitive?
    }
}

/**
 * the creature has now moved, lets check if it will stop
 * 
 * note that all directions are pure, there is no mixing of directions.
 * 
 
fn stop_moving_creature_handler(creature: Creature, mut transform: Transform) {


};  

fn snap_to_grid(transform: &mut Transform) {
    transform = round(transform/grid_size)*grid_size;
}


fn is_exactly_on_grid() {
    round(transform/grid_size)*grid_size
}*/



/***********
 * SYSTEMS *
 ***********
 * 
 * 
 * 
 * 
 */
fn start_moving_creatures_system(
    mut commands: Commands,
    player_moved: Trigger<OnPlayerMove>,
    mut creature_query: Query<(Entity, &Creature)>,
    mut player_query: Query<&Player>,
) {
    //let player=player_query.get_;
    let player_direction: &Direction = &player_moved.direction;
    for (entity, creature) in creature_query.iter_mut() {
        //set movement TODO do in initial movement
        let new_direction = match creature {
            Creature::Ant => player_direction,
            Creature::Spider => player_direction,
            Creature::Rolypoly => player_direction,
            //Creature::Snake => player_direction,
            //Creature::Wasp => player_direction,
            //Creature::Tick => player_direction,
        };
        commands.entity(entity).insert(
            Moving{direction: new_direction.clone(), speed: 2.0}
        );
    }
}

// let teh creatures move! 
fn moving_creatures_system(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &Creature, &Moving, &mut Transform)>,
    world_grid: Res<WorldGrid>,
) {
    for (entity, creature, moving, mut transform) in creature_query.iter_mut() {
        move_creature_handler(
            world_grid, 
            commands, 
            entity, 
            //creature,
            transform,
            moving
        );

        // handle creature stops
        // stop_moving_creature_handler(creature, movement)
    }
}

// let teh creatures move! 
fn look_for_stopped_world_system(
    mut commands: Commands,
    mut moving_creatures_query: Query<&Moving>,
    mut world_state: ResMut<WorldMovementState>,
) {
    // if the world state was moving, anf the query is empty, the world state will be stopped
    if moving_creatures_query.is_empty() && (world_state.into() == WorldMovementState::Moving) {
        //let w = world_state.as_mut();
        //w = WorldMovementState::Stopped.into();
        let mut w = world_state.into_inner();
        *w = WorldMovementState::Stopped;
    }
}



pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Creature>()
        .add_systems(Update, (moving_creatures_system, look_for_stopped_world_system));
    }      
}
