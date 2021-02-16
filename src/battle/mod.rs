use bevy::prelude::*;

use crate::AppState;
use crate::APPSTATES;

/// Marker for despawning when exiting `AppState::Battle`
pub struct StateCleanup;

pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(APPSTATES, AppState::Battle, setup_battle.system())
            // TODO: Uses same camera as overworld
            // For now we want the same behavior, and it doesn't get despawned when leaving
            // overworld, so spawning a new one doesn't work
            .on_state_update(
                APPSTATES,
                AppState::Battle,
                crate::overworld::camera::rotate_camera.system(),
            )
            .on_state_update(APPSTATES, AppState::Battle, back_to_overworld.system())
            .on_state_exit(
                APPSTATES,
                AppState::Battle,
                crate::despawn_all::<StateCleanup>.system(),
            );
    }
}

fn setup_battle(
    commands: &mut Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_server: ResMut<AssetServer>,
) {
    spawn_board(commands, &mut asset_server, &mut materials);

    commands
        .spawn(LightBundle {
            transform: Transform::from_xyz(5.0, 10.0, 5.0),
            light: Light {
                color: Color::rgb(0.5, 0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(StateCleanup);
}

fn spawn_board(
    commands: &mut Commands,
    asset_server: &mut AssetServer,
    materials: &mut Assets<StandardMaterial>,
) {
    let mesh = asset_server.load("meshes/hex.gltf#Mesh0/Primitive0");

    commands
        .spawn(PbrBundle {
            mesh,
            transform: Transform::from_scale(Vec3::splat(0.8)), // More magic numbers
            material: materials.add(Color::WHITE.into()),
            ..Default::default()
        })
        .with(StateCleanup);
}

fn back_to_overworld(mut state: ResMut<State<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set_next(AppState::Overworld).unwrap();
    }
}
