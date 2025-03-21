use bevy::{
    prelude::*,
    app::{App, Plugin}, 
    ecs::component::Component, 
    reflect::Reflect
};

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Obstacle;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Walkable;

pub struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Obstacle>()
        .register_type::<Walkable>();
    }
}