use avian3d::prelude::*;
use bevy::{math::ops::cos, math::ops::sin, prelude::*};

#[derive(Component)]
pub struct Wall;

/**
 * tmp spawning method
 */
pub(super) fn setup_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut t = Transform::from_xyz(3.0, 1.0, 4.0);
    t.scale = Vec3::new(2.0, 1.0, 2.0);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(t.scale))),
        MeshMaterial3d(materials.add(Color::srgb_u8(190, 255, 220))),
        t,
        Collider::cuboid(t.scale.x, t.scale.y, t.scale.z),
        RigidBody::Static,
        Wall,
    ));

    // test
    let mut t = Transform::from_xyz(1.0, 4.0, 2.0);
    let an = 60.0_f32.to_radians();
    let a = sin(an / 2.);
    let d = Vec3::new(1., 2., 1.).normalize();
    t.rotation = Quat::from_xyzw(a * d.x, a * d.y, a * d.z, cos(an / 2.));
    t.scale = Vec3::new(1.0, 1.0, 1.0);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(t.scale))),
        MeshMaterial3d(materials.add(Color::srgb_u8(190, 255, 220))),
        t,
        Collider::cuboid(t.scale.x, t.scale.y, t.scale.z),
        RigidBody::Dynamic,
        CollidingEntities::default(),
    ));
}
