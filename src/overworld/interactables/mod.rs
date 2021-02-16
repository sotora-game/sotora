use bevy::prelude::*;

use crate::overworld::player::Player;
use crate::user_config::KeyBinds;

pub mod battle_starter;
pub mod dialog_starter;

pub fn interactable_interact<B: Component + Clone>(
    player: Query<&Transform, With<Player>>,
    interactables: Query<(&Transform, &B)>,
    input: Res<Input<KeyCode>>,
    keybinds: Res<KeyBinds>,
) -> Option<B> {
    if !input.just_pressed(keybinds.interact) {
        return None;
    }

    // TODO: O(p*i), should be changed for a smarter implementation
    for p_transform in player.iter() {
        for (i_transform, interactable) in interactables.iter() {
            let distance = p_transform.translation - i_transform.translation;

            if distance.length() < 1.0 {
                return Some(interactable.clone());
            }
        }
    }

    None
}
