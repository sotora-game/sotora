use bevy::prelude::*;

use self::camera::Camera;

use crate::AppState;
use crate::APPSTATES;

pub mod camera;

/// Marker for despawning when exiting `AppState::Battle`
pub struct StateCleanup;

pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(APPSTATES, AppState::Battle, setup_battle.system())
            .on_state_update(APPSTATES, AppState::Battle, camera::rotate_camera.system())
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
    let _camera_entity = spawn_camera(commands);

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

fn spawn_camera(commands: &mut Commands) -> Entity {
    let mut transform = Transform::from_translation(Vec3::new(0., 15., -15.));
    transform.look_at(Vec3::zero(), Vec3::unit_y());

    let root = commands
        .spawn(())
        .with(Transform::default())
        .with(GlobalTransform::default())
        .with(Camera)
        .current_entity()
        .unwrap();

    let camera = commands
        .spawn(PerspectiveCameraBundle {
            transform,
            ..Default::default()
        })
        .with(StateCleanup)
        .current_entity()
        .unwrap();

    commands.push_children(root, &[camera]);

    root
}

fn back_to_overworld(mut state: ResMut<State<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set_next(AppState::Overworld).unwrap();
    }
}
