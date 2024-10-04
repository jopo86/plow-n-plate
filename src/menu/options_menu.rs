use bevy::prelude::*;

use super::MenuObj;
use crate::global::colors::CustomColors;
use crate::global::state::{AppState, Menu};

pub struct OptionsMenuPlugin;

impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu(Menu::Options)), sys_spawn_ui);
        app.add_systems(OnExit(AppState::Menu(Menu::Options)), sys_despawn_ui);
        app.add_systems(
            Update,
            sys_buttons.run_if(in_state(AppState::Menu(Menu::Options))),
        );
    }
}

#[derive(Component)]
struct OptionsMenuObj;

#[derive(Component)]
enum Action {
    Back,
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
            background_color: Color::BG.into(),
            ..Default::default()
        },
        MenuObj,
        OptionsMenuObj,
    ))
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Options",
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
            OptionsMenuObj,
        ));

        build_button(parent, &assets, "Back", Action::Back);
    });
}

fn build_button(parent: &mut ChildBuilder, assets: &Res<AssetServer>, text: &str, action: Action) {
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
                background_color: Color::FG_NORMAL.into(),
                border_color: Color::BLACK.into(),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..Default::default()
            },
            MenuObj,
            OptionsMenuObj,
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
                OptionsMenuObj,
            ));
        });
}

fn sys_despawn_ui(mut cmd: Commands, q_entities: Query<Entity, With<OptionsMenuObj>>) {
    for e in q_entities.iter() {
        cmd.entity(e).despawn();
    }
}

fn sys_buttons(
    mut q_interaction: Query<
        (&Interaction, &mut BackgroundColor, &Action),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, action) in q_interaction.iter_mut() {
        match interaction {
            Interaction::Pressed => match action {
                Action::Back => next_state.set(AppState::Menu(Menu::Main)),
            },
            Interaction::Hovered => *color = Color::FG_HOVER.into(),
            Interaction::None => *color = Color::FG_NORMAL.into(),
        }
    }
}
