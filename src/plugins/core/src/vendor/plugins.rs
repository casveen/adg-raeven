use bevy::prelude::*;
use bevy_framepace::FramepacePlugin;

use super::framepace::framepace;
pub struct VendorPlugin;
impl Plugin for VendorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FramepacePlugin)
            .add_systems(Startup, (framepace::register_input, framepace::startup))
            .add_systems(Update, framepace::read_input);
    }
}
