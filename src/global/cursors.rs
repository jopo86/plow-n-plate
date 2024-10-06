use bevy::prelude::*;

use super::{assets::cursors, MousePos};

pub struct CursorsPlugin;

impl Plugin for CursorsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CursorState>()
            .add_systems(Startup, sys_spawn_cursor)
            .add_systems(Update, sys_move_cursor.after(super::sys_update_mouse_pos))
            .add_systems(OnEnter(CursorState::Arrow), sys_set_cursor_arrow)
            .add_systems(OnEnter(CursorState::Pointer), sys_set_cursor_pointer)
            .add_systems(OnEnter(CursorState::Grab), sys_set_cursor_grab);
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CursorState {
    #[default]
    Arrow,
    Pointer,
    Grab,
}

#[derive(Component)]
pub struct Cursor;

fn sys_spawn_cursor(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn((
        ImageBundle {
            image: UiImage::new(assets.load(cursors::ARROW)),
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..Default::default()
        },
        Cursor,
    ));
}

fn sys_move_cursor(
    mut evr_cursor: EventReader<CursorMoved>,
    mut q_cursor: Query<&mut Style, With<Cursor>>,
) {
    if let Ok(mut style) = q_cursor.get_single_mut() {
        for ev in evr_cursor.read() {
            style.left = Val::Px(ev.position.x);
            style.top = Val::Px(ev.position.y);
        }
    }
}

fn sys_set_cursor_arrow(mut q_cursor: Query<&mut UiImage, With<Cursor>>, assets: Res<AssetServer>) {
    if let Ok(mut cursor_img) = q_cursor.get_single_mut() {
        *cursor_img = UiImage::new(assets.load(cursors::ARROW));
    }
}

fn sys_set_cursor_pointer(
    mut q_cursor: Query<&mut UiImage, With<Cursor>>,
    assets: Res<AssetServer>,
) {
    if let Ok(mut cursor_img) = q_cursor.get_single_mut() {
        *cursor_img = UiImage::new(assets.load(cursors::POINTER));
    }
}

fn sys_set_cursor_grab(mut q_cursor: Query<&mut UiImage, With<Cursor>>, assets: Res<AssetServer>) {
    if let Ok(mut cursor_img) = q_cursor.get_single_mut() {
        *cursor_img = UiImage::new(assets.load(cursors::GRAB));
    }
}
