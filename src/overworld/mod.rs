use bevy::prelude::*;

use self::{
    camera::Camera,
    interactables::{battle_starter::BattleStarter, dialog_starter::DialogStarter},
    player::Player,
};

use crate::hud_area_label::HudAreaLabel;
use crate::AppState;

pub mod camera;
pub mod interactables;
pub mod player;

/// Marker for despawning when exiting `AppState::Overworld`
#[derive(Component)]
pub struct StateCleanup;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Overworld)
                .with_system(setup_overworld)
                .with_system(show_area_title),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Overworld)
                .with_system(player::move_player)
                .with_system(camera::rotate_camera)
                .with_system(
                    interactables::interactable_interact::<BattleStarter>
                        .chain(interactables::battle_starter::interactable_start_battle),
                )
                .with_system(
                    interactables::interactable_interact::<DialogStarter>
                        .chain(interactables::dialog_starter::interactable_start_dialog),
                )
                .with_system(back_to_menu),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Overworld).with_system(crate::despawn_all::<StateCleanup>),
        );
    }
}

fn setup_overworld(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut s_materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_entity = spawn_player(&mut commands, &mut meshes, &mut s_materials);
    let camera_entity = spawn_camera(&mut commands);

    commands
        .entity(player_entity)
        .insert_children(0, &[camera_entity]);

    spawn_interactables(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut s_materials,
    );

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

fn spawn_player(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            material: materials.add(Color::rgb(0.1, 0.8, 0.2).into()),
            ..Default::default()
        })
        .insert(StateCleanup);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1., 2., 1.))),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(0., 1.0, 0.)),
            ..Default::default()
        })
        .insert(StateCleanup)
        .insert(Player { speed: 10. })
        .id()
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

    commands.entity(root).insert_children(0, &[camera]);

    root
}

fn spawn_interactables(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    s_materials: &mut Assets<StandardMaterial>
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1., 1., 1.))),
            material: s_materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(5., 1.0, 5.)),
            ..Default::default()
        })
        .insert(BattleStarter)
        .insert(StateCleanup);

    let ferris_handle = asset_server.load("sprites/ferris-happy.png");
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1., 2., 1.))),
            material: s_materials.add(Color::RED.into()),
            transform: Transform::from_translation(Vec3::new(-5., 1.0, 5.)),
            ..Default::default()
        })
        .insert(DialogStarter {
            npc_name: "Ferris".to_string(),
            sprite: ferris_handle.into(),
        })
        .insert(StateCleanup);
}

fn show_area_title(mut hud: ResMut<HudAreaLabel>) {
    hud.show_area_title("Overworld");
}

pub fn back_to_menu(mut state: ResMut<State<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set(AppState::MainMenu).unwrap();
    }
}
