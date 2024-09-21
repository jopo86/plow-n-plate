// Global is for states and systems/resources th
use bevy::prelude::*;

pub mod states;
mod systems;
pub mod resources;

use states::AppState;
use systems::*;
use resources::*;

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(AppState::Game);
        app.init_resource::<MousePos>();
        app.add_systems(Startup, (sys_spawn_camera, sys_spawn_ui));
        app.add_systems(Update, (sys_close_if_esc, sys_update_mouse_pos));
    }
}
