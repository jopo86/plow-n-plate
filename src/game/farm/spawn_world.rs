use super::defs::*;

use bevy::prelude::*;

use crate::game::{GameObj, SPRITE_SCALE};
use crate::global::state::{Game, State};

pub struct SpawnWorldPlugin;

impl Plugin for SpawnWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Game(Game::Farm)), sys_spawn_world);
        app.add_systems(OnExit(State::Game(Game::Farm)), sys_despawn_world);
    }
}

fn sys_spawn_world(mut cmd: Commands, asset_server: Res<AssetServer>) {

    for i in -20..=20 {
        for j in -20..=20 {
            if (-1..=1).contains(&i) && (-1..=1).contains(&j) {
                continue;
            }
            spawn_plot(
                PlotType::Grass,
                Vec3::new(i as f32, j as f32, 0.0),
                &mut cmd,
                &asset_server,
            );
        }
    }

    for i in -1..=1 {
        for j in -1..=1 {
            spawn_plot(
                PlotType::Dirt,
                Vec3::new(i as f32, j as f32, 0.0),
                &mut cmd,
                &asset_server,
            );
            spawn_crop(
                CropType::Wheat,
                Vec3::new(i as f32, j as f32, 1.0),
                &mut cmd,
                &asset_server,
            );
        }
    }
}

fn sys_despawn_world(
    mut cmd: Commands,
    q_world: Query<Entity, With<FarmGameObj>>,
) {
    for e in &q_world {
        cmd.entity(e).despawn();
    }
}

fn spawn_plot(
    // * not a system, just a helper function used from a system
    plot_type: PlotType,
    mut pos: Vec3,
    cmd: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    pos *= 16.0 * SPRITE_SCALE;

    cmd.spawn(PlotBundle(
        SpriteBundle {
            texture: asset_server.load(match plot_type {
                PlotType::Dirt => "textures/dirt.png",
                PlotType::Grass => "textures/grass.png",
            }),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z)
                .with_scale(Vec3::splat(SPRITE_SCALE)),
            ..default()
        },
        GameObj,
        FarmGameObj,
        Plot,
    ));
}

fn spawn_crop(
    // * not a system, just a helper function used from a system
    crop_type: CropType,
    mut pos: Vec3,
    cmd: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    pos *= 16.0 * SPRITE_SCALE;
    pos.y += 8.0 * SPRITE_SCALE; // account for 16x32 texture, we want the origin to be 1/4 up, not the center

    cmd.spawn(CropBundle(
        SpriteBundle {
            texture: asset_server.load(match crop_type {
                CropType::Wheat => "textures/wheat.png",
            }),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z)
                .with_scale(Vec3::splat(SPRITE_SCALE)),
            ..default()
        },
        GameObj,
        FarmGameObj,
        Crop,
    ));
}
