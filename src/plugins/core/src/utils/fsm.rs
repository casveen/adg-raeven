
/**
 * Basis of FSM system
 * Expects an entity hierarchy of
 * <main_entity>/<fsm_entity>/<current_state>
 * with this macro changing the current state
 * 
 * each state is an observer
 * 
 * Query example:
 * fsm: Single<Entity, With<PlayerFsm>>,
 * current_state: Single<&Children, With<PlayerFsm>>,
 * mut commands: Commands,
 */
#[macro_export]
macro_rules! new_state {
    ($commands:expr, $fsm:expr, $children:expr, $next_state:expr) => {{
        for c in *$children {
            $commands.entity(*c).remove_parent().despawn();
        }
        let new_state = $commands.add_observer($next_state).id();
        $commands.entity(*$fsm).insert_children(0, &[new_state]);
    }};
}
