use bevy::prelude::*;

use crate::{
    camera::isometric_camera::CameraYaw,
    input::input_manager::{self, button, motion, InputManager},
};

use super::player_visuals::RenderPlugin;

pub struct PlayerControllerPlugin;
impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderPlugin)
            .add_systems(
                Startup,
                (
                    register_input,
                    spawn_player,
                    //
                ),
            )
            .add_systems(Update, process_input)
            .add_observer(observe_event);
    }
}

#[derive(Component)]
#[require(Transform(|| Transform::from_xyz(0., 0., 0.)))]
pub struct Player;

#[derive(Event)]
pub struct PlayerMovementEvent {
    pub motion: Option<Vec3>,
}
impl PlayerMovementEvent {
    pub fn empty() -> Self {
        Self { motion: None }
    }
}

#[derive(Event)]
pub struct PlayerFloatyEvent {
    pub active: bool,
}

#[derive(Event)]
pub struct PlayerCordyCeptEvent {
    pub active: bool,
}

#[derive(Event)]
pub enum PlayerEvent {
    Movement(PlayerMovementEvent),
    Floaty(PlayerFloatyEvent),
    CordyCept(PlayerCordyCeptEvent),
}

#[derive(Resource, Default)]
pub struct PlayerSpawn {
    pub transform: Transform,
}

static MOVEMENT: input_manager::Action = input_manager::Action("movement");
static ABILITY_FLOATY: input_manager::Action = input_manager::Action("ability_floaty");
static ABILITY_CORDYCEPT: input_manager::Action = input_manager::Action("ability_cordycept");

fn register_input(mut im: ResMut<input_manager::InputManager>) {
    im.register_action_motion(
        MOVEMENT,
        vec![
            motion::Entry {
                input_type: input_manager::InputType::Keyboard,
                relations: vec![
                    motion::Relation::KeyCode(KeyCode::KeyW, motion::Axis::PosY),
                    motion::Relation::KeyCode(KeyCode::KeyS, motion::Axis::NegY),
                    motion::Relation::KeyCode(KeyCode::KeyD, motion::Axis::PosX),
                    motion::Relation::KeyCode(KeyCode::KeyA, motion::Axis::NegX),
                ],
            },
            motion::Entry {
                input_type: input_manager::InputType::Gamepad,
                relations: vec![
                    motion::Relation::GamepadAxis(GamepadAxis::LeftStickY, motion::Axis::Y),
                    motion::Relation::GamepadAxis(GamepadAxis::LeftStickX, motion::Axis::X),
                ],
            },
        ],
    );

    im.register_action_button(
        ABILITY_FLOATY,
        vec![
            button::Variant::Keyboard(KeyCode::KeyK),
            button::Variant::Gamepad(GamepadButton::North),
        ],
    );
    im.register_action_button(
        ABILITY_CORDYCEPT,
        vec![
            button::Variant::Keyboard(KeyCode::KeyJ),
            button::Variant::Gamepad(GamepadButton::East),
        ],
    );
}

fn spawn_player(mut commands: Commands, player_spawn: Res<PlayerSpawn>) {
    commands.spawn((Player, player_spawn.transform, fsm::Fsm::new()));
}

fn process_input(
    im: Res<InputManager>,
    yaw: Res<CameraYaw>,
    mut commands: Commands,
    mut moved_last_frame: Local<bool>,
) {
    if im.is_action_just_pressed(ABILITY_FLOATY) {
        commands.trigger(PlayerEvent::Floaty(PlayerFloatyEvent { active: true }));
    } else if im.is_action_just_released(ABILITY_FLOATY) {
        commands.trigger(PlayerEvent::Floaty(PlayerFloatyEvent { active: false }));
    }

    if im.is_action_just_pressed(ABILITY_CORDYCEPT) {
        commands.trigger(PlayerEvent::CordyCept(PlayerCordyCeptEvent {
            active: true,
        }));
    } else if im.is_action_just_released(ABILITY_CORDYCEPT) {
        commands.trigger(PlayerEvent::CordyCept(PlayerCordyCeptEvent {
            active: false,
        }));
    }

    let Some(direction) = im.get_motion(MOVEMENT).get_motion_opt_y(yaw.get()) else {
        if *moved_last_frame {
            commands.trigger(PlayerEvent::Movement(PlayerMovementEvent { motion: None }));
        }
        *moved_last_frame = false;
        return;
    };
    *moved_last_frame = true;

    commands.trigger(PlayerEvent::Movement(PlayerMovementEvent {
        motion: Some(direction),
    }));
}

fn observe_event(
    event: Trigger<PlayerEvent>,
    mut fsm: Single<&mut fsm::Fsm, With<Player>>,
    mut transform: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    fsm.process_event(&event, &mut fsm::ContextAggregate(&mut transform, &time));
}

pub(super) mod fsm {
    use crate::player::states::idle_run::IdleRunState;

    use super::PlayerEvent;
    use bevy::prelude::*;

    pub struct ContextAggregate<'a>(pub &'a mut Transform, pub &'a Time);

    pub trait TState: Send + Sync {
        fn get_name(&self) -> &'static str;
        fn enter_state(&self, event: &PlayerEvent, aggregate: &mut ContextAggregate);
        fn process_event(
            &self,
            event: &PlayerEvent,
            aggregate: &mut ContextAggregate,
        ) -> Option<Box<dyn TState>>;
    }

    #[derive(Component)]
    pub(super) struct Fsm {
        current_state: Box<dyn TState>,
    }
    impl Fsm {
        pub fn new() -> Self {
            Self {
                current_state: Box::new(IdleRunState),
            }
        }

        pub(super) fn process_event(
            &mut self,
            event: &PlayerEvent,
            anim_update: &mut ContextAggregate,
        ) {
            let new_state = self.current_state.process_event(event, anim_update);

            let Some(new_state) = new_state else {
                return;
            };

            debug!("Player enter state {}", new_state.get_name());
            self.current_state = new_state;
            self.current_state.enter_state(event, anim_update);
        }
    }
}
