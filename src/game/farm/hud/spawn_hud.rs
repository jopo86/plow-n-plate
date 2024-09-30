use bevy::prelude::*;

use crate::{
    game::farm::FarmHudObj,
    global::{
        colors::CustomColors,
        state::{Game, AppState},
    },
};

use super::helpers::{top_bar, inventory};

pub struct SpawnHudPlugin;

impl Plugin for SpawnHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game(Game::Farm)), sys_spawn_hud);
        app.add_systems(OnExit(AppState::Game(Game::Farm)), sys_despawn_hud);
    }
}

fn sys_spawn_hud(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn((
        NodeBundle {
            background_color: Color::TRANSPARENT.into(),
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        },
        FarmHudObj,
    ))
    .with_children(|parent| {
        top_bar::build(parent, &assets);
        inventory::build(parent, &assets);
    });
}

fn sys_despawn_hud(mut cmd: Commands, q_hud: Query<Entity, With<FarmHudObj>>) {
    for e in &q_hud {
        cmd.entity(e).despawn();
    }
}
