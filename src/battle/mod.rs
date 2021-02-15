use bevy::pbr::AmbientLight;
use bevy::prelude::*;

use crate::AppState;
use crate::APPSTATES;

/// Marker for despawning when exiting `AppState::Battle`
pub struct StateCleanup;

pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(APPSTATES, AppState::Battle, setup_battle.system())
            // NOTE: Uses same camera as overworld
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut light: ResMut<AmbientLight>,
) {
    light.color = Color::rgb(0.9, 0.9, 0.9);

    spawn_board(commands, &mut meshes, &mut materials);
}

fn spawn_board(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    // Magic number
    let half_board_size = 7;

    for i in -half_board_size..half_board_size {
        for j in -half_board_size..half_board_size {
            // TODO Change for hexagons
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 0.8 })),
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    material: materials.add(Color::WHITE.into()),
                    ..Default::default()
                })
                .with(StateCleanup);
        }
    }
}

fn back_to_overworld(mut state: ResMut<State<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set_next(AppState::Overworld).unwrap();
    }
}
