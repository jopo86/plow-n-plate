mod farm;
mod kitchen;
mod inventory;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((farm::FarmPlugin, kitchen::KitchenPlugin, inventory::InventoryPlugin));
    }
}

const SPRITE_SCALE: f32 = 3.0;

#[derive(Component)]
struct GameObj;
