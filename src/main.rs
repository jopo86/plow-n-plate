mod game;
mod global;
mod menu;

use bevy::prelude::*;
use global::state::{Menu, Game, AppState};

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
        // .insert_state(AppState::Menu(Menu::Main))
        .insert_state(AppState::Game(Game::Farm)) // temporary
        .add_plugins((global::GlobalPlugin, game::GamePlugin, menu::MenuPlugin))
        .run();
}
