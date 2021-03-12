use bevy::prelude::*;

use self::camera::Camera;

use crate::hud_area_label::HudAreaLabel;
use crate::AppState;
use crate::Stage;

pub mod camera;

/// Marker for despawning when exiting `AppState::Battle`
pub struct StateCleanup;

pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(Stage::AppState, AppState::Battle, setup_battle.system())
            .on_state_enter(Stage::AppState, AppState::Battle, show_area_title.system())
            .on_state_update(
                Stage::AppState,
                AppState::Battle,
                camera::rotate_camera.system(),
            )
            .on_state_update(
                Stage::AppState,
                AppState::Battle,
                back_to_overworld.system(),
            )
            .on_state_exit(
                Stage::AppState,
                AppState::Battle,
                crate::despawn_all::<StateCleanup>.system(),
            );
    }
}

fn setup_battle(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_server: ResMut<AssetServer>,
) {
    spawn_board(&mut commands, &mut asset_server, &mut materials);
    let _camera_entity = spawn_camera(&mut commands);

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
    transform.look_at(Vec3::ZERO, Vec3::Y);

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

fn show_area_title(mut hud: ResMut<HudAreaLabel>) {
    hud.show_area_title("The battle of Bevytown");
}

fn back_to_overworld(mut state: ResMut<State<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set_next(AppState::Overworld).unwrap();
    }
}
