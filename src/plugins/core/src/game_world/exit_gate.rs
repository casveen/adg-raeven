use bevy::prelude::*;

pub(super) struct ExitGatePlugin;
impl Plugin for ExitGatePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(destroy_exit_gate)
            .register_type::<ExitGate>();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ExitGate;

#[derive(Event)]
pub struct DestroyExitGate;

// This system destroys all exit gates
// We assume there is only one exit game for this game-demo
fn destroy_exit_gate(
    _: Trigger<DestroyExitGate>,
    mut commands: Commands,
    q_exit_gates: Query<(Entity, &ExitGate)>,
) {
    for (entity, _) in q_exit_gates.iter() {
        commands.entity(entity).remove_parent().despawn();
    }

    // Add other effects here
    // ...
}
