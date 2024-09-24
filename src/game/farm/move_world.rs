use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

use crate::game::GameObj;
use crate::global::resources::MousePos;
use crate::global::state::{Game, AppState};

pub struct MoveWorldPlugin;

impl Plugin for MoveWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (sys_move_with_mouse, sys_scale_with_scroll).run_if(in_state(AppState::Game(Game::Farm))),
        );
    }
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
        let scale_factor = 1.0
            + ev.y
                * match ev.unit {
                    MouseScrollUnit::Line => 0.1,
                    MouseScrollUnit::Pixel => 0.05,
                };

        if let (Ok(window), Ok((camera, camera_transform))) =
            (q_window.get_single(), q_camera.get_single())
        {
            if let Some(screen_pos) = window.cursor_position() {
                // no idea how this shit works i got it from copilot
                if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
                    // convert screen coords to world coords
                    for mut transform in q_plots_and_crops.iter_mut() {
                        let offset = transform.translation - world_pos.extend(0.0);

                        transform.scale *= scale_factor;
                        transform.translation = world_pos.extend(0.0) + offset * scale_factor;
                    }
                }
            }
        }
    }
}
