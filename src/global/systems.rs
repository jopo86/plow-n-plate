// Systems that run for all states

use bevy::{prelude::*, window::PrimaryWindow};

use super::resources::*;

pub fn sys_close_if_esc(kbd: Res<ButtonInput<KeyCode>>, mut ew_exit: EventWriter<AppExit>) {
    if kbd.just_pressed(KeyCode::Escape) {
        ew_exit.send(AppExit::Success);
    }
}

pub fn sys_spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

pub fn sys_update_mouse_pos(
    mut mouse_pos: ResMut<MousePos>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = q_window.get_single() {
        if let Some(pos) = window.cursor_position() {
            // Deltas will remain the same if the user drags outside of the window.
            // This check mitigates it, but it will still happen if they are dragging fast enough
            if pos.x >= window.width() * 0.98
                || pos.x <= window.width() * 0.02
                || pos.y >= window.height() * 0.98
                || pos.y <= window.height() * 0.02
            {
                (mouse_pos.dx, mouse_pos.dy) = (0.0, 0.0);
                return;
            }

            mouse_pos.dx = pos.x - mouse_pos.x;
            mouse_pos.dy = pos.y - mouse_pos.y;
            mouse_pos.x = pos.x;
            mouse_pos.y = pos.y;
        }
    }
}
