use bevy::{
    prelude::*,
    app::{App, Plugin}, 
    ecs::{component::Component, entity::Entity, system::Resource}, 
    math::IVec3, reflect::Reflect, 
    utils::{HashMap, HashSet}
};

#[derive(Resource)]
pub struct WorldGrid (
    pub HashMap<Coordinate, HashSet<Entity>>,
    pub f32
);

impl Default for WorldGrid {
    fn default() -> Self {
        WorldGrid(default(), 3.0)
    }
}

#[derive(Component, Reflect, Debug, Eq, Hash, PartialEq, Clone)]
#[reflect(Component)]
pub struct Coordinate(pub IVec3);

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

pub struct WorldGridPlugin;
impl Plugin for WorldGridPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Coordinate>()
        .init_resource::<WorldGrid>()
        .add_systems(Update, init_coordinates); 
    }
}