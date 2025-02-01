use crate::{
    new_state,
    player::player_controller::{Player, PlayerEvent, PlayerFsm},
};
use bevy::prelude::*;

use super::utils::movement;

const RUN_SPEED: f32 = 7.0;
const ROTATION_SPEED: f32 = 15.0;

pub fn process_event(
    event: Trigger<PlayerEvent>,
    mut commands: Commands,
    fsm: Single<Entity, With<PlayerFsm>>,
    current_state: Single<&Children, With<PlayerFsm>>,
    mut transform: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    match event.event() {
        PlayerEvent::CordyCept(event) => {
            if !event.active {
                new_state!(commands, fsm, current_state, super::idle_run::process_event);
            }
        }
        PlayerEvent::Movement(event) => {
            // get entities?
            // ability might just be global, as long as someone has the status
            if let Some(motion) = event.motion {
                let movement = motion * RUN_SPEED * time.delta_secs();
                transform.translation += movement;
                movement::rotate_player(motion, &mut *transform, ROTATION_SPEED, &time);

                commands.trigger(CordyCeptMovement(movement));
            }
        }
        _ => (),
    }
}

#[derive(Component)]
pub struct CordyCeptedComponent;
#[derive(Event)]
pub struct CordyCeptMovement(pub Vec3);
