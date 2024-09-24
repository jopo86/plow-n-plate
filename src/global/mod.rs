pub mod colors;
pub mod resources;
pub mod state;
mod systems;

use bevy::prelude::*;

use resources::*;
use systems::*;

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePos>();
        app.add_systems(Startup, sys_spawn_camera);
        app.add_systems(Update, (sys_close_if_esc, sys_update_mouse_pos));
    }
}
