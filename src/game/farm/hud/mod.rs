mod helpers;
mod spawn_hud;
mod interactions;

use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn_hud::SpawnHudPlugin,
            interactions::InteractionsPlugin,
        ));
    }
}
