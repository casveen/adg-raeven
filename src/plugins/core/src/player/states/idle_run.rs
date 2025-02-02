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
    fsm: Single<Entity, With<PlayerFsm>>,
    current_state: Single<&Children, With<PlayerFsm>>,
    mut commands: Commands,
    mut transform: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
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
    let Some(motion) = event.motion else {
        return;
    };

    let movement = motion * RUN_SPEED * time.delta_secs();
    transform.translation += movement;

    movement::rotate_player(motion, transform, ROTATION_SPEED, time);
}
