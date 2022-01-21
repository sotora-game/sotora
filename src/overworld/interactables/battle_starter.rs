use bevy::prelude::*;

use crate::AppState;

#[derive(Component, Clone)]
pub struct BattleStarter;

pub fn interactable_start_battle(
    In(interactable): In<Option<BattleStarter>>,
    mut state: ResMut<State<AppState>>,
) {
    if interactable.is_some() {
        state.set(AppState::Battle).unwrap();
    }
}
