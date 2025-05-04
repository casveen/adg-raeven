use avian3d::prelude::CollidingEntities;
use bevy::prelude::*;

use crate::player::controller::Player;

pub(super) struct WinScreenPlugin;
impl Plugin for WinScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_collision)
            .add_observer(player_win_event)
            .register_type::<WinScreen>();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WinScreen;

#[derive(Event)]
pub struct WinEvent;

fn player_collision(
    q_winscreen_collider: Query<Entity, With<WinScreen>>,
    q_player: Query<&CollidingEntities, With<Player>>,
    mut commands: Commands,
) {
    for wc_entity in q_winscreen_collider.iter() {
        for p_colliding_entities in q_player.iter() {
            if p_colliding_entities.contains(&wc_entity) {
                commands.trigger(WinEvent)
            }
        }
    }
}

fn player_win_event(_: Trigger<WinEvent>, mut local_check: Local<bool>) {
    if *local_check {
        return;
    }
    *local_check = true;
    info!("!!! PLAYER WINS");
    warn!("!!! PLAYER WINS");
    error!("!!! PLAYER WINS");
}
