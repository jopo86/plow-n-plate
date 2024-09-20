mod states;
mod game;
mod menu;

use bevy::prelude::*;
use bevy::app::AppExit;

use states::AppState;

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
        .add_plugins((game::GamePlugin, menu::MenuPlugin))
        .insert_state(AppState::Game) // ! change to `.init_resource::<AppState>()` once menu / main menu states are created
        .add_systems(Startup, (sys_spawn_camera, sys_spawn_ui))
        .add_systems(Update, sys_close_if_esc)
        .run();
}

fn sys_close_if_esc(
    kbd: Res<ButtonInput<KeyCode>>,
    mut ew_exit: EventWriter<AppExit>,
) {
    if kbd.just_pressed(KeyCode::Escape) {
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
