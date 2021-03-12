use bevy::render::camera::{ActiveCameras, Camera};
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::component::Component,
};
use bevy::{ecs::schedule::ReportExecutionOrderAmbiguities, prelude::*};

use battle::BattlePlugin;
use dialog::DialogPlugin;
use menu::MenuPlugin;
use overworld::OverworldPlugin;

use crate::hud_area_label::{
    setup_hud_area_label, update_hud_area_label, HudAreaLabel, HudAreaLabelAssets,
};
use crate::user_config::{KeyBinds, UserConfig};

mod battle;
mod dialog;
mod hud_area_label;
mod menu;
mod overworld;
mod user_config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
pub enum Stage {
    AppState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    MainMenu,
    SettingsMenu,
    Overworld,
    Battle,
    Dialog,
}

/// Despawn all entities with given component
///
/// Useful for streamlined cleanup
///
/// ## Camera workaround
/// Make sure to include a `T` cleanup marker component when spawning a camera that
/// needs to be cleaned up, otherwise the game will panic when the camera despawns.
///
/// This is a workaround for [Bevy issue #1452](https://github.com/bevyengine/bevy/issues/1452).
fn despawn_all<T: Component>(
    mut cmd: Commands,
    query: Query<Entity, With<T>>,
    mut cameras: ResMut<ActiveCameras>,
    camera_query: Query<&Camera, With<T>>,
) {
    // FIXME workaround for https://github.com/bevyengine/bevy/issues/1452 - should be removed when the issue is fixed upstream
    for camera in camera_query.iter() {
        if let Some(name) = &camera.name {
            // When a camera despawns it doesn't seem to get removed from ActiveCameras,
            // causing a panic when the Bevy internal code tries to `unwrap` on a nonexistent
            // entity. By manually setting the active camera to None, we make sure that Bevy
            // doesn't try to use it.
            //
            // Removing this key altogether does not seem to be a good idea because it doesn't
            // get re-added when spawning a new camera (meaning the new camera isn't activated
            // at all).
            cameras.cameras.insert(name.clone(), None);
        }
    }

    for e in query.iter() {
        cmd.despawn_recursive(e);
    }
}

fn main() {
    App::build()
        // Bevy configurations
        .insert_resource(ReportExecutionOrderAmbiguities)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            title: "SOTORA™ PRE-ALPHA".into(),
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .insert_resource(KeyBinds::load())
        .init_resource::<UiAssets>()
        .add_startup_system(global_setup.system())
        // HUD area label
        .init_resource::<HudAreaLabelAssets>()
        .init_resource::<HudAreaLabel>()
        .add_startup_system(setup_hud_area_label.system())
        .add_system(update_hud_area_label.system())
        // AppState
        .insert_resource(State::new(AppState::MainMenu))
        .add_stage_before(
            CoreStage::Update,
            Stage::AppState,
            StateStage::<AppState>::default(),
        )
        // State Plugins
        .add_plugin(MenuPlugin)
        .add_plugin(OverworldPlugin)
        .add_plugin(BattlePlugin)
        .add_plugin(DialogPlugin)
        .run();
}

fn global_setup(mut commands: Commands) {
    commands.spawn(UiCameraBundle::default());
}

pub struct UiAssets {
    button_normal: Handle<ColorMaterial>,
    button_hover: Handle<ColorMaterial>,
    button_active: Handle<ColorMaterial>,

    menu_panel_background: Handle<ColorMaterial>,

    transparent: Handle<ColorMaterial>,
    white: Handle<ColorMaterial>,
    black: Handle<ColorMaterial>,

    font_light: Handle<Font>,
    font_light_italic: Handle<Font>,
    font_regular: Handle<Font>,
    #[allow(unused)]
    font_regular_italic: Handle<Font>,
    font_bold: Handle<Font>,
    #[allow(unused)]
    font_bold_italic: Handle<Font>,
}

impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let assets = world.get_resource_mut::<AssetServer>().unwrap();

        UiAssets {
            button_normal: materials.add(Color::rgb(0.3, 0.3, 0.36).into()),
            button_hover: materials.add(Color::rgb(0.4, 0.4, 0.46).into()),
            button_active: materials.add(Color::rgb(0.24, 0.24, 0.32).into()),

            menu_panel_background: materials.add(Color::rgb(0.2, 0.2, 0.24).into()),

            transparent: materials.add(Color::NONE.into()),
            white: materials.add(Color::WHITE.into()),
            black: materials.add(Color::BLACK.into()),

            font_light: assets.load("fonts/sansation/Sansation-Light.ttf"),
            font_light_italic: assets.load("fonts/sansation/Sansation-LightItalic.ttf"),
            font_regular: assets.load("fonts/sansation/Sansation-Regular.ttf"),
            font_regular_italic: assets.load("fonts/sansation/Sansation-Italic.ttf"),
            font_bold: assets.load("fonts/sansation/Sansation-Bold.ttf"),
            font_bold_italic: assets.load("fonts/sansation/Sansation-BoldItalic.ttf"),
        }
    }
}
