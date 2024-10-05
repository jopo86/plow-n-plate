use bevy::prelude::*;

use std::collections::HashMap;

use crate::global::state::in_game_state;

use super::farm::InventorySlotHudObj;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>();
        app.add_systems(Startup, sys_fill_inventory);
        app.add_systems(Update, sys_update_inventory.run_if(in_game_state));
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
            Self::Wheat => "textures/wheat_icon.png",
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
        if let Some(new_val) = val.checked_sub(n) {
            // check u32 val > 0
            *val = new_val;
            true
        } else {
            println!("Attempted to take {n}x {item:?}, only have {}", self.get(item));
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

fn sys_update_inventory(
    inventory: Res<Inventory>,
    assets: Res<AssetServer>,
    mut q_slots: Query<(&mut Children, &mut UiImage), With<InventorySlotHudObj>>,
    mut q_text: Query<&mut Text>,
) {
    if !inventory.is_changed() {
        return;
    }

    for ((item, count), (mut children, mut img)) in inventory.0.iter().zip(q_slots.iter_mut()) {
        if *count == 0 {
            continue;
        }

        *img = UiImage::new(assets.load(item.get_texture_path()));
        q_text.get_mut(children[0]).unwrap().sections[0].value = count.to_string();
    }
}
