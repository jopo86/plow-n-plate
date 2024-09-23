use crate::game::GameObj;
use bevy::prelude::*;

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
