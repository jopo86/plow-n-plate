use bevy::prelude::*;

use crate::global::states::MenuState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>();
    }
}
