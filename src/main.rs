use bevy::prelude::*;
use menu::MenuPlugin;
use overworld::OverworldPlugin;

use crate::user_config::{KeyBinds, UserConfig};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

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
}

/// Despawn all entities with given component
///
/// Useful for streamlined cleanup
fn despawn_all<T: Component>(cmd: &mut Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
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
        // AppState
        .insert_resource(State::new(AppState::MainMenu))
        .add_stage_before(stage::UPDATE, APPSTATES, StateStage::<AppState>::default())
        // State Plugins
        .add_plugin(MenuPlugin)
        .add_plugin(OverworldPlugin)
        .run();
}

fn global_setup(commands: &mut Commands) {
    commands.spawn(UiCameraBundle::default());
}
