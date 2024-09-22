use bevy::prelude::*;

use super::MenuObj;
use crate::global::state::{Game, Menu, State};
use crate::global::colors::CustomColors;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Menu(Menu::Main)), sys_spawn_ui);
        app.add_systems(OnExit(State::Menu(Menu::Main)), sys_despawn_ui);
        app.add_systems(
            Update,
            sys_buttons.run_if(in_state(State::Menu(Menu::Main))),
        );
    }
}

#[derive(Component)]
struct MainMenuObj;

fn sys_spawn_ui(mut cmd: Commands, asset_server: Res<AssetServer>) { //! kill me
    cmd.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::GRAY_2.into(),
            ..Default::default()
        },
        MenuObj,
        MainMenuObj,
    ))
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Plow n' Plate",
                TextStyle {
                    color: Color::WHITE,
                    font: asset_server.load("fonts/Main.ttf"),
                    font_size: 96.0,
                    ..Default::default()
                },
            )
            .with_style(Style {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..Default::default()
            }),
            MenuObj,
            MainMenuObj,
        ));

        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Auto,
                        padding: UiRect::axes(Val::Px(25.0), Val::Px(15.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::GRAY_0.into(),
                    border_color: Color::BLACK.into(),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..Default::default()
                },
                MenuObj,
                MainMenuObj,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Play",
                        TextStyle {
                            color: Color::WHITE,
                            font: asset_server.load("fonts/Main.ttf"),
                            font_size: 48.0,
                            ..Default::default()
                        },
                    ),
                    MenuObj,
                    MainMenuObj,
                ));
            });

        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Auto,
                        padding: UiRect::axes(Val::Px(25.0), Val::Px(15.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::GRAY_0.into(),
                    border_color: Color::BLACK.into(),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..Default::default()
                },
                MenuObj,
                MainMenuObj,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Options",
                        TextStyle {
                            color: Color::WHITE,
                            font: asset_server.load("fonts/Main.ttf"),
                            font_size: 48.0,
                            ..Default::default()
                        },
                    ),
                    MenuObj,
                    MainMenuObj,
                ));
            });
    });
}

fn sys_despawn_ui(mut cmd: Commands, q_entities: Query<Entity, With<MainMenuObj>>) {
    for e in q_entities.iter() {
        cmd.entity(e).despawn();
    }
}

fn sys_buttons(
    mut q_interaction: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    q_text: Query<&Text>,
    mut next_state: ResMut<NextState<State>>,
) {
    for (interaction, mut color, children) in q_interaction.iter_mut() {
        let text = &q_text.get(children[0]).unwrap().sections[0].value[..];
        match *interaction {
            Interaction::Pressed => match text {
                "Play" => next_state.set(State::Game(Game::Farm)),
                "Options" => next_state.set(State::Menu(Menu::Options)),
                _ => {}
            },
            Interaction::Hovered => {
                *color = Color::GRAY_1.into();
            }
            Interaction::None => {
                *color = Color::GRAY_0.into();
            }
        }
    }
}
