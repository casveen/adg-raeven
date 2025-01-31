use crate::input::input_manager::{button, Action, InputManager};
use bevy::prelude::*;
use bevy_framepace::*;

const ACTION_INCREASE_FRAMERATE: Action = Action("increase_framerate");
const ACTION_DECREASE_FRAMERATE: Action = Action("decrease_framerate");

pub fn startup(mut settings: ResMut<FramepaceSettings>) {
    settings.limiter = Limiter::Auto;
}

pub fn register_input(mut input: ResMut<InputManager>) {
    input.register_action_button(
        ACTION_INCREASE_FRAMERATE,
        vec![button::Variant::Keyboard(KeyCode::Equal)],
    );
    input.register_action_button(
        ACTION_DECREASE_FRAMERATE,
        vec![button::Variant::Keyboard(KeyCode::Minus)],
    );
}

pub fn read_input(
    input: Res<InputManager>,
    mut settings: ResMut<FramepaceSettings>,
    mut state: Local<State>,
) {
    if input.is_action_just_pressed(ACTION_INCREASE_FRAMERATE) {
        settings.limiter = state.increase_framerate();
    } else if input.is_action_just_pressed(ACTION_DECREASE_FRAMERATE) {
        settings.limiter = state.decrease_framerate();
    }
}

#[derive(Default)]
pub struct State(u8);
impl State {
    const MAX: u8 = 5;
    const MIN: u8 = 0;

    fn increase_framerate(&mut self) -> Limiter {
        match self.0 {
            Self::MIN => (),
            _ => self.0 -= 1,
        };
        self.get_framerate()
    }

    fn decrease_framerate(&mut self) -> Limiter {
        match self.0 {
            Self::MAX => (),
            _ => self.0 += 1,
        };
        self.get_framerate()
    }

    fn get_framerate(&self) -> Limiter {
        match self.0 {
            5 => Limiter::from_framerate(15.),
            4 => Limiter::from_framerate(30.),
            3 => Limiter::from_framerate(60.),
            2 => Limiter::from_framerate(90.),
            1 => Limiter::from_framerate(120.),
            _ => Limiter::Auto,
        }
    }
}
