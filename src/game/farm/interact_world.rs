use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

use crate::game::GameObj;
use crate::global::resources::MousePos;
use crate::global::state::{Game, AppState};

pub struct InteractWorldPlugin;

impl Plugin for InteractWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (sys_move_with_mouse, sys_scale_with_scroll, sys_click)
                .run_if(in_state(AppState::Game(Game::Farm))),
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
    mouse_pos: Res<MousePos>,
) {
    // TODO: add scale boundaries so u cant zoom in/out forever

    for ev in er_scroll.read() {
        let scale_factor = 1.0
            + ev.y
                * match ev.unit {
                    MouseScrollUnit::Line => 0.1,
                    MouseScrollUnit::Pixel => 0.05,
                };
        for mut transform in q_plots_and_crops.iter_mut() {
            let offset = transform.translation - mouse_pos.world;

            transform.scale *= scale_factor;
            transform.translation = mouse_pos.world + offset * scale_factor;
        }

    }
}

fn sys_click(
    mouse_btn: Res<ButtonInput<MouseButton>>,
    mouse_pos: Res<MousePos>,
    mut cmd: Commands,
    q_world: Query<(Entity, &Transform), With<GameObj>>,
) {
    // TODO: Make it so it won't detect click if the world was just dragged
    // TODO: Just use rectangle collision instead of this finding the closest one
    // TODO: Prioritize highest z-index (crops before plots)
    // TODO: Add wheat to inventory if clicked (as a start)

    if mouse_btn.just_released(MouseButton::Left) {
        let mut i: usize = 0;
        let mut min_len = f32::MAX;
        let mut e = None;

        for (j, (_e, transform)) in q_world.iter().enumerate() {
            let len = (transform.translation - mouse_pos.world).length();
            if len < min_len {
                min_len = len;
                i = j;
                e = Some(_e);
            }
        }

        if let Some(e) = e {
            cmd.entity(e).despawn();
        }
    }
}
