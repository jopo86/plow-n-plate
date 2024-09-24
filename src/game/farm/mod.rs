mod hud;
mod spawn_world;
mod move_world;

use bevy::prelude::*;
use super::GameObj;

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

pub enum PlotType {
    Grass,
    Dirt,
}

pub enum CropType {
    Wheat,
}

#[derive(Component)]
pub struct FarmGameObj;

#[derive(Component)]
pub struct FarmHudObj;

#[derive(Component)]
pub struct Plot;

#[derive(Component)]
pub struct Crop;

#[derive(Bundle)]
pub struct PlotBundle(pub SpriteBundle, pub GameObj, pub FarmGameObj, pub Plot);

#[derive(Bundle)]
pub struct CropBundle(pub SpriteBundle, pub GameObj, pub FarmGameObj, pub Crop);
