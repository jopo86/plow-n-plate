use bevy::prelude::*;

use crate::{
    game::{
        farm::InventorySlotHudObj,
        inventory::{Inventory, Item},
    },
    global::{
        colors::CustomColors,
        keybinds::{Action, Keybinds},
        state::{in_game_state, AppState, Game, Menu},
    },
};

use super::helpers::{inventory, top_bar};

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sys_top_bar_interactions.run_if(in_state(AppState::Game(Game::Farm))),
                (sys_show_or_hide_inventory, sys_update_inventory).run_if(in_game_state),
            ),
        );
    }
}

fn sys_top_bar_interactions(
    mut q_buttons: Query<
        (&Interaction, &mut BackgroundColor, &top_bar::Action),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    use top_bar::Action;

    for (interaction, mut bg_col, action) in &mut q_buttons {
        match interaction {
            Interaction::Pressed => match action {
                Action::Home => next_state.set(AppState::Menu(Menu::Main)),
                _ => {}
            },
            Interaction::Hovered => *bg_col = Color::HUD_HOVER.into(),
            Interaction::None => *bg_col = Color::HUD.into(),
        }
    }
}

fn sys_show_or_hide_inventory(
    mut q_inventory: Query<&mut Style, With<inventory::InventoryNode>>,
    kbd: Res<ButtonInput<KeyCode>>,
    keybinds: Res<Keybinds>,
) {
    if let Ok(mut inv_style) = q_inventory.get_single_mut() {
        if kbd.just_pressed(*keybinds.0.get(&Action::ToggleInventory).unwrap()) {
            inv_style.display = match inv_style.display {
                Display::None => Display::Flex,
                Display::Flex => Display::None,
                _ => Display::None,
            };
        }
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
