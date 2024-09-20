use bevy::prelude::*;
use crate::game::GameObj;

pub enum PlotType {
    Grass,
    Dirt,
}

pub enum CropType {
    Wheat,
}

#[derive(Component)]
pub struct Plot;

#[derive(Component)]
pub struct Crop;

#[derive(Bundle)]
pub struct PlotBundle(pub SpriteBundle, pub GameObj, pub Plot);

#[derive(Bundle)]
pub struct CropBundle(pub SpriteBundle, pub GameObj, pub Crop);
