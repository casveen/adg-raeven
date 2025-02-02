use bevy::prelude::*;

pub mod ant;

pub struct EnemiesPlugin;
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.register_type::<ant::AntRespawnTimer>()
            .add_systems(
                Update,
                (
                    ant::wall_collision,
                    ant::spawner_evaluate_spawning,
                    ant::tick_spawn_timers,
                    ant::respawn_timer.run_if(ant::respawn_timer_run_if),
                ),
            )
            .init_gizmo_group::<ant::CollisionGizmo>()
            .add_observer(ant::spawn_ant)
            .add_observer(ant::kill_ant)
            .add_observer(ant::cordyceptmovement);
    }
}
