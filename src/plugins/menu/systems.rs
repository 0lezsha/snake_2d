use crate::{resources::*, AppState};

use super::components::*;
use bevy::{app::AppExit, prelude::*};

const DEFAULT_BUTTON_COLOR: Color = Color::NONE;
const HOVERED_BUTTON_COLOR: Color = Color::DARK_GREEN;
const PRESSED_BUTTON_COLOR: Color = Color::GREEN;

pub fn init_state(mut commands: Commands, score: Res<Score>) {
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect::vertical(Val::Px(6.)),
                ..default()
            },
            ..default()
        })
        .id();

    // Title
    let title = commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Snake Game",
                TextStyle {
                    font_size: 42.,
                    ..default()
                },
            ));
        })
        .id();

    commands.entity(root).push_children(&[title]);
    //

    let containers_color = Color::rgba(0.11, 0.11, 0.11, 0.44);

    let menu_container = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(30.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect::vertical(Val::Px(6.)),
                ..default()
            },
            background_color: containers_color.into(),
            ..default()
        })
        .id();

    commands.entity(root).push_children(&[menu_container]);

    // Buttons
    let play_button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::vertical(Val::Px(8.)),
                    ..default()
                },
                background_color: DEFAULT_BUTTON_COLOR.into(),
                ..default()
            },
            ButtonAction::Play,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play",
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            ));
        })
        .id();

    commands
        .entity(menu_container)
        .push_children(&[play_button]);

    let quit_button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::vertical(Val::Px(8.)),
                    ..default()
                },
                background_color: DEFAULT_BUTTON_COLOR.into(),
                ..default()
            },
            ButtonAction::Quit,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Quit",
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            ));
        })
        .id();

    commands
        .entity(menu_container)
        .push_children(&[quit_button]);
    //

    let scoreboard_container = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(10.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: containers_color.into(),
            ..default()
        })
        .id();

    // Scores
    let max_score = commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::horizontal(Val::Percent(6.)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let value = format!("High Score is: {}", score.max);
            parent.spawn(TextBundle::from_section(value, TextStyle::default()));
        })
        .id();

    commands
        .entity(scoreboard_container)
        .push_children(&[max_score]);

    let last_score = commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::horizontal(Val::Percent(6.)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let value = format!("Last Score is: {}", score.current);
            parent.spawn(TextBundle::from_section(value, TextStyle::default()));
        })
        .id();

    commands
        .entity(scoreboard_container)
        .push_children(&[last_score]);
    //

    commands.entity(root).push_children(&[scoreboard_container]);
}

pub fn check_interactions(
    mut interactions: Query<
        (&mut BackgroundColor, &Interaction, &ButtonAction),
        Changed<Interaction>,
    >,
    mut app_state: ResMut<NextState<AppState>>,
    mut quit_app_event: EventWriter<AppExit>,
) {
    for (mut color, interaction, action) in interactions.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();

                match action {
                    ButtonAction::Play => {
                        app_state.set(AppState::Game);
                    }
                    ButtonAction::Quit => {
                        quit_app_event.send_default();
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = DEFAULT_BUTTON_COLOR.into();
            }
        }
    }
}
