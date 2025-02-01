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
        PlayerEvent::Floaty(event) => {
            if !event.active {
                new_state!(commands, fsm, current_state, super::idle_run::process_event);
            }
        }
        _ => (),
    }
}
