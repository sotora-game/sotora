use bevy::prelude::*;

use crate::menu::{button, MenuAssets};
use crate::AppState;

/// Marker for despawning when exiting `AppState::SettingsMenu`
pub struct StateCleanup;

pub fn button_exit_settings_menu(In(clicked): In<bool>, mut state: ResMut<State<AppState>>) {
    if clicked {
        state.set_next(AppState::MainMenu).unwrap();
    }
}

pub fn setup(commands: &mut Commands, assets: Res<MenuAssets>) {
    let button_style = Style {
        size: Size::new(Val::Auto, Val::Auto),
        margin: Rect::all(Val::Px(5.0)),
        padding: Rect {
            left: Val::Px(12.0),
            right: Val::Px(12.0),
            top: Val::Px(8.0),
            bottom: Val::Px(8.0),
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
                // Settings panel
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(400.0), Val::Auto),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        padding: Rect::all(Val::Px(8.0)),
                        ..Default::default()
                    },
                    material: assets.menu_panel_background.clone(),
                    ..Default::default()
                })
                .with_children(|menu| {
                    // Title
                    menu.spawn(TextBundle {
                        text: Text::with_section(
                            "SETTINGS",
                            TextStyle {
                                font: assets.font_bold.clone(),
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.95),
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
                    // Settings menu content placeholder
                    .spawn(TextBundle {
                        text: Text::with_section(
                            "coming soon",
                            TextStyle {
                                font: assets.font_light_italic.clone(),
                                font_size: 15.0,
                                color: Color::rgb(0.9, 0.9, 0.95),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                })
                // Spacer
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(8.0)),
                        ..Default::default()
                    },
                    material: assets.transparent.clone(),
                    ..Default::default()
                })
                // Button bar under settings panel
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(400.0), Val::Auto),
                        justify_content: JustifyContent::FlexEnd,
                        ..Default::default()
                    },
                    material: assets.menu_panel_background.clone(),
                    ..Default::default()
                })
                .with_children(|button_bar| {
                    button_bar
                        // Back button
                        .spawn(ButtonBundle {
                            material: assets.button_normal.clone(),
                            style: button_style.clone(),
                            ..Default::default()
                        })
                        .with(button::ExitSettingsMenu)
                        .with_children(|button| {
                            button.spawn(TextBundle {
                                text: Text::with_section(
                                    "Back",
                                    button_text_style.clone(),
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                });
        });
}
