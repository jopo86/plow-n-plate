use bevy::prelude::*;

use super::MenuObj;
use crate::global::colors::CustomColors;
use crate::global::state::{Game, Menu, AppState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu(Menu::Main)), sys_spawn_ui);
        app.add_systems(OnExit(AppState::Menu(Menu::Main)), sys_despawn_ui);
        app.add_systems(
            Update,
            sys_interactions.run_if(in_state(AppState::Menu(Menu::Main))),
        );
    }
}

#[derive(Component)]
struct MainMenuObj;

#[derive(Component)]
enum Action {
    Play,
    Options,
    Quit,
}

fn sys_spawn_ui(mut cmd: Commands, assets: Res<AssetServer>) {
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
                    font: assets.load("fonts/Main.ttf"),
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

        build_button(parent, &assets, "Play", Action::Play);
        build_button(parent, &assets, "Options", Action::Options);
        build_button(parent, &assets, "Quit", Action::Quit);
    });
}

fn build_button(
    parent: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    text: &str,
    action: Action,
) {
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
            action,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        color: Color::WHITE,
                        font: assets.load("fonts/Main.ttf"),
                        font_size: 48.0,
                        ..Default::default()
                    },
                ),
                MenuObj,
                MainMenuObj,
            ));
        });
}

fn sys_despawn_ui(mut cmd: Commands, q_ui: Query<Entity, With<MainMenuObj>>) {
    for e in &q_ui {
        cmd.entity(e).despawn();
    }
}

fn sys_interactions(
    mut q_interaction: Query<
        (&Interaction, &mut BackgroundColor, &Action),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut ew_exit: EventWriter<AppExit>,
) {
    for (interaction, mut bg_col, action) in q_interaction.iter_mut() {
        match interaction {
            Interaction::Pressed => match action {
                Action::Play => next_state.set(AppState::Game(Game::Farm)),
                Action::Options => next_state.set(AppState::Menu(Menu::Options)),
                Action::Quit => {
                    ew_exit.send(AppExit::Success);
                }
            },
            Interaction::Hovered => {
                *bg_col = Color::GRAY_1.into();
            }
            Interaction::None => {
                *bg_col = Color::GRAY_0.into();
            }
        }
    }
}
