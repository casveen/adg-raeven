use avian3d::math::PI;
use bevy::{log::info, math::{IVec3, Vec2, Vec3, Vec3Swizzles}, reflect::Reflect};

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


const MPI3OVER4: f32 = -PI*3.0/4.0;
const MPIHALF: f32 = -PI*2.0;
const MPI1OVER4: f32 = -PI*1.0/4.0;
const PI1OVER4: f32 = PI*1.0/4.0;
const PIHALF: f32 = PI*2.0;
const PI3OVER4: f32 = PI*3.0/4.0;

 
impl From<bevy::prelude::Vec2> for Direction {
    fn from(vector: Vec2) -> Direction {
        info!("angle {:?}", vector.to_angle());
        match vector.to_angle() {
            MPI1OVER4..PI1OVER4  => Direction::East,
            PI1OVER4..PI3OVER4   => Direction::North,
            MPI3OVER4..MPI1OVER4 => Direction::South,
            _                    => Direction::West,
        }
    }
}

impl From<bevy::prelude::Vec3> for Direction {
    fn from(vector: Vec3) -> Direction {
        Direction::from(vector.xz())
    }
}

impl From<bevy::prelude::IVec3> for Direction {
    fn from(vector: IVec3) -> Direction {
        Direction::from(vector.as_vec3().xz())
    }
}

impl From<&Direction> for bevy::prelude::Vec3 {
    fn from(direction: &Direction) -> Vec3 {
        match direction {
            Direction::North => Vec3::Z,
            Direction::East =>  Vec3::X,
            Direction::South => -Vec3::Z,
            Direction::West =>  -Vec3::X,
        }
    }
}

impl From<Direction> for bevy::prelude::Vec3 {
    fn from(direction: Direction) -> Vec3 {
        match direction {
            Direction::North => Vec3::Z,
            Direction::East =>  Vec3::X,
            Direction::South => -Vec3::Z,
            Direction::West =>  -Vec3::X,
        }
    }
}

impl From<&Direction> for bevy::prelude::Vec2 {
    fn from(direction: &Direction) -> Vec2 {
        Vec3::from(direction).xz()
    }
}

/*impl Into<bevy::prelude::Vec3> for Direction {
    fn into(self) -> Vec3 {
        let v3 : Vec3 = self.into();
        v3.xy()
    }
}*/