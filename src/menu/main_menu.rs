use bevy::app::AppExit;
use bevy::prelude::*;

use crate::menu::{button, MenuAssets};
use crate::AppState;

/// Marker for despawning when exiting `AppState::MainMenu`
pub struct StateCleanup;

pub fn button_exit_app(In(clicked): In<bool>, mut app_exit: ResMut<Events<AppExit>>) {
    if clicked {
        app_exit.send(AppExit);
    }
}

pub fn button_enter_game(In(clicked): In<bool>, mut state: ResMut<State<AppState>>) {
    if clicked {
        state.set_next(AppState::Overworld).unwrap();
    }
}

pub fn button_open_settings_menu(In(clicked): In<bool>, mut state: ResMut<State<AppState>>) {
    if clicked {
        state.set_next(AppState::SettingsMenu).unwrap();
    }
}

pub fn setup(commands: &mut Commands, assets: Res<MenuAssets>) {
    let button_style = Style {
        size: Size::new(Val::Auto, Val::Auto),
        margin: Rect::all(Val::Px(5.0)),
        padding: Rect {
            left: Val::Px(12.0),
            right: Val::Px(12.0),
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
        },
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_text_style = TextStyle {
        font: assets.font_light.clone(),
        font_size: 18.0,
        color: Color::WHITE,
    };

    commands
        // Container
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: assets.transparent.clone(),
            ..Default::default()
        })
        .with(StateCleanup)
        .with_children(|root| {
            root
                // Game title
                .spawn(TextBundle {
                    text: Text::with_section(
                        "SOTORA",
                        TextStyle {
                            font: assets.font_regular.clone(),
                            font_size: 25.0,
                            color: Color::rgb(0.9, 0.9, 0.95),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .spawn(TextBundle {
                    text: Text::with_section(
                        "a bevy community game",
                        TextStyle {
                            font: assets.font_light.clone(),
                            font_size: 15.0,
                            color: Color::rgb(0.7, 0.7, 0.75),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                // Spacer
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(16.0)),
                        ..Default::default()
                    },
                    material: assets.transparent.clone(),
                    ..Default::default()
                })
                // Menu panel
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Auto),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Stretch,
                        ..Default::default()
                    },
                    material: assets.menu_panel_background.clone(),
                    ..Default::default()
                })
                .with_children(|menu| {
                    menu
                        // Play button
                        .spawn(ButtonBundle {
                            material: assets.button_normal.clone(),
                            style: button_style.clone(),
                            ..Default::default()
                        })
                        .with(button::EnterGame)
                        .with_children(|button| {
                            button.spawn(TextBundle {
                                text: Text::with_section(
                                    "Play",
                                    button_text_style.clone(),
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        })
                        // Spacer
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(16.0)),
                                ..Default::default()
                            },
                            material: assets.transparent.clone(),
                            ..Default::default()
                        })
                        // Settings button
                        .spawn(ButtonBundle {
                            material: assets.button_normal.clone(),
                            style: button_style.clone(),
                            ..Default::default()
                        })
                        .with(button::OpenSettingsMenu)
                        .with_children(|button| {
                            button.spawn(TextBundle {
                                text: Text::with_section(
                                    "Settings",
                                    button_text_style.clone(),
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        })
                        // Quit button
                        .spawn(ButtonBundle {
                            material: assets.button_normal.clone(),
                            style: button_style.clone(),
                            ..Default::default()
                        })
                        .with(button::ExitApp)
                        .with_children(|button| {
                            button.spawn(TextBundle {
                                text: Text::with_section(
                                    "Quit",
                                    button_text_style.clone(),
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                });
        });
}
