
use bevy::prelude::*;
use blenvy::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct Bloomable {
    bloomed: bool,
    bloom_tween: f32, //[0,1]
    to_bloom: String,
}


fn bloom(
    // `Entity` can be used to get the ID of things that match the query
    query_bloom: Query<(Entity, &Bloomable)>,
    // we also need Commands, so we can despawn entities if we have to
    mut commands: Commands,
) {
    //query_bloom.single_mut()
    commands.spawn((
        BlueprintInfo::from_path("spawnable.glb"),
        SpawnBlueprint,
        Transform::from_xyz(0.0, 0.0, 0.0), // VERY important !!
    ));
}

/*pub struct Mushroom;

impl Bloomable for Mushroom {

}*/


pub struct MushroomPlugin;
impl Plugin for MushroomPlugin {
    
    fn build(&self, app: &mut App) {
        app.register_type::<Bloomable>();
    }
}
