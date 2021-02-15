use bevy::prelude::*;

use crate::AppState;
use crate::menu::{ClickAction, MenuMaterials, StateCleanup};

pub fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    materials: Res<MenuMaterials>,
) {
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
        font: asset_server.load("fonts/sansation/sansation_light.ttf"),
        font_size: 18.0,
        color: Color::WHITE,
    };

    commands
        .spawn(UiCameraBundle::default())
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
            material: materials.transparent.clone(),
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
                            font: asset_server.load("fonts/sansation/sansation_regular.ttf"),
                            font_size: 25.0,
                            color: Color::rgb(0.9, 0.9, 0.95),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .spawn(TextBundle {
                    text: Text::with_section(
                        "a bevy game",
                        TextStyle {
                            font: asset_server.load("fonts/sansation/sansation_light.ttf"),
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
                    material: materials.transparent.clone(),
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
                    material: materials.menu_panel_background.clone(),
                    ..Default::default()
                })
                .with_children(|menu| {
                    menu
                        // Play button
                        .spawn(ButtonBundle {
                            material: materials.button_normal.clone(),
                            style: button_style.clone(),
                            ..Default::default()
                        })
                        .with(ClickAction::ChangeState(AppState::Overworld))
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
                            material: materials.transparent.clone(),
                            ..Default::default()
                        })
                        // Settings button
                        .spawn(ButtonBundle {
                            material: materials.button_normal.clone(),
                            style: button_style.clone(),
                            ..Default::default()
                        })
                        .with(ClickAction::ChangeState(AppState::SettingsMenu))
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
                            material: materials.button_normal.clone(),
                            style: button_style.clone(),
                            ..Default::default()
                        })
                        .with(ClickAction::Exit)
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
