mod game;
mod global;
mod menu;

use bevy::prelude::*;
use global::state::AppState;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Plow n' Plate".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .init_state::<AppState>()
        .add_plugins((global::GlobalPlugin, game::GamePlugin, menu::MenuPlugin))
        .run();
}
