use bevy::{
    color::palettes::css::CORNSILK,
    prelude::*,
    render::mesh::{SphereKind, SphereMeshBuilder},
};
use bevy_hanabi::prelude::*;


#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Puffable;

#[derive(Event)]
struct Puff;



fn puff(
    trigger: Trigger<Puff>,
    puffables: Query<(&Puffable, &Children)>,
    mut effect: Query<&mut EffectInitializers>,
) {
    if let Ok((_, children)) = puffables.get(trigger.entity()) {
        for &child in children.iter() {
            if let Ok(mut chi) = effect.get_mut(child) {
                chi.reset();
            } 
        }
    }
}

// puff the puffable entities
fn set_puff_assets_to_puffables(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity, &mut Puffable), Added<Puffable>>,
) {
    // Create the effect asset.
    for (entity, _) in query.iter_mut() {
        // Create the mesh.
        let mesh = meshes.add(SphereMeshBuilder::new(0.5, SphereKind::Ico { subdivisions: 2 }).build());
        let effect = create_effect(mesh, &mut effects);

        // Spawn the effect.
        let particle_entity =commands.spawn((
            Name::new("puff"),
            ParticleEffectBundle {
                effect: ParticleEffect::new(effect),
                ..default()
            },
        )).id();

        commands.entity(entity)
        .add_child(particle_entity)
        .observe(puff);
    }
}




// Builds the smoke puffs.
fn create_effect(mesh: Handle<Mesh>, effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let writer = ExprWriter::new();

    // Position the particle laterally within a small radius.
    let init_xz_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
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
        (writer.rand(ScalarType::Float) * writer.lit(0.75) + writer.lit(0.25)).expr(),
    );

    // Make the particles grow over time.
    let update_size = SetAttributeModifier::new(
        Attribute::SIZE,
        writer
            .attr(Attribute::F32_0)
            .mul(
                (writer
                    .lit(1.0)
                    .add((writer.attr(Attribute::AGE)).mul(writer.lit(10)))
            ).min(writer.lit(1.0)))
            .expr(),
    );

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
                CORNSILK.with_alpha(1.0).to_vec4(),
                CORNSILK.with_alpha(0.0).to_vec4()
            )
    };

    let module = writer.finish();

    // Add the effect.
    effects.add(
        EffectAsset::new(512, Spawner::once(64.0.into(), false), module)
            .with_name("Puff_effect")
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
            .mesh(mesh),
    )
}




pub struct PuffPlugin;
impl Plugin for PuffPlugin {
    
    fn build(&self, app: &mut App) {
        app
        .register_type::<Puffable>()
        .add_systems(Update, set_puff_assets_to_puffables)
        .add_plugins(HanabiPlugin);
    }
}
