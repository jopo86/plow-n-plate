use bevy::prelude::*;

use super::defs::FarmHudObj;
use crate::global::{
    state::{State, Game},
    colors::CustomColors,
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Game(Game::Farm)), sys_spawn_hud);
        app.add_systems(OnExit(State::Game(Game::Farm)), sys_despawn_hud);
    }
}

fn sys_spawn_hud(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    cmd.spawn((
        NodeBundle {
            background_color: Color::TRANSPARENT.into(),
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        },
        FarmHudObj,
    )).with_children(|parent| {
        // TODO
    });
}

fn sys_despawn_hud(
    mut cmd: Commands,
    q_hud: Query<Entity, With<FarmHudObj>>,
) {
    for e in &q_hud {
        cmd.entity(e).despawn();
    }
}
