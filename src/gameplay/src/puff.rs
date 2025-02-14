
use std::{error::Error, f32::consts::FRAC_PI_2};

use bevy::{
    color::palettes::css::FOREST_GREEN,
    core_pipeline::tonemapping::Tonemapping,
    math::vec3,
    prelude::*,
    render::mesh::{SphereKind, SphereMeshBuilder},
};
use bevy_hanabi::prelude::*;


#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Puffable{
}


// Performs initialization of the scene.
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Create the mesh.
    let mesh = meshes.add(SphereMeshBuilder::new(0.5, SphereKind::Ico { subdivisions: 4 }).build());

    // Create the effect asset.
    let effect = create_effect(mesh, &mut effects);

    // Spawn the effect.
    commands.spawn((
        Name::new("cartoon explosion"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect),
            ..default()
        },
    ));
}






// Builds the smoke puffs.
fn create_effect(mesh: Handle<Mesh>, effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let writer = ExprWriter::new();

    // Position the particle laterally within a small radius.
    let init_xz_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::Y).mul(writer.lit(2.0)).expr(),
        axis: writer.lit(Vec3::Y).expr(),
        radius: writer.lit(0.2).expr(),
        dimension: ShapeDimension::Volume,
    };

    // Position the particle vertically. Jiggle it a little bit for variety's
    // sake.
    let init_y_pos = SetAttributeModifier::new(
        Attribute::POSITION,
        writer
            .attr(Attribute::POSITION)
            .add(writer.lit(Vec3::Y)*writer.rand(ScalarType::Float).mul(writer.lit(1.2)))
            .expr(),
    );

    // Set up the age and lifetime.
    let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.0).expr());
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(3.0).expr());

    // Vary the size a bit.
    let init_size = SetAttributeModifier::new(
        Attribute::F32_0,
        (writer.rand(ScalarType::Float) * writer.lit(0.25) + writer.lit(0.25)).expr(),
    );

    // Make the particles grow over time.
    let update_size = SetAttributeModifier::new(
        Attribute::SIZE,
        writer
            .attr(Attribute::F32_0)
            .mul(
                writer
                    .lit(1.0)
                    .add((writer.attr(Attribute::AGE)).mul(writer.lit(10)))
                    ,
            ).min(writer.lit(1.0))
            .expr(),
    );

    // Make the particles move backwards at a constant speed.
    /*let init_velocity = SetAttributeModifier::new(
        Attribute::VELOCITY,
    
        writer.lit(vec3(0.0, 0.0, -20.0)).expr(),
    );*/

    /*let velocity = SetAttributeModifier::new(
        Attribute::VELOCITY,

        (writer.attr(Attribute::VELOCITY)*(writer.lit(1.0)-writer.attr(Attribute::AGE))
            //writer.attr(Attribute::VELOCITY).normalized()*(writer.attr(Attribute::F32_0).step(writer.lit(1.5)))   
            

        
        )
        .expr()
        //(writer.rand(ScalarType::Float)*writer.lit(4.0)+writer.lit(1)).expr(),
    );*/

    /*let update_speed = SetAttributeModifier::new(
        Attribute::F32_0,
        (writer.attr(Attribute::F32_0)+writer.lit(0.01)).expr()
        //(writer.rand(ScalarType::Float)*writer.lit(4.0)+writer.lit(1)).expr(),
    );*/

    /*let phase_1 = SetAttributeModifier::new(
        Attribute::F32_0,
        (writer.attr(Attribute::AGE)).step(writer.lit(1.5)).expr()
        //(writer.rand(ScalarType::Float)*writer.lit(4.0)+writer.lit(1)).expr(),
    );
    let phase_1_val = SetAttributeModifier::new(
        Attribute::F32_1,
        (writer.attr(Attribute::F32_0)).mul(writer.attr(Attribute::AGE)).expr()
        //(writer.rand(ScalarType::Float)*writer.lit(4.0)+writer.lit(1)).expr(),
    );
    let phase_2 = SetAttributeModifier::new(
        Attribute::F32_2,
        (writer.lit(1)-writer.attr(Attribute::F32_0)).expr()
        //(writer.rand(ScalarType::Float)*writer.lit(4.0)+writer.lit(1)).expr(),
    );
    let phase_2_val = SetAttributeModifier::new(
        Attribute::F32_3,
        (writer.attr(Attribute::F32_2)).mul(writer.lit(1.5)-writer.attr(Attribute::AGE)/writer.lit(1.5)).expr()
        //(writer.rand(ScalarType::Float)*writer.lit(4.0)+writer.lit(1)).expr(),
    );*/


    let phase1 = 1.0;
    let phase05 = 0.2;

    let init_velocity = SetAttributeModifier::new(
        Attribute::F32X3_0,
        (
            ( // speed
                writer.rand(ScalarType::Float)*writer.lit(10.0)+writer.lit(5)
            )
            .mul( // random direction in XZ plane, normalized
                (
                    writer.lit(Vec3::X)*(writer.lit(2)*writer.rand(ScalarType::Float)+writer.lit(-1))+
                    writer.lit(Vec3::Z)*(writer.lit(2)*writer.rand(ScalarType::Float)+writer.lit(-1))
                ).normalized()
            )
        )
        .expr()
    );

    /*fn pocket(t: WriterExpr, from: WriterExpr, to: WriterExpr) -> WriterExpr {
        let u = t.clone();
        let v = t.clone();

        (writer.lit(1.0)-u.step(from))-(writer.lit(1.0)-v.step(to))
    }*/

    let velocity = SetAttributeModifier::new(
        Attribute::VELOCITY,
        (
            writer.attr(Attribute::F32X3_0) //direction AND speed, as set in init_velocity
            .mul(
                writer.lit(1.0)/((writer.lit(10)*writer.attr(Attribute::AGE)).exp()) //constant speed in phase 1
            )
            .mul(writer.lit(1.0)-writer.attr(Attribute::AGE).step(writer.lit(phase1)))
            
            +
            (writer.attr(Attribute::F32X3_0)
            .dot(writer.lit(Vec3::Y))) //in phase 2, move upwards
            .mul(
                
                writer.lit(1.0) //constant speed in phase 1
            )
            .mul(writer.attr(Attribute::AGE).step(writer.lit(phase05)))
        ).expr()
    );

    let acceleration =  SetAttributeModifier::new(
        Attribute::F32X3_0,
        (
            writer.attr(Attribute::F32X3_0)
            +
            writer.lit(Vec3::Y) //in phase 2, ACCELERATE upwards
            .mul(
                writer.lit(0.005) //constant speed in phase 1
            )
            .mul(writer.attr(Attribute::AGE).step(writer.lit(phase05)))
        ).expr()
    );

    let update_alpha =  ColorOverLifetimeModifier {
        gradient: 
            Gradient::linear(
                Vec4::new(1.0,1.0,1.0,1.0), 
                Vec4::new(1.0,1.0,1.0,0.0), 
            )
    };
        /*Attribute::ALPHA,
        (writer.lit(1)).expr()*/
    



    let module = writer.finish();

    // Add the effect.
    effects.add(
        EffectAsset::new(64, Spawner::burst(32.0.into(), 3.0.into()), module)
            .with_name("cartoon explosion")
            .init(init_xz_pos)
            .init(init_y_pos)
            .init(init_age)
            .init(init_lifetime)
            .init(init_velocity)
            .init(init_size)
            .update(velocity)
            .update(update_size)
            .update(acceleration)
            .render(update_alpha)
            //.update(velocity)
            //.update(init_accel)
            //.update(update_size)
            //.render(render_lambertian)
            .mesh(mesh),
    )
}




pub struct PuffPlugin;
impl Plugin for PuffPlugin {
    
    fn build(&self, app: &mut App) {

        app.register_type::<Puffable>()
        //.add_systems(Update, (add_mushrooms_to_bloomables, update_mushroom_size));
        .add_systems(Startup, setup)
        .add_plugins(HanabiPlugin);

        //.add_systems(Update, setup_scene_once_loaded)
    }
}
