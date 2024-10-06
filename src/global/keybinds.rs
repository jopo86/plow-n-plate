use std::collections::HashMap;

use bevy::prelude::*;

pub struct KeybindsPlugin;

impl Plugin for KeybindsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Keybinds>();
        app.add_systems(Startup, sys_fill_keybinds);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    ToggleInventory,
    LockCamera,
    // TODO: more actions...
}

#[derive(Resource)]
pub struct Keybinds(pub HashMap<Action, KeyCode>);

impl Keybinds {
    pub fn get(&self, action: Action) -> KeyCode {
        self.0.get(&action).unwrap().clone()
    }
}

impl Default for Keybinds {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

fn sys_fill_keybinds(mut keybinds: ResMut<Keybinds>) {
    keybinds.0.insert(Action::ToggleInventory, KeyCode::Tab);
    keybinds.0.insert(Action::LockCamera, KeyCode::ControlLeft);
    // TODO: more keybinds...
}
