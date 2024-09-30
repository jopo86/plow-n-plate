use bevy::prelude::*;

use std::collections::HashMap;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>();
        app.add_systems(Startup, sys_fill_inventory);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    Wheat,

}

impl Item {
    fn all() -> &'static [Item] {
        &[
            Item::Wheat,
        ]
    }
}

#[derive(Resource)]
struct Inventory(HashMap<Item, i32>);

impl Default for Inventory {
    fn default() -> Self {
        Inventory(HashMap::new())
    }
}

fn sys_fill_inventory(mut inventory: ResMut<Inventory>) {
    for item in Item::all() {
        inventory.0.insert(*item, 0);
    }
}
