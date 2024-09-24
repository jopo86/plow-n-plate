use bevy::prelude::*;

use crate::global::{
    state::{State, Menu, Game},
    colors::CustomColors,
};
use super::helpers::top_bar::{TopBarBtn, TopBarBtnType};

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sys_top_bar_interactions.run_if(in_state(State::Game(Game::Farm))));
    }
}

fn sys_top_bar_interactions(
    mut q_buttons: Query<
        (&Interaction, &mut BackgroundColor, &TopBarBtn),
        (Changed<Interaction>, With<Button>)
    >,
    mut next_state: ResMut<NextState<State>>,
) {
    for (interaction, mut bg_col, btn) in &mut q_buttons {
        match *interaction {
            Interaction::Pressed => match btn.0 {
                TopBarBtnType::Back => next_state.set(State::Menu(Menu::Main)),
                _ => {},
            }
            Interaction::Hovered => *bg_col = Color::GRAY_1.into(),
            Interaction::None => *bg_col = Color::GRAY_0.into(),
        }
    }
}
