use bevy::prelude::*;

use crate::game::farm::FarmHudObj;
use crate::global::colors::CustomColors;
use crate::global::resources;

pub fn build(parent: &mut ChildBuilder, assets: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            background_color: Color::HUD_TRANSLUCENT.into(),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            style: Style {
                width: Val::Vw(50.0),
                height: Val::Vh(30.0),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        FarmHudObj,
    ))
    .with_children(|parent| {
        for _ in 0..3 {
            build_slot_row(parent, assets);
        }
    });
}

fn build_slot_row(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
) {
    parent.spawn((
        NodeBundle {
            background_color: Color::TRANSPARENT.into(),
            style: Style {
                width: Val::Percent(98.0),
                height: Val::Percent(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        FarmHudObj,
    ))
    .with_children(|parent| {
        for _ in 0..10 {
            build_slot(parent, assets);
        }
    });
}

fn build_slot(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
) {
    parent.spawn((
        NodeBundle {
            background_color: Color::HUD_TRANSLUCENT.into(),
            border_radius: BorderRadius::all(Val::Px(3.0)),
            style: Style {
                width: Val::Px(20.0),
                height: Val::Px(20.0),
                margin: UiRect::all(Val::Px(2.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        FarmHudObj,
    ))
    .with_children(|parent| {

    });
}
