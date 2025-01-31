use bevy::prelude::*;

const TITLE: &str = "SporoMancer";

#[derive(Resource)]
pub struct Title(pub &'static str);
impl Default for Title {
    fn default() -> Self {
        Self(TITLE)
    }
}

pub(super) fn startup_set_title(mut windows: Query<&mut Window>, title: Res<Title>) {
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    window.title = title.0.to_string();
}
