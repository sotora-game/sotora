use bevy::pbr::AmbientLight;
use bevy::prelude::*;

use self::{camera::Camera, player::Player};
use crate::AppState;

pub mod camera;
pub mod player;

/// Marker for despawning when exiting `AppState::Overworld`
pub struct StateCleanup;

pub fn setup_overworld(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut light: ResMut<AmbientLight>,
) {
    light.color = Color::rgb(0.9, 0.9, 0.9);

    let _player_entity = spawn_player(commands, &mut meshes, &mut materials);
    let _camera_entity = spawn_camera(commands);

    // FIXME re-enable this when https://github.com/bevyengine/bevy/issues/1452 is addressed so the camera despawns again
    //commands.push_children(player_entity, &[camera_entity]);
}

fn spawn_player(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            material: materials.add(Color::rgb(0.1, 0.8, 0.2).into()),
            ..Default::default()
        })
        .with(StateCleanup)
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1., 2., 1.))),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(0., 1.0, 0.)),
            ..Default::default()
        })
        .with(StateCleanup)
        .with(Player { speed: 10. })
        .current_entity()
        .unwrap()
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
        .current_entity()
        .unwrap();

    commands.push_children(root, &[camera]);

    root
}

pub fn back_to_menu(mut state: ResMut<State<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set_next(AppState::MainMenu).unwrap();
    }
}
