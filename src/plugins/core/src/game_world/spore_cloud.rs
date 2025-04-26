use avian3d::prelude::*;
use bevy::{
    color::palettes::css::CORNSILK,
    prelude::*,
    render::mesh::{SphereKind, SphereMeshBuilder},
};
use bevy_hanabi::prelude::*;

/**
 * OPTIMIZATIONS:
 * - use a single particle systems for all mushrooms, instead of creating it exactly when needed.
 * 
 */

pub(super) struct SporeCloudPlugin;
impl Plugin for SporeCloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(Update, tick_spore_cloud)
            .add_observer(spawn_spore_cloud);
    }
}

const LIFETIME: f32 = 5.0;
const SIZE: f32 = 2.0;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct SporeCloud(Timer);
impl Default for SporeCloud {
    fn default() -> Self {
        Self(Timer::from_seconds(LIFETIME, TimerMode::Once))
    }
}

fn tick_spore_cloud(
    mut query: Query<(Entity, &mut SporeCloud)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut spore_cloud) in query.iter_mut() {
        spore_cloud.0.tick(time.delta());
        if spore_cloud.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Event)]
pub struct SpawnSporeCloud(pub Transform);

fn spawn_spore_cloud(
    trigger: Trigger<SpawnSporeCloud>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut effects: ResMut<Assets<EffectAsset>>, // particle systems go here
) {
    // Create the mesh.
    let mesh = meshes.add(SphereMeshBuilder::new(0.5, SphereKind::Ico { subdivisions: 2 }).build());
    let effect = create_effect(mesh, &mut effects);
    // Spawn the effect.
    commands.spawn((
        SporeCloud::default(),
        Name::new("puff"),
        trigger.event().0,
        RigidBody::Dynamic,
        Collider::cuboid(SIZE, SIZE, SIZE), // TODO: teeeeechnically we wont need this, if we use a grid this is just visual
                                            // ParticleEffectBundle {
                                            //     effect: ParticleEffect::new(effect),
                                            //     ..default()
                                            // },
    ));
}

/*******************
 * PARTICLES STUFF *
 ******************/
// Builds the spore puff as a particle system
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
        EffectAsset::new(128, Spawner::once(64.0.into(), true), module)
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