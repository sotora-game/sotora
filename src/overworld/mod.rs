use bevy::prelude::*;

use self::{
    camera::Camera,
    interactables::{battle_starter::BattleStarter, dialog_starter::DialogStarter},
    player::Player,
};

use crate::AppState;
use crate::APPSTATES;

pub mod camera;
pub mod interactables;
pub mod player;

/// Marker for despawning when exiting `AppState::Overworld`
pub struct StateCleanup;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(APPSTATES, AppState::Overworld, setup_overworld.system())
            .on_state_update(APPSTATES, AppState::Overworld, player::move_player.system())
            .on_state_update(
                APPSTATES,
                AppState::Overworld,
                camera::rotate_camera.system(),
            )
            .on_state_update(
                APPSTATES,
                AppState::Overworld,
                interactables::interactable_interact::<BattleStarter>
                    .system()
                    .chain(interactables::battle_starter::interactable_start_battle.system()),
            )
            .on_state_update(
                APPSTATES,
                AppState::Overworld,
                interactables::interactable_interact::<DialogStarter>
                    .system()
                    .chain(interactables::dialog_starter::interactable_start_dialog.system()),
            )
            .on_state_update(APPSTATES, AppState::Overworld, back_to_menu.system())
            .on_state_exit(
                APPSTATES,
                AppState::Overworld,
                crate::despawn_all::<StateCleanup>.system(),
            );
    }
}

fn setup_overworld(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut s_materials: ResMut<Assets<StandardMaterial>>,
    mut c_materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_entity = spawn_player(commands, &mut meshes, &mut s_materials);
    let camera_entity = spawn_camera(commands);

    commands.push_children(player_entity, &[camera_entity]);

    spawn_interactables(
        commands,
        &asset_server,
        &mut meshes,
        &mut s_materials,
        &mut c_materials,
    );

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
        .with(StateCleanup)
        .current_entity()
        .unwrap();

    commands.push_children(root, &[camera]);

    root
}

fn spawn_interactables(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    s_materials: &mut Assets<StandardMaterial>,
    c_materials: &mut Assets<ColorMaterial>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1., 1., 1.))),
            material: s_materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(5., 1.0, 5.)),
            ..Default::default()
        })
        .with(BattleStarter)
        .with(StateCleanup);

    let ferris_handle = asset_server.load("sprites/ferris-happy.png");
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1., 2., 1.))),
            material: s_materials.add(Color::RED.into()),
            transform: Transform::from_translation(Vec3::new(-5., 1.0, 5.)),
            ..Default::default()
        })
        .with(DialogStarter {
            npc_name: "Ferris".to_string(),
            sprite: c_materials.add(ferris_handle.into()),
        })
        .with(StateCleanup);
}

pub fn back_to_menu(mut state: ResMut<State<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        state.set_next(AppState::MainMenu).unwrap();
    }
}
