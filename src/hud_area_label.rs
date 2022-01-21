//! The HUD area label is a little decorative title that pops up at the top of
//! the screen when the player enters a new area in the world.

use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

/// How many animation frames there are for the open/close animation of the decorative images
const BORDER_ANIMATION_FRAMES: usize = 10;

/// Width of the decorative border images
const BORDER_WIDTH: f32 = 80.0;

/// Height of the decorative border images
const BORDER_HEIGHT: f32 = 10.0;

/// Padding in pixels between the decorative border images and the text
const TEXT_PADDING: f32 = 10.0;

/// How large the title text should be
const TITLE_FONT_SIZE: f32 = 14.0;

/// Offset of the title in pixels from the top of the window
const OFFSET_FROM_TOP: f32 = 40.0;

/// How long a title should be visible, in seconds
const TITLE_DISPLAY_TIME: f32 = 2.5;

/// How long the show/hide animation should last, in seconds
const TITLE_ANIMATION_TIME: f32 = 0.8;

/// Loads the assets required for the HUD area label
pub struct HudAreaLabelAssets {
    hud_area_border_frames: [Handle<Image>; BORDER_ANIMATION_FRAMES],
    transparent: UiColor,
    font_bold: Handle<Font>,
}

impl FromWorld for HudAreaLabelAssets {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let assets = world.get_resource_mut::<AssetServer>().unwrap();

        HudAreaLabelAssets {
            hud_area_border_frames: [
                assets.load("ui/hud_area_title_border/1.png").into(),
                assets.load("ui/hud_area_title_border/2.png").into(),
                assets.load("ui/hud_area_title_border/3.png").into(),
                assets.load("ui/hud_area_title_border/4.png").into(),
                assets.load("ui/hud_area_title_border/5.png").into(),
                assets.load("ui/hud_area_title_border/6.png").into(),
                assets.load("ui/hud_area_title_border/7.png").into(),
                assets.load("ui/hud_area_title_border/8.png").into(),
                assets.load("ui/hud_area_title_border/9.png").into(),
                assets.load("ui/hud_area_title_border/10.png").into(),
            ],

            transparent: Color::NONE.into(),

            font_bold: assets.load("fonts/sansation/Sansation-Bold.ttf"),
        }
    }
}

/// In-progress animation state
struct Animation {
    opening: bool,
    frame_index: usize,
    timer: Timer,
}

impl Animation {
    fn new_opening() -> Self {
        Animation {
            opening: true,
            frame_index: 0,
            timer: Timer::from_seconds(TITLE_ANIMATION_TIME / BORDER_ANIMATION_FRAMES as f32, true),
        }
    }

    fn new_closing() -> Self {
        Animation {
            opening: false,
            frame_index: BORDER_ANIMATION_FRAMES - 1,
            timer: Timer::from_seconds(TITLE_ANIMATION_TIME / BORDER_ANIMATION_FRAMES as f32, true),
        }
    }

    fn tick(mut self, delta_time: Duration) -> Option<Self> {
        self.timer.tick(delta_time);

        if self.timer.just_finished() {
            if (self.opening && self.frame_index >= BORDER_ANIMATION_FRAMES - 1)
                || (!self.opening && self.frame_index == 0)
            {
                None
            } else {
                if self.opening {
                    self.frame_index += 1;
                } else {
                    self.frame_index -= 1;
                }

                Some(self)
            }
        } else {
            Some(self)
        }
    }
}

/// Displays a temporary area label on top of the screen
#[derive(Default)]
pub struct HudAreaLabel {
    new_label: Option<String>,
    animation: Option<Animation>,
    hide_timer: Option<Timer>,
}

impl HudAreaLabel {
    fn is_fully_visible(&self) -> bool {
        self.animation.is_some()
    }

    /// Schedules a new area title to be displayed
    pub fn show_area_title<S: Into<String>>(&mut self, label: S) {
        self.new_label = Some(label.into().to_uppercase());
        self.animation = Some(Animation::new_opening());
        self.hide_timer = Some(Timer::from_seconds(TITLE_DISPLAY_TIME, false));
    }
}

/// Used to identify UI nodes belonging to the HUD area label
#[derive(Component)]
pub struct HudAreaLabelNode;

/// Updates the HUD area label system
pub fn update_hud_area_label(
    time: Res<Time>,
    assets: Res<HudAreaLabelAssets>,
    mut hud_label: ResMut<HudAreaLabel>,
    mut text_query: Query<&mut Text, With<HudAreaLabelNode>>,
    mut images: Query<&mut Handle<Image>, With<HudAreaLabelNode>>,
) {
    let delta_time = time.delta();

    if let Some(new_text) = hud_label.new_label.take() {
        if let Some(mut text) = text_query.iter_mut().next() {
            text.sections[0].value = new_text;
        }
    }

    if let Some(animation) = hud_label.animation.take() {
        hud_label.animation = animation.tick(delta_time);

        if let Some(animation) = &hud_label.animation {
            for mut image in images.iter_mut() {
                *image = assets.hud_area_border_frames[animation.frame_index].clone();
            }

            for mut text in text_query.iter_mut() {
                let opacity = animation.frame_index as f32 / BORDER_ANIMATION_FRAMES as f32;
                text.sections[0].style.color = Color::rgba(1.0, 1.0, 1.0, opacity);
            }
        } else if hud_label.is_fully_visible() {
            for mut image in images.iter_mut() {
                *image = assets.hud_area_border_frames[BORDER_ANIMATION_FRAMES - 1].clone();
            }
        }
    }

    if let Some(hide_timer) = &mut hud_label.hide_timer {
        hide_timer.tick(delta_time);

        if hide_timer.just_finished() {
            hud_label.animation = Some(Animation::new_closing());
        }
    }
}

/// Sets up the components necessary for the HUD area label
pub fn setup_hud_area_label(mut commands: Commands, assets: Res<HudAreaLabelAssets>) {
    commands
        // Root container at the top of the screen
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(OFFSET_FROM_TOP),
                    left: Val::Px(0.0),
                    right: Val::Auto,
                    bottom: Val::Auto,
                },
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: assets.transparent,
            ..Default::default()
        })
        .with_children(|root| {
            // Left side decorative image
            root.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(BORDER_WIDTH), Val::Px(BORDER_HEIGHT)),
                    ..Default::default()
                },
                image: assets.hud_area_border_frames[0].clone().into(),
                ..Default::default()
            })
            .insert(HudAreaLabelNode);

            // Text container with padding
            root.spawn_bundle(NodeBundle {
                style: Style {
                    padding: Rect {
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        left: Val::Px(TEXT_PADDING),
                        right: Val::Px(TEXT_PADDING),
                    },
                    ..Default::default()
                },
                color: assets.transparent,
                ..Default::default()
            })
            .with_children(|text_container| {
                text_container
                    .spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font: assets.font_bold.clone(),
                                font_size: TITLE_FONT_SIZE,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    })
                    .insert(HudAreaLabelNode);
            });

            // Right side decorative image
            root.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(BORDER_WIDTH), Val::Px(BORDER_HEIGHT)),
                    ..Default::default()
                },
                image: assets.hud_area_border_frames[0].clone().into(),
                transform: Transform {
                    rotation: Quat::from_rotation_z(PI),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(HudAreaLabelNode);
        });
}
