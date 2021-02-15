use bevy::prelude::*;

use super::player::Player;
use crate::AppState;

pub struct Interactable;

pub fn interact_with_interactables(
    interactables: Query<&Transform, With<Interactable>>,
    player: Query<&Transform, With<Player>>,
    mut state: ResMut<State<AppState>>,
) {
    // NOTE: O(n^2), should be changed for a smarter implementation
    for p_transform in player.iter() {
        for i_transform in interactables.iter() {
            let distance = p_transform.translation - i_transform.translation;
            if distance.length() < 1.0 {
                state.set_next(AppState::Battle).unwrap();
            }
        }
    }
}
