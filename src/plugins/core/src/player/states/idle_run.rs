use crate::{
    new_state,
    player::{
        controller::{Player, PlayerEvent, PlayerFsm, PlayerMovementEvent},
        states::utils::movement,
    },
};
use bevy::prelude::*;

const RUN_SPEED: f32 = 10.0;
const ROTATION_SPEED: f32 = 22.0;

pub fn process_event(
    event: Trigger<PlayerEvent>,
    fsm: Query<Entity, With<PlayerFsm>>,
    current_state: Query<&Children, With<PlayerFsm>>,
    mut transform: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if fsm.is_empty() || current_state.is_empty() || transform.is_empty() {
        debug!("idle_run process_event. fsm || current_state || transform .. is_empty");
        return;
    }
    let fsm = fsm.single();
    let current_state = current_state.single();
    let mut transform = transform.single_mut();

    match event.event() {
        PlayerEvent::Movement(event) => idle_run(&event, &mut *transform, &time),
        PlayerEvent::Floaty(event) => {
            if event.active {
                new_state!(commands, fsm, current_state, super::floaty::process_event);
            }
        }
        PlayerEvent::CordyCept(event) => {
            if event.active {
                new_state!(
                    commands,
                    fsm,
                    current_state,
                    super::cordycept::process_event
                );
            }
        }
    }
}

fn idle_run(event: &PlayerMovementEvent, transform: &mut Transform, time: &Time) {
    let Some(motion) = &event.motion else {
        return;
    };
    let motion_vector: bevy::prelude::Vec3 = Vec3::from(motion);
    let movement = motion_vector * RUN_SPEED * time.delta_secs();
    transform.translation += movement;

    movement::rotate_player(motion_vector, transform, ROTATION_SPEED, time);
}
