use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct Camera;

pub fn rotate_camera(
    mut query: Query<&mut Transform, With<Camera>>,
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
