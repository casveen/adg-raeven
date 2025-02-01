use crate::player::player_controller::{
    fsm::{ContextAggregate, TState},
    PlayerEvent,
    PlayerMovementEvent,
};
use bevy::prelude::*;

const PLAYER_RUN_SPEED: f32 = 10.0;

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

        let movement = motion * PLAYER_RUN_SPEED * time.delta_secs();
        transform.translation += movement;
        let direction = motion.normalize(); // gamepad motion can be less than 1.
        let forward = -Vec3::Y.cross(direction);
        let right = direction;
        let matrix = Mat3::from_cols(right, Vec3::Y, forward);
        let quat = Quat::from_mat3(&matrix);
        transform.rotation = quat;
    }
}
