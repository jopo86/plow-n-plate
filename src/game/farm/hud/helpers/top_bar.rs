use bevy::prelude::*;

use crate::game::farm::FarmHudObj;
use crate::global::colors::CustomColors;

#[derive(Component)]
pub enum Action {
    Back,
    Other,
}

pub fn build(parent: &mut ChildBuilder, assets: &Res<AssetServer>) {
    parent
        .spawn((
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
        ))
        .with_children(|parent| {
            build_img_button(parent, assets, "icons/3.png", Action::Back);
            build_text_button(parent, assets, "?", Action::Other);
            build_text_button(parent, assets, "?", Action::Other);
            build_text_button(parent, assets, "?", Action::Other);
            build_text_button(parent, assets, "?", Action::Other);
        });
}

fn build_text_button(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    text: &str,
    action: Action,
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
            action,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        color: Color::WHITE,
                        font: assets.load("fonts/Main.ttf"),
                        font_size: 24.0,
                        ..Default::default()
                    },
                ),
                FarmHudObj,
            ));
        });
}

fn build_img_button(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    img_path: &'static str,
    action: Action,
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
            action,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(assets.load(img_path)),
                    transform: Transform::from_scale(Vec3::splat(2.0)),
                    ..Default::default()
                },
                FarmHudObj,
            ));
        });
}
