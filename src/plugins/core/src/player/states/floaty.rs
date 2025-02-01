use crate::player::player_controller::{
    fsm::{ContextAggregate, TState},
    PlayerEvent,
};
use bevy::prelude::*;

use super::idle_run::IdleRunState;

pub struct FloatyState;
impl TState for FloatyState {
    fn get_name(&self) -> &'static str {
        "FloatyState"
    }

    fn enter_state(&self, event: &PlayerEvent, aggregate: &mut ContextAggregate) {}

    fn process_event(
        &self,
        event: &PlayerEvent,
        aggregate: &mut ContextAggregate,
    ) -> Option<Box<dyn TState>> {
        match event {
            PlayerEvent::Floaty(floaty) => {
                if !floaty.active {
                    return Some(Box::new(IdleRunState));
                }
            }
            _ => (),
        };
        None
    }
}
