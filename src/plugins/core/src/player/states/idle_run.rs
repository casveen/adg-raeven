use crate::player::player_controller::{
    fsm::{ContextAggregate, TState},
    PlayerEvent, PlayerMovementEvent,
};
use bevy::prelude::*;

const RUN_SPEED: f32 = 10.0;
const ROTATION_SPEED: f32 = 22.0;

pub struct IdleRunState;
impl TState for IdleRunState {
    fn enter_state(&self, event: &PlayerEvent, aggregate: &mut ContextAggregate) {}

    fn process_event(
        &self,
        event: &PlayerEvent,
        aggregate: &mut ContextAggregate,
    ) -> Option<Box<dyn TState>> {
        match event {
            PlayerEvent::Movement(event) => self.idle_run(event, aggregate),
            _ => (),
        };
        None
    }
}

impl IdleRunState {
    fn idle_run(&self, event: &PlayerMovementEvent, aggregate: &mut ContextAggregate) {
        let Some(motion) = event.motion else {
            return;
        };

        let ContextAggregate(transform, time) = &mut *aggregate;

        let movement = motion * RUN_SPEED * time.delta_secs();
        transform.translation += movement;

        let right = motion.normalize(); // gamepad motion can be less than 1.
        let forward = -Vec3::Y.cross(right);
        let matrix = Mat3::from_cols(right, Vec3::Y, forward);
        let quat = Quat::from_mat3(&matrix);
        transform.rotation = transform
            .rotation
            .slerp(quat, time.delta_secs() * ROTATION_SPEED);
    }
}
