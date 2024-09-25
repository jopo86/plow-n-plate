use bevy::prelude::*;

use crate::global::{
    colors::CustomColors,
    state::{Game, Menu, AppState},
};

use super::helpers::top_bar;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            sys_top_bar_interactions.run_if(in_state(AppState::Game(Game::Farm))),
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
            Interaction::Hovered => *bg_col = Color::FG_HOVER.into(),
            Interaction::None => *bg_col = Color::FG_NORMAL.into(),
        }
    }
}
