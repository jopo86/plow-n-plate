mod farm;
mod kitchen;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((farm::FarmPlugin, kitchen::KitchenPlugin));
    }
}

const SPRITE_SCALE: f32 = 3.0;

#[derive(Component)]
struct GameObj;
