mod main_menu;
mod options_menu;

use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((main_menu::MainMenuPlugin, options_menu::OptionsMenuPlugin));
    }
}

#[derive(Component)]
struct MenuObj;
