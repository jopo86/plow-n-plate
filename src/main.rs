use bevy::prelude::*;
use bevy::app::AppExit;

const SCALE: f32 = 3.0;

enum QuadType {
    Grass,
    Dirt,
}

enum CropType {
    Wheat,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (sys_spawn_camera, sys_spawn_ui, sys_spawn_quads))
        .add_systems(Update, sys_close_if_esc)
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

fn sys_spawn_quads(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    for i in -1..=1 {
        for j in -1..=1 {
            spawn_quad(QuadType::Dirt, Vec3::new(i as f32, j as f32, 0.0), &mut cmd, &asset_server);
            spawn_crop(CropType::Wheat, Vec3::new(i as f32, j as f32, 1.0), &mut cmd, &asset_server);
        }
        
        spawn_quad(QuadType::Grass, Vec3::new(-2.0, i as f32, 0.0), &mut cmd, &asset_server);
        spawn_quad(QuadType::Grass, Vec3::new(i as f32, 2.0, 0.0), &mut cmd, &asset_server);

        spawn_quad(QuadType::Grass, Vec3::new(2.0, i as f32, 0.0), &mut cmd, &asset_server);
        spawn_quad(QuadType::Grass, Vec3::new(i as f32, -2.0, 0.0), &mut cmd, &asset_server);
    }

    spawn_quad(QuadType::Grass, Vec3::new(-2.0, -2.0, 0.0), &mut cmd, &asset_server);
    spawn_quad(QuadType::Grass, Vec3::new(-2.0,  2.0, 0.0), &mut cmd, &asset_server);
    spawn_quad(QuadType::Grass, Vec3::new( 2.0, -2.0, 0.0), &mut cmd, &asset_server);
    spawn_quad(QuadType::Grass, Vec3::new( 2.0,  2.0, 0.0), &mut cmd, &asset_server);
}

fn spawn_quad(
    quad_type: QuadType,
    mut pos: Vec3,
    cmd: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    pos *= 16.0 * SCALE;

    cmd.spawn(SpriteBundle {
        texture: asset_server.load(match quad_type {
            QuadType::Dirt => "textures/dirt.png",
            QuadType::Grass => "textures/grass.png",
        }),
        transform: Transform::from_xyz(pos.x, pos.y, pos.z).with_scale(Vec3::splat(SCALE)),
        ..default()
    });
}

fn spawn_crop(
    crop_type: CropType,
    mut pos: Vec3,
    cmd: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    pos *= 16.0 * SCALE;
    pos.y += 8.0 * SCALE;

    cmd.spawn(SpriteBundle {
        texture: asset_server.load(match crop_type {
            CropType::Wheat => "textures/wheat.png",
        }),
        transform: Transform::from_xyz(pos.x, pos.y, pos.z).with_scale(Vec3::splat(SCALE)),
        ..default()
    });
}
