mod hud;
mod interact_world;
mod spawn_world;

use super::GameObj;
use bevy::prelude::*;

pub struct FarmPlugin;

impl Plugin for FarmPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldScale>();
        app.add_plugins((
            spawn_world::SpawnWorldPlugin,
            interact_world::InteractWorldPlugin,
            hud::HudPlugin,
        ));
    }
}

#[derive(Component)]
pub enum PlotType {
    Grass,
    Dirt,
}

#[derive(Component)]
pub enum CropType {
    Wheat,
}

#[derive(Resource)]
pub struct WorldScale(f32);

impl WorldScale {
    pub const MIN: f32 = 0.3;
    pub const MAX: f32 = 7.5;
}

impl Default for WorldScale {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component)]
pub struct FarmGameObj;

#[derive(Component)]
pub struct FarmHudObj;

#[derive(Component)]
pub struct InventorySlotHudObj;

#[derive(Bundle)]
pub struct PlotBundle(pub SpriteBundle, pub GameObj, pub FarmGameObj, pub PlotType);

#[derive(Bundle)]
pub struct CropBundle(pub SpriteBundle, pub GameObj, pub FarmGameObj, pub CropType);
