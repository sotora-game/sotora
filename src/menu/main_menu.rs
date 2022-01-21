use bevy::app::AppExit;
use bevy::prelude::*;

use crate::menu::button;
use crate::AppState;
use crate::UiAssets;

/// Marker for despawning when exiting `AppState::MainMenu`
#[derive(Component)]
pub struct StateCleanup;

pub fn button_exit_app(In(clicked): In<bool>, mut app_exit: EventWriter<AppExit>) {
    if clicked {
        app_exit.send(AppExit);
    }
}

pub fn button_enter_game(In(clicked): In<bool>, mut state: ResMut<State<AppState>>) {
    if clicked {
        state.set(AppState::Overworld).unwrap();
    }
}

pub fn button_open_settings_menu(In(clicked): In<bool>, mut state: ResMut<State<AppState>>) {
    if clicked {
        state.set(AppState::SettingsMenu).unwrap();
    }
}

pub fn setup(mut commands: Commands, assets: Res<UiAssets>) {
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
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: assets.transparent,
            ..Default::default()
        })
        .insert(StateCleanup)
        .with_children(|root| {
            // Game title
            root.spawn_bundle(TextBundle {
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
            });
            root.spawn_bundle(TextBundle {
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
            });
            // Spacer
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(16.0)),
                    ..Default::default()
                },
                color: assets.transparent,
                ..Default::default()
            });
            // Menu panel
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Auto),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Stretch,
                    ..Default::default()
                },
                color: assets.menu_panel_background,
                ..Default::default()
            })
            .with_children(|menu| {
                // Play button
                menu.spawn_bundle(ButtonBundle {
                    color: assets.button_normal,
                    style: button_style.clone(),
                    ..Default::default()
                })
                .insert(button::EnterGame)
                .with_children(|button| {
                    button.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Play",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
                // Spacer
                menu.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(16.0)),
                        ..Default::default()
                    },
                    color: assets.transparent,
                    ..Default::default()
                });
                // Settings button
                menu.spawn_bundle(ButtonBundle {
                    color: assets.button_normal,
                    style: button_style.clone(),
                    ..Default::default()
                })
                .insert(button::OpenSettingsMenu)
                .with_children(|button| {
                    button.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Settings",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
                // Quit button
                menu.spawn_bundle(ButtonBundle {
                    color: assets.button_normal,
                    style: button_style.clone(),
                    ..Default::default()
                })
                .insert(button::ExitApp)
                .with_children(|button| {
                    button.spawn_bundle(TextBundle {
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
