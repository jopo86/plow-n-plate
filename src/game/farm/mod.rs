mod defs;
use defs::*;

use bevy::{input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel}, prelude::*, window::PrimaryWindow};
use crate::global::{states::{GameState, FarmState}, resources::MousePos};
use super::{GameObj, SPRITE_SCALE};

pub struct FarmPlugin;

impl Plugin for FarmPlugin {
    fn build(&self, app: &mut App) {
        // app.init_state::<FarmState>();   (can't do this yet because FarmState doesn't impl Default)
        app.add_systems(OnEnter(GameState::Farm), sys_spawn_plots);
        app.add_systems(
            Update, 
            (sys_move_with_mouse, sys_scale_with_scroll)
                .run_if(in_state(GameState::Farm))
        );
    }
}

fn sys_spawn_plots(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    for i in -1..=1 {
        for j in -1..=1 {
            spawn_plot(PlotType::Dirt, Vec3::new(i as f32, j as f32, 0.0), &mut cmd, &asset_server);
            spawn_crop(CropType::Wheat, Vec3::new(i as f32, j as f32, 1.0), &mut cmd, &asset_server);
        }
        
        spawn_plot(PlotType::Grass, Vec3::new(-2.0, i as f32, 0.0), &mut cmd, &asset_server);
        spawn_plot(PlotType::Grass, Vec3::new(i as f32, 2.0, 0.0), &mut cmd, &asset_server);

        spawn_plot(PlotType::Grass, Vec3::new(2.0, i as f32, 0.0), &mut cmd, &asset_server);
        spawn_plot(PlotType::Grass, Vec3::new(i as f32, -2.0, 0.0), &mut cmd, &asset_server);
    }

    spawn_plot(PlotType::Grass, Vec3::new(-2.0, -2.0, 0.0), &mut cmd, &asset_server);
    spawn_plot(PlotType::Grass, Vec3::new(-2.0,  2.0, 0.0), &mut cmd, &asset_server);
    spawn_plot(PlotType::Grass, Vec3::new( 2.0, -2.0, 0.0), &mut cmd, &asset_server);
    spawn_plot(PlotType::Grass, Vec3::new( 2.0,  2.0, 0.0), &mut cmd, &asset_server);
}

fn sys_move_with_mouse(
    mut q_plots_and_crops: Query<&mut Transform, With<GameObj>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_pos: Res<MousePos>, // custom resource
) {
    if !mouse_button.pressed(MouseButton::Left) && !mouse_button.pressed(MouseButton::Right) {
        return;
    }

    for mut transform in q_plots_and_crops.iter_mut() {
        transform.translation.x += mouse_pos.dx;
        transform.translation.y -= mouse_pos.dy;
    }
}

fn sys_scale_with_scroll(
    mut q_plots_and_crops: Query<&mut Transform, With<GameObj>>,
    mut er_scroll: EventReader<MouseWheel>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // TODO: add scale boundaries so u cant zoom in/out forever

    for ev in er_scroll.read() {
        let scale_factor = 1.0 + ev.y * match ev.unit {
            MouseScrollUnit::Line => 0.1,
            MouseScrollUnit::Pixel => 0.05,
        };

        let window = q_window.single();

        if let Some(screen_pos) = window.cursor_position() { // no idea how this shit works i got it from copilot
            let (camera, camera_transform) = q_camera.single();
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) { // convert screen coords to world coords
                for mut transform in q_plots_and_crops.iter_mut() {
                    let offset = transform.translation - world_pos.extend(0.0);

                    transform.scale *= scale_factor;
                    transform.translation = world_pos.extend(0.0) + offset * scale_factor;
                }
            }
        }
    }
}

fn spawn_plot(  // * not a system, just a helper function used from a system
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
            transform: Transform::from_xyz(pos.x, pos.y, pos.z).with_scale(Vec3::splat(SPRITE_SCALE)),
            ..default()
        },
        GameObj,
        Plot,
    ));
}

fn spawn_crop(  // * not a system, just a helper function used from a system
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
            transform: Transform::from_xyz(pos.x, pos.y, pos.z).with_scale(Vec3::splat(SPRITE_SCALE)),
            ..default()
        },
        GameObj,
        Crop,
    ));
}
