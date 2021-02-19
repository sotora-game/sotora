use bevy::prelude::*;

use crate::AppState;
use crate::Stage;
use crate::UiAssets;

/// Marker for despawning when exiting `AppState::Dialog`
pub struct StateCleanup;

pub struct DialogPlugin;
impl Plugin for DialogPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(Stage::AppState, AppState::Dialog, setup_dialog.system())
            .on_state_update(
                Stage::AppState,
                AppState::Dialog,
                back_to_overworld.system(),
            )
            .on_state_exit(
                Stage::AppState,
                AppState::Dialog,
                crate::despawn_all::<StateCleanup>.system(),
            );
    }
}

fn back_to_overworld(mut state: ResMut<State<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set_next(AppState::Overworld).unwrap();
    }
}

pub struct DialogResource {
    pub npc_name: String,
    pub sprite: Handle<ColorMaterial>,
}

pub fn setup_dialog(commands: &mut Commands, assets: Res<UiAssets>, dialog: Res<DialogResource>) {
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
                // NPC image
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(60.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: assets.white.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            // TODO Fix this so it's not streched
                            size: Size::new(Val::Percent(60.0), Val::Percent(100.0)),
                            ..Default::default()
                        },
                        material: dialog.sprite.clone(),
                        ..Default::default()
                    });
                })
                // Text box
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Stretch,
                        ..Default::default()
                    },
                    material: assets.black.clone(),
                    ..Default::default()
                })
                .with_children(|textbox| {
                    textbox
                        // NPC name
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                flex_direction: FlexDirection::ColumnReverse,
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Stretch,
                                margin: Rect {
                                    // Magic number
                                    left: Val::Percent(3.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            material: assets.black.clone(),
                            ..Default::default()
                        })
                        .with_children(|root| {
                            root.spawn(TextBundle {
                                text: Text::with_section(
                                    &dialog.npc_name,
                                    TextStyle {
                                        font: assets.font_regular.clone(),
                                        font_size: 25.0,
                                        color: Color::rgb(0.9, 0.9, 0.95),
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        })
                        // Messages
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                                flex_direction: FlexDirection::ColumnReverse,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: assets.black.clone(),
                            ..Default::default()
                        })
                        .with_children(|root| {
                            root.spawn(TextBundle {
                                text: Text::with_section(
                                    "Hello there!",
                                    TextStyle {
                                        font: assets.font_regular.clone(),
                                        font_size: 25.0,
                                        color: Color::rgb(0.9, 0.9, 0.95),
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                });
        });
}
