use bevy::prelude::*;

use crate::global::states::KitchenState;

pub struct KitchenPlugin;

impl Plugin for KitchenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<KitchenState>();
    }
}
