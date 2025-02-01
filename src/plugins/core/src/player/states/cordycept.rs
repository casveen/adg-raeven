use crate::{
    new_state,
    player::player_controller::{PlayerEvent, PlayerFsm},
};
use bevy::prelude::*;

pub fn process_event(
    event: Trigger<PlayerEvent>,
    fsm: Single<Entity, With<PlayerFsm>>,
    current_state: Single<&Children, With<PlayerFsm>>,
    mut commands: Commands,
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
                commands.trigger(CordyCeptMovement(motion));
            }
        }
        _ => (),
    }
}

#[derive(Component)]
pub struct CordyCeptedComponent;
#[derive(Event)]
pub struct CordyCeptMovement(pub Vec3);
