use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::window::PrimaryWindow;

const SCALE: f32 = 3.0;

enum PlotType {
    Grass,
    Dirt,
}

enum CropType {
    Wheat,
}

#[derive(Component)]
struct Plot;

#[derive(Component)]
struct Crop;

// TODO: lots of cleanup to be done obviously, just getting a little demo going

/*
 * CONVENTIONS (for systems) (this may get annoying idk)
 * - prefix fn name with sys_
 * - prefix queries with q_
 * - prefix event writers with ew_
 * - prefix event readers with er_
 * - cmd for Commands
 * - resources (and cmd) should be the only names w/o prefixes
 * - put each arg on its own line
 */

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (sys_spawn_camera, sys_spawn_ui, sys_spawn_plots))
        .add_systems(Update, (sys_close_if_esc, sys_move_with_mouse, sys_scale_with_scroll))
        .run();
}

fn sys_close_if_esc(
    r_kbd: Res<ButtonInput<KeyCode>>,
    mut ew_exit: EventWriter<AppExit>,
) {
    if r_kbd.just_pressed(KeyCode::Escape) {
        ew_exit.send(AppExit::Success);
    }
}

fn sys_spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn sys_spawn_ui(
    mut cmd: Commands,
    asset_server: Res<AssetServer>
) {
    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::srgba(0.0, 0.3, 0.9, 0.0).into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "Plow n' Plate",
                TextStyle {
                    font: asset_server.load("fonts/main.ttf"),
                    font_size: 80.0,
                    ..default()
                },
            )
        );
    });
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
    mut q_plots_and_crops: Query<&mut Transform, Or<(With<Plot>, With<Crop>)>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut er_mouse: EventReader<MouseMotion>,
) {
    if !mouse_button.pressed(MouseButton::Left) && !mouse_button.pressed(MouseButton::Right) {
        return;
    }

    for ev in er_mouse.read() {
        for mut transform in q_plots_and_crops.iter_mut() {
            transform.translation.x += ev.delta.x;
            transform.translation.y -= ev.delta.y;
        }
    }
}

fn sys_scale_with_scroll(
    mut q_plots_and_crops: Query<&mut Transform, Or<(With<Plot>, With<Crop>)>>,
    mut er_scroll: EventReader<MouseWheel>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // TODO: add scale boundaries so u cant zoom in/out forever

    for ev in er_scroll.read() {
        let scale_factor = 1.0 + ev.y * match ev.unit {
            MouseScrollUnit::Line => 0.05,
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

fn spawn_plot(
    plot_type: PlotType,
    mut pos: Vec3,
    cmd: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    pos *= 16.0 * SCALE;

    cmd.spawn((
        SpriteBundle {
            texture: asset_server.load(match plot_type {
                PlotType::Dirt => "textures/dirt.png",
                PlotType::Grass => "textures/grass.png",
            }),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z).with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Plot,
    ));
}

fn spawn_crop(
    crop_type: CropType,
    mut pos: Vec3,
    cmd: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    pos *= 16.0 * SCALE;
    pos.y += 8.0 * SCALE; // account for 16x32 texture, we want the origin to be 1/4 up, not the center

    cmd.spawn((
        SpriteBundle {
            texture: asset_server.load(match crop_type {
                CropType::Wheat => "textures/wheat.png",
            }),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z).with_scale(Vec3::splat(SCALE)),
            ..default()
        },
        Crop,
    ));
}
