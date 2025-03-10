use bevy::{math::{Vec2, Vec3, Vec3Swizzles}, reflect::Reflect};

/***********
 * HELPERS *
 ***********/
#[derive(Reflect, Debug, Clone)]
pub(crate) enum Direction {
    North, 
    East, 
    South, 
    West,
}
 
impl Direction { //Opposite for Direction
    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East  => Direction::West,
            Direction::South => Direction::North,
            Direction::West  => Direction::East
        }
    }
}
 
impl From<bevy::prelude::Vec2> for Direction {
    fn from(vector: Vec2) -> Direction {
        match vector.to_angle()%360.0 {
            45.0..135.0      => Direction::North,
            135.0..225.0     => Direction::West,
            225.0..315.0     => Direction::South,
            _                => Direction::East,
        }
    }
}

impl From<bevy::prelude::Vec3> for Direction {
    fn from(vector: Vec3) -> Direction {
        Direction::from(vector.xy())
    }
}

impl From<&Direction> for bevy::prelude::Vec3 {
    fn from(direction: &Direction) -> Vec3 {
        match direction {
            Direction::North => Vec3::Y,
            Direction::East =>  Vec3::X,
            Direction::South => -Vec3::Y,
            Direction::West =>  -Vec3::X,
        }
    }
}

impl From<Direction> for bevy::prelude::Vec3 {
    fn from(direction: Direction) -> Vec3 {
        match direction {
            Direction::North => Vec3::Y,
            Direction::East =>  Vec3::X,
            Direction::South => -Vec3::Y,
            Direction::West =>  -Vec3::X,
        }
    }
}

impl From<&Direction> for bevy::prelude::Vec2 {
    fn from(direction: &Direction) -> Vec2 {
        Vec3::from(direction).xy()
    }
}

/*impl Into<bevy::prelude::Vec3> for Direction {
    fn into(self) -> Vec3 {
        let v3 : Vec3 = self.into();
        v3.xy()
    }
}*/