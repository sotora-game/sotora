use bevy::prelude::*;

use crate::user_config::KeyBinds;

pub struct Player {
    pub speed: f32,
}

pub fn move_player(
    input: Res<Input<KeyCode>>,
    keybinds: Res<KeyBinds>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, player) in query.iter_mut() {
        let forward = transform.forward();
        let left = forward.cross(Vec3::unit_y());
        let delta = time.delta_seconds() * player.speed;

        if input.pressed(keybinds.move_forward) {
            transform.translation += forward * delta;
        }
        if input.pressed(keybinds.move_backward) {
            transform.translation -= forward * delta;
        }
        if input.pressed(keybinds.move_left) {
            transform.translation -= left * delta;
        }
        if input.pressed(keybinds.move_right) {
            transform.translation += left * delta;
        }
    }
}
