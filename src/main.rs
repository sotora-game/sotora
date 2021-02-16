use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::{ActiveCameras, Camera};

use battle::BattlePlugin;
use menu::MenuPlugin;
use overworld::OverworldPlugin;

use crate::hud_area_label::{
    setup_hud_area_label, update_hud_area_label, HudAreaLabel, HudAreaLabelAssets,
};
use crate::user_config::{KeyBinds, UserConfig};

mod battle;
mod hud_area_label;
mod menu;
mod overworld;
mod user_config;

/// Label for the AppState stage
const APPSTATES: &str = "AppStates";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    MainMenu,
    SettingsMenu,
    Overworld,
    Battle,
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
    cmd: &mut Commands,
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
        .add_startup_system(global_setup.system())
        // HUD area label
        .init_resource::<HudAreaLabelAssets>()
        .init_resource::<HudAreaLabel>()
        .add_startup_system(setup_hud_area_label.system())
        .add_system(update_hud_area_label.system())
        // AppState
        .insert_resource(State::new(AppState::MainMenu))
        .add_stage_before(stage::UPDATE, APPSTATES, StateStage::<AppState>::default())
        // State Plugins
        .add_plugin(MenuPlugin)
        .add_plugin(OverworldPlugin)
        .add_plugin(BattlePlugin)
        .run();
}

fn global_setup(commands: &mut Commands) {
    commands.spawn(UiCameraBundle::default());
}
