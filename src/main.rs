use bevy::prelude::*;
use bevy::app::AppExit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (sys_spawn_camera, sys_spawn_ui))
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
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::srgb(0.0, 0.3, 0.9).into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "Plow n' Plate",
                TextStyle {
                    font: asset_server.load("fonts/Roboto.ttf"),
                    font_size: 80.0,
                    ..default()
                },
            )
        );
    });
}
