use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use super::{CropType, PlotType, WorldScale};
use crate::game::{
    inventory::{Inventory, Item},
    GameObj, SPRITE_SCALE,
};
use crate::global::{
    math,
    resources::MousePos,
    state::{AppState, Game},
};

pub struct InteractWorldPlugin;

// The drag timer allows for differentiation between clicks to drag the world and to just click on the world.
#[derive(Resource)]
pub struct DragTimer(pub Timer);

impl Default for DragTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Once))
    }
}

impl Plugin for InteractWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DragTimer>();
        app.add_systems(
            Update,
            (
                sys_move_with_mouse,
                sys_scale_with_scroll,
                sys_click,
                sys_tick_drag_timer,
            )
                .run_if(in_state(AppState::Game(Game::Farm))),
        );
    }
}

fn sys_move_with_mouse(
    mut q_plots_and_crops: Query<&mut Transform, With<GameObj>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_pos: Res<MousePos>,
    mut drag_timer: ResMut<DragTimer>,
) {
    if (mouse_button.pressed(MouseButton::Left) || mouse_button.pressed(MouseButton::Right))
        && (mouse_pos.dx != 0.0 || mouse_pos.dy != 0.0)
    {
        for mut transform in q_plots_and_crops.iter_mut() {
            transform.translation.x += mouse_pos.dx;
            transform.translation.y -= mouse_pos.dy;
        }

        drag_timer.0.reset();
    }
}

fn sys_tick_drag_timer(mut drag_timer: ResMut<DragTimer>, time: Res<Time>) {
    drag_timer.0.tick(time.delta());
}

fn sys_scale_with_scroll(
    mut q_plots_and_crops: Query<&mut Transform, With<GameObj>>,
    mut er_scroll: EventReader<MouseWheel>,
    mouse_pos: Res<MousePos>,
    mut world_scale: ResMut<WorldScale>,
) {
    for ev in er_scroll.read() {
        let scale_factor = 1.0
            + ev.y
                * match ev.unit {
                    MouseScrollUnit::Line => 0.1,
                    MouseScrollUnit::Pixel => 0.05,
                };

        world_scale.0 *= scale_factor;

        if world_scale.0 < WorldScale::MIN {
            world_scale.0 = WorldScale::MIN;
            return;
        } else if world_scale.0 > WorldScale::MAX {
            world_scale.0 = WorldScale::MAX;
            return;
        }

        for mut transform in q_plots_and_crops.iter_mut() {
            let offset = transform.translation - mouse_pos.world;

            transform.scale *= scale_factor;
            transform.translation = mouse_pos.world + offset * scale_factor;
        }
    }
}

fn sys_click(
    mut cmd: Commands,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    mouse_pos: Res<MousePos>,
    drag_timer: Res<DragTimer>,
    world_scale: Res<WorldScale>,
    mut inventory: ResMut<Inventory>,
    q_crops: Query<(Entity, &Transform, &CropType)>,
    q_plots: Query<(Entity, &Transform, &PlotType)>,
) {
    if mouse_btn.just_released(MouseButton::Left) && drag_timer.0.finished() {
        // only register clicks if world wasn't being dragged
        for (e, transform, crop_type) in &q_crops {
            if math::pt_rect_collision(
                mouse_pos.world.xy(),
                transform.translation.xy() - Vec2::new(0.0, 8.0 * SPRITE_SCALE), // account for crop origin adjustment
                Vec2::splat(16.0 * SPRITE_SCALE * world_scale.0),
            ) {
                cmd.entity(e).despawn();
                match crop_type {
                    CropType::Wheat => {
                        inventory.add(Item::Wheat, 1);
                    }
                }
                return;
            }
        }

        for (e, transform, plot_type) in &q_plots {
            if math::pt_rect_collision(
                mouse_pos.world.xy(),
                transform.translation.xy(),
                Vec2::splat(16.0 * SPRITE_SCALE * world_scale.0),
            ) {
                // TODO
            }
        }
    }
}
