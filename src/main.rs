use bevy::prelude::*;

use crate::menu::MenuAssets;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod menu;
mod overworld;

/// Despawn all entities with given component
///
/// Useful for streamlined cleanup
fn despawn_all<T: Component>(cmd: &mut Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.despawn_recursive(e);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    MainMenu,
    SettingsMenu,
    Overworld,
}

fn main() {
    /// Label for the AppState stage
    const APPSTATES: &str = "AppStates";

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
        .init_resource::<MenuAssets>()
        .add_startup_system(global_setup.system())
        // AppState
        .insert_resource(State::new(AppState::MainMenu))
        .add_stage_before(stage::UPDATE, APPSTATES, StateStage::<AppState>::default())
        // Main menu
        .on_state_enter(
            APPSTATES,
            AppState::MainMenu,
            menu::main_menu::setup.system(),
        )
        .on_state_update(
            APPSTATES,
            AppState::MainMenu,
            menu::button_interact.system(),
        )
        .on_state_exit(
            APPSTATES,
            AppState::MainMenu,
            despawn_all::<menu::main_menu::StateCleanup>.system(),
        )
        // Settings menu
        .on_state_enter(
            APPSTATES,
            AppState::SettingsMenu,
            menu::settings::setup.system(),
        )
        .on_state_update(
            APPSTATES,
            AppState::SettingsMenu,
            menu::button_interact.system(),
        )
        .on_state_exit(
            APPSTATES,
            AppState::SettingsMenu,
            despawn_all::<menu::settings::StateCleanup>.system(),
        )
        // Overworld
        .on_state_enter(
            APPSTATES,
            AppState::Overworld,
            overworld::setup_overworld.system(),
        )
        .on_state_update(
            APPSTATES,
            AppState::Overworld,
            overworld::player::move_player.system(),
        )
        .on_state_update(
            APPSTATES,
            AppState::Overworld,
            overworld::camera::rotate_camera.system(),
        )
        .on_state_update(
            APPSTATES,
            AppState::Overworld,
            overworld::back_to_menu.system(),
        )
        .on_state_exit(
            APPSTATES,
            AppState::Overworld,
            despawn_all::<overworld::StateCleanup>.system(),
        )
        .run();
}

fn global_setup(commands: &mut Commands) {
    commands.spawn(UiCameraBundle::default());
}
