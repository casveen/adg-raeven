use bevy::prelude::*;

use super::{framerate::framerate, title};

pub struct VendorPlugin;
impl Plugin for VendorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(framerate::FrameratePlugin)
            .insert_resource(title::Title::default())
            .add_systems(Startup, title::startup_set_title);
    }
}
