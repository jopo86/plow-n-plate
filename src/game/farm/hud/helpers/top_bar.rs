use bevy::prelude::*;

use crate::game::farm::FarmHudObj;
use crate::global::colors::CustomColors;

#[derive(Component)]
pub struct TopBarBtn(pub TopBarBtnType);

pub enum TopBarBtnType {
    Back,
    Other,
}

pub fn build(
    parent: &mut ChildBuilder<'_>,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn((
        NodeBundle {
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.65).into(),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            style: Style {
                width: Val::Vw(98.0),
                height: Val::Vh(8.0),
                margin: UiRect::all(Val::Vw(1.0)),
                align_items: AlignItems::Center,
                column_gap: Val::Vh(1.0),
                padding: UiRect::left(Val::Vh(1.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        FarmHudObj,
    )).with_children(|parent| {
        build_button(parent, asset_server, "<--", TopBarBtnType::Back);
        build_button(parent, asset_server, "?", TopBarBtnType::Other);
        build_button(parent, asset_server, "?", TopBarBtnType::Other);
        build_button(parent, asset_server, "?", TopBarBtnType::Other);
        build_button(parent, asset_server, "?", TopBarBtnType::Other);
    });
}

fn build_button(
    parent: &mut ChildBuilder<'_>,
    asset_server: &Res<AssetServer>,
    text: &str,
    btn_type: TopBarBtnType,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Vh(6.0),
                    height: Val::Vh(6.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::GRAY_0.into(),
                border_color: Color::BLACK.into(),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..Default::default()
            },
            FarmHudObj,
            TopBarBtn(btn_type),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        color: Color::WHITE,
                        font: asset_server.load("fonts/Main.ttf"),
                        font_size: 24.0,
                        ..Default::default()
                    },
                ),
                FarmHudObj,
            ));
        });
}
