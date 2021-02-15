use bevy::prelude::*;

pub struct Player {
    pub speed: f32,
}

pub fn move_player(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, player) in query.iter_mut() {
        let forward = transform.forward();
        let left = forward.cross(Vec3::unit_y());
        let delta = time.delta_seconds() * player.speed;
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
