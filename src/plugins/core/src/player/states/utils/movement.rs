use bevy::prelude::*;


pub fn rotate_player(
    motion: Vec3,
    transform: &mut Transform,
    rotation_speed: f32,
    time: &Time
) {
    let right = motion.normalize(); // gamepad motion can be less than 1.
    let forward = -Vec3::Y.cross(right);
    let matrix = Mat3::from_cols(right, Vec3::Y, forward);
    let quat = Quat::from_mat3(&matrix);
    transform.rotation = transform
        .rotation
        .slerp(quat, time.delta_secs() * rotation_speed);
}
