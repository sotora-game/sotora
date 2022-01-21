use bevy::prelude::*;

use self::camera::Camera;

use crate::hud_area_label::HudAreaLabel;
use crate::AppState;

pub mod camera;

/// Marker for despawning when exiting `AppState::Battle`
#[derive(Component)]
pub struct StateCleanup;

pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Battle)
                .with_system(setup_battle)
                .with_system(show_area_title),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Battle)
                .with_system(camera::rotate_camera)
                .with_system(back_to_overworld),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Battle).with_system(crate::despawn_all::<StateCleanup>),
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
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(5.0, 10.0, 5.0),
            point_light: PointLight {
                color: Color::rgb(0.5, 0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(StateCleanup);
}

fn spawn_board(
    commands: &mut Commands,
    asset_server: &mut AssetServer,
    materials: &mut Assets<StandardMaterial>,
) {
    let mesh = asset_server.load("meshes/hex.gltf#Mesh0/Primitive0");

    commands
        .spawn_bundle(PbrBundle {
            mesh,
            transform: Transform::from_scale(Vec3::splat(0.8)), // More magic numbers
            material: materials.add(Color::WHITE.into()),
            ..Default::default()
        })
        .insert(StateCleanup);
}

fn spawn_camera(commands: &mut Commands) -> Entity {
    let mut transform = Transform::from_translation(Vec3::new(0., 15., -15.));
    transform.look_at(Vec3::ZERO, Vec3::Y);

    let root = commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Camera)
        .id();

    let camera = commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform,
            ..Default::default()
        })
        .insert(StateCleanup)
        .id();

    commands.entity(root).push_children(&[camera]);

    root
}

fn show_area_title(mut hud: ResMut<HudAreaLabel>) {
    hud.show_area_title("The battle of Bevytown");
}

fn back_to_overworld(mut state: ResMut<State<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set(AppState::Overworld).unwrap();
    }
}
