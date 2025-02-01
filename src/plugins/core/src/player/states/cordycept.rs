use crate::player::player_controller::{
    fsm::{ContextAggregate, TState},
    PlayerEvent,
};
use bevy::prelude::*;

use super::idle_run::IdleRunState;

pub struct CordyCeptState;
impl TState for CordyCeptState {
    fn get_name(&self) -> &'static str {
        "CordyCeptState"
    }

    fn enter_state(&self, event: &PlayerEvent, aggregate: &mut ContextAggregate) {}

    fn process_event(
        &self,
        event: &PlayerEvent,
        aggregate: &mut ContextAggregate,
    ) -> Option<Box<dyn TState>> {
        match event {
            PlayerEvent::CordyCept(cordycept) => {
                if !cordycept.active {
                    return Some(Box::new(IdleRunState));
                }
            }
            _ => (),
        };
        None
    }
}
