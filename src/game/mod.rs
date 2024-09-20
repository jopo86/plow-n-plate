mod farm;
mod kitchen;

use bevy::prelude::*;

use crate::states::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((farm::FarmPlugin, kitchen::KitchenPlugin));
        app.init_state::<GameState>();
    }
}

const SPRITE_SCALE: f32 = 3.0;

#[derive(Component)]
struct GameObj;
