
use bevy::prelude::*;
use bevy::render::mesh::morph::MeshMorphWeights;
use blenvy::*;

/**
 * Components and systems for attaching a mushroom to a entity, which can grow/shrink on some command
 * These are NOT cordyceps, or puffy, mushrooms. But intended as environment mechanics
 * 
 * components:
 *     Bloomable: this entity can spawn a mushroom(as specified by MushroomToBloom).
 *          In blender: place this on the entity that SPAWNS the mushroom
 *     Blooms: this entity IS a mushroom. This component handles the blendshapes on the mushroom mesh.
 *          In blender: place this on the mesh object of the entity that is spawned (the one that will get a MeshMorphWeight in bevy).
 * How to use:
 *     
 * 
 * TODO:
 * - currently, only amanita is ever spawned. others should be possible in the future
 */

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Bloomable{
    to_bloom: MushroomToBloom,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Blooms{
    bloomed: bool,
    bloom_value: f32, // [0,1]. Current state in transition, 1 being bloomed, and 0 not.
    bloom_tween: f32, // speed of transition
}

//Terrible helper function for taking a bloom_value ([0,1]) and returning the transitions when using three blendshapes
fn interpolate_three_blendshapes(t:f32) -> (f32, f32, f32) {
    return (
        (1.-2.*t),
        (1.-(2.*t-1.).abs()),
        (2.*t-1.)
    );
}

// all bloomables shall have a small, basis and big blendshape!
#[derive(Reflect, Debug)]
enum MushroomToBloom {
    Amanita,
    Inkcap
}

impl Default for MushroomToBloom {
    fn default() -> Self { MushroomToBloom::Amanita }
}

/**
 * Add a mushroom entity to each entity having a bloomable
 */
fn add_mushrooms_to_bloomables(
    query_bloom: Query<(Entity, &Bloomable), Added<Bloomable>>,
    mut commands: Commands,
) {
    for (entity, _) in query_bloom.iter() {
        let mushroom = commands.spawn((
            BlueprintInfo::from_path("blueprints/amanita.glb"), // TODO: dont use hardcoded path
            SpawnBlueprint,
            HideUntilReady,
        )).id();
        commands.entity(entity).add_child(mushroom);
    }
}

fn update_mushroom_size(
    mut query_mushroom: Query<(&mut Blooms, &mut MeshMorphWeights)>,
) {
    for (mut mush, mut weights ) 
        in query_mushroom.iter_mut() {
        //let Mushroom{mut bloomed : b, ref mut bloom_value : v, mut bloom_tween : a, mut blendshape_index_small : b, mut blendshape_index_basis: c, mut blendshape_index_large: d } 
        //= mush;
        // TODO: how the fcuk are you supposed to mutably unstructure a struct??? 

        let sign = if mush.bloomed {1.} else {-1.}; // TODO: there has to be a sexier way to do this...
        mush.bloom_value=(mush.bloom_value+sign*mush.bloom_tween).clamp(0.,1.);

        let curve = EasingCurve::new(0.,1.,
            if mush.bloomed {EaseFunction::ElasticOut} else {EaseFunction::ElasticIn});

        let (t0, t1, t2) = interpolate_three_blendshapes(curve.sample(mush.bloom_value).expect("bloom_value outside unit interval!"));

        weights.weights_mut()[1]=t0; //
        weights.weights_mut()[0]=t1; // TODO: this is an extremely fragile hack to circumvent that I couldnt get the
        weights.weights_mut()[2]=t2; // blendshapes by name.
    }
}

pub struct MushroomPlugin;
impl Plugin for MushroomPlugin {
    
    fn build(&self, app: &mut App) {
        app.register_type::<Bloomable>()
        .register_type::<Blooms>()
        .add_systems(Update, (add_mushrooms_to_bloomables, update_mushroom_size));
    }
}
