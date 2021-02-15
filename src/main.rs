use bevy::prelude::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

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
    Overworld,
}

fn main() {
    /// Label for the AppState stage
    const APPSTATES: &str = "AppStates";

    let n = 0;
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
        // AppState
        .insert_resource(State::new(AppState::Overworld))
        .add_stage_before(stage::UPDATE, APPSTATES, StateStage::<AppState>::default())
        // Overworld
        .on_state_enter(APPSTATES, AppState::Overworld, overworld::setup_overworld.system())
        .on_state_update(APPSTATES, AppState::Overworld, overworld::player::move_player.system())
        .on_state_update(APPSTATES, AppState::Overworld, overworld::camera::rotate_camera.system())
        .on_state_exit(APPSTATES, AppState::Overworld, despawn_all::<overworld::StateCleanup>.system())
        .run();
}
