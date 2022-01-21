use bevy::prelude::*;

use crate::menu::button;
use crate::AppState;
use crate::UiAssets;

/// Marker for despawning when exiting `AppState::SettingsMenu`
#[derive(Component)]
pub struct StateCleanup;

pub fn button_exit_settings_menu(In(clicked): In<bool>, mut state: ResMut<State<AppState>>) {
    if clicked {
        state.set(AppState::MainMenu).unwrap();
    }
}

pub fn setup(mut commands: Commands, assets: Res<UiAssets>) {
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

    // Container
    commands
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
            // Settings panel
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(400.0), Val::Auto),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    padding: Rect::all(Val::Px(8.0)),
                    ..Default::default()
                },
                color: assets.menu_panel_background,
                ..Default::default()
            })
            .with_children(|menu| {
                // Title
                menu.spawn_bundle(TextBundle {
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
                // Settings menu content placeholder
                menu.spawn_bundle(TextBundle {
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
            });
            // Spacer
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(8.0)),
                    ..Default::default()
                },
                color: assets.transparent,
                ..Default::default()
            });
            // Button bar under settings panel
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(400.0), Val::Auto),
                    justify_content: JustifyContent::FlexEnd,
                    ..Default::default()
                },
                color: assets.menu_panel_background,
                ..Default::default()
            })
            .with_children(|button_bar| {
                button_bar
                    // Back button
                    .spawn_bundle(ButtonBundle {
                        color: assets.button_normal,
                        style: button_style.clone(),
                        ..Default::default()
                    })
                    .insert(button::ExitSettingsMenu)
                    .with_children(|button| {
                        button.spawn_bundle(TextBundle {
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
