mod helpers;
mod interactions;
mod spawn_hud;

use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((spawn_hud::SpawnHudPlugin, interactions::InteractionsPlugin));
    }
}
