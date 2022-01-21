use bevy::prelude::*;

use crate::AppState;
use crate::UiAssets;

/// Marker for despawning when exiting `AppState::Dialog`
#[derive(Component)]
pub struct StateCleanup;

pub struct DialogPlugin;
impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Dialog).with_system(setup_dialog))
            .add_system_set(SystemSet::on_update(AppState::Dialog).with_system(back_to_overworld))
            .add_system_set(
                SystemSet::on_exit(AppState::Dialog)
                    .with_system(crate::despawn_all::<StateCleanup>),
            );
    }
}

fn back_to_overworld(mut state: ResMut<State<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set(AppState::Overworld).unwrap();
    }
}

pub struct DialogResource {
    pub npc_name: String,
    pub sprite: Handle<Image>,
}

pub fn setup_dialog(mut commands: Commands, assets: Res<UiAssets>, dialog: Res<DialogResource>) {
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
            // NPC image
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(60.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: assets.white,
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        // TODO Fix this so it's not streched
                        size: Size::new(Val::Percent(60.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    image: dialog.sprite.clone().into(),
                    ..Default::default()
                });
            });
            // Text box
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Stretch,
                    ..Default::default()
                },
                color: assets.black,
                ..Default::default()
            })
            .with_children(|textbox| {
                // NPC name
                textbox
                    .spawn_bundle(NodeBundle {
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
                        color: assets.black,
                        ..Default::default()
                    })
                    .with_children(|root| {
                        root.spawn_bundle(TextBundle {
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
                    });
                // Messages
                textbox
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                            flex_direction: FlexDirection::ColumnReverse,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: assets.black,
                        ..Default::default()
                    })
                    .with_children(|root| {
                        root.spawn_bundle(TextBundle {
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
