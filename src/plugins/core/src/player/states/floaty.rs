use crate::{
    new_state,
    player::controller::{PlayerEvent, PlayerFsm},
};
use bevy::prelude::*;

pub fn process_event(
    event: Trigger<PlayerEvent>,
    fsm: Query<Entity, With<PlayerFsm>>,
    current_state: Query<&Children, With<PlayerFsm>>,
    mut commands: Commands,
) {
    // if fsm.is_empty() || current_state.is_empty() || transform.is_empty() {
    if fsm.is_empty() || current_state.is_empty() {
        // debug!("corcycept process_event. fsm || current_state || transform .. is_empty");
        debug!("corcycept process_event. fsm || current_state .. is_empty");
        return;
    }
    let fsm = fsm.single();
    let current_state = current_state.single();
    // let mut transform = transform.single_mut();

    match event.event() {
        PlayerEvent::Floaty(event) => {
            if !event.active {
                new_state!(commands, fsm, current_state, super::idle_run::process_event);
            }
        }
        _ => (),
    }
}
