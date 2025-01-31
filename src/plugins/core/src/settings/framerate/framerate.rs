use crate::{
    input::input_manager::{button, Action, InputManager},
    settings::title::Title,
};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_framepace::*;

pub struct FrameratePlugin;
impl Plugin for FrameratePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FramepacePlugin, FrameTimeDiagnosticsPlugin::default()))
            .insert_resource(ShowFps::default())
            .add_systems(Startup, (register_input, startup))
            .add_systems(Update, (read_input, show_fps));
    }
}

fn show_fps(
    mut windows: Query<&mut Window>,
    diagnostics: Res<DiagnosticsStore>,
    show_fps: Res<ShowFps>,
    mut cached_show_fps: Local<ShowFps>,
    title: Res<Title>,
) {
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    let smoothed = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .map(|x| x.smoothed().unwrap_or_default())
        .unwrap_or_default();

    if show_fps.0 {
        window.title = format!("FPS {smoothed:.0}");
    }

    if *show_fps != *cached_show_fps {
        cached_show_fps.0 = show_fps.0;
        window.title = title.0.to_string();
    }
}

const ACTION_INCREASE_FRAMERATE: Action = Action("increase_framerate");
const ACTION_DECREASE_FRAMERATE: Action = Action("decrease_framerate");
const ACTION_SHOW_FPS: Action = Action("show_fps");

fn startup(mut settings: ResMut<FramepaceSettings>) {
    settings.limiter = Limiter::Auto;
}

fn register_input(mut input: ResMut<InputManager>) {
    input.register_action_button(
        ACTION_INCREASE_FRAMERATE,
        vec![button::Variant::Keyboard(KeyCode::Equal)],
    );
    input.register_action_button(
        ACTION_DECREASE_FRAMERATE,
        vec![button::Variant::Keyboard(KeyCode::Minus)],
    );
    input.register_action_button(
        ACTION_SHOW_FPS,
        vec![button::Variant::Keyboard(KeyCode::Backspace)],
    );
}

fn read_input(
    input: Res<InputManager>,
    mut settings: ResMut<FramepaceSettings>,
    mut state: Local<State>,
    mut show_fps: ResMut<ShowFps>,
) {
    if input.is_action_just_pressed(ACTION_INCREASE_FRAMERATE) {
        settings.limiter = state.increase_framerate();
    } else if input.is_action_just_pressed(ACTION_DECREASE_FRAMERATE) {
        settings.limiter = state.decrease_framerate();
    }

    if input.is_action_just_pressed(ACTION_SHOW_FPS) {
        show_fps.0 = !show_fps.0;
    }
}

#[derive(Resource, Default, PartialEq, Eq)]
struct ShowFps(bool);

#[derive(Default)]
struct State(u8);
impl State {
    const MAX: u8 = 7;
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
            7 => Limiter::from_framerate(15.),
            6 => Limiter::from_framerate(20.),
            5 => Limiter::from_framerate(30.),
            4 => Limiter::from_framerate(45.),
            3 => Limiter::from_framerate(60.),
            2 => Limiter::from_framerate(90.),
            1 => Limiter::from_framerate(120.),
            _ => Limiter::Auto,
        }
    }
}
