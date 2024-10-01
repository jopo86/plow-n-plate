use bevy::prelude::*;

use crate::game::farm::FarmHudObj;
use crate::global::colors::CustomColors;
use crate::global::resources;

#[derive(Component)]
pub struct InventoryNode;

pub fn build(cmd: &mut Commands, assets: &Res<AssetServer>) {
    cmd.spawn((
        NodeBundle {
            background_color: Color::TRANSPARENT.into(),
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..Default::default()
            },
            ..Default::default()
        },
        FarmHudObj,
        InventoryNode,
    ))
    .with_children(|parent| {
        parent.spawn((
            NodeBundle {
                background_color: Color::HUD.into(),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::SpaceBetween,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(6.0)),
                    row_gap: Val::Px(4.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            FarmHudObj,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Inventory",
                    TextStyle {
                        font: assets.load("fonts/main.ttf"),
                        color: Color::WHITE,
                        font_size: 24.0,
                    },
                ).with_style(
                    Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    }
                ),
                FarmHudObj,
            ));
            for _ in 0..3 {
                build_slot_row(parent, assets);
            }
        });
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
                height: Val::Px(44.0),
                column_gap: Val::Px(4.0),
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
        ButtonBundle {
            background_color: Color::HUD.into(),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            style: Style {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                ..Default::default()
            },
            ..Default::default()
        },
        FarmHudObj,
    ))
    .with_children(|parent| {

    });
}
