use bevy::input::mouse::MouseMotion;
use bevy::pbr::AmbientLight;
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 10.;

struct Camera;
struct Player;

fn setup_overworld(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut light: ResMut<AmbientLight>,
) {
    light.color = Color::rgb(0.9, 0.9, 0.9);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            material: materials.add(Color::rgb(0.1, 0.8, 0.2).into()),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1., 2., 1.))),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(0., 1.0, 0.)),
            ..Default::default()
        })
        .with(Player)
        .with_children(|parent| {
            let mut transform = Transform::from_translation(Vec3::new(0., 15., -15.));
            transform.look_at(Vec3::zero(), Vec3::unit_y());
            parent
                .spawn(PerspectiveCameraBundle {
                    transform,
                    ..Default::default()
                })
                .with(Camera);
        });
}

fn move_player(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let forward = transform.forward();
        let left = forward.cross(Vec3::unit_y());
        let delta = time.delta_seconds() * PLAYER_SPEED;
        if input.pressed(KeyCode::W) {
            transform.translation += forward * delta;
        }
        if input.pressed(KeyCode::S) {
            transform.translation -= forward * delta;
        }
        if input.pressed(KeyCode::A) {
            transform.translation -= left * delta;
        }
        if input.pressed(KeyCode::D) {
            transform.translation += left * delta;
        }
    }
}

fn rotate_player(
    mut query: Query<&mut Transform, With<Player>>,
    mut mouse_events: EventReader<MouseMotion>,
    window: Res<WindowDescriptor>,
) {
    for event in mouse_events.iter() {
        let rotation = Quat::from_rotation_y(-4. * event.delta.x / window.width);
        for mut transform in query.iter_mut() {
            transform.rotate(rotation);
        }
    }
}

pub struct OverworldPlugin;
impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_overworld.system())
            .add_system(move_player.system())
            .add_system(rotate_player.system());
    }
}
