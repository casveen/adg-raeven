use crate::{
    new_state,
    player::controller::{Player, PlayerEvent, PlayerFsm},
};
use bevy::prelude::*;

use super::utils::movement;

const RUN_SPEED: f32 = 7.0;
const ROTATION_SPEED: f32 = 15.0;

pub fn process_event(
    event: Trigger<PlayerEvent>,
    fsm: Query<Entity, With<PlayerFsm>>,
    current_state: Query<&Children, With<PlayerFsm>>,
    mut transform: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if fsm.is_empty() || current_state.is_empty() || transform.is_empty() {
        debug!("corcycept process_event. fsm || current_state || transform .. is_empty");
        return;
    }
    let fsm = fsm.single();
    let current_state = current_state.single();
    let mut transform = transform.single_mut();

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

                commands.trigger(CordyCeptMovement(movement, transform.rotation));
            }
        }
        _ => (),
    }
}

#[derive(Component)]
pub struct CordyCeptedComponent;
#[derive(Event)]
pub struct CordyCeptMovement(pub Vec3, pub Quat);
