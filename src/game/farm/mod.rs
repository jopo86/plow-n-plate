mod defs;
mod hud;
mod spawn_world;
mod move_world;

use crate::global::state::{Game, State};

use bevy::prelude::*;

pub struct FarmPlugin;

impl Plugin for FarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn_world::SpawnWorldPlugin,
            move_world::MoveWorldPlugin,
            hud::HudPlugin,
        ));
    }
}
