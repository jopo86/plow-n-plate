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
    pub fn all() -> &'static [Item] {
        &[Item::Wheat /*, ...*/]
    }

    pub fn get_texture_path(&self) -> &'static str {
        match self {
            Self::Wheat => "textures/wheat.png",
            // ...
        }
    }
}

#[derive(Resource)]
pub struct Inventory(pub HashMap<Item, u32>);

impl Inventory {
    pub fn get(&self, item: Item) -> u32 {
        *self.0.get(&item).unwrap()
    }

    pub fn add(&mut self, item: Item, n: u32) {
        *self.0.get_mut(&item).unwrap() += n;
    }

    pub fn take(&mut self, item: Item, n: u32) -> bool {
        let val = self.0.get_mut(&item).unwrap();
        if let Some(new_val) = val.checked_sub(n) { // check u32 val > 0
            *val = new_val;
            true
        } else {
            false
        }
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

fn sys_fill_inventory(mut inventory: ResMut<Inventory>) {
    for item in Item::all() {
        inventory.0.insert(*item, 0);
    }
}
