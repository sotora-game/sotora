use bevy::prelude::*;

use crate::AppState;

#[derive(Clone)]
pub struct BattleStarter;

pub fn interactable_start_battle(
    In(interactable): In<Option<BattleStarter>>,
    mut state: ResMut<State<AppState>>,
) {
    if interactable.is_some() {
        state.set_next(AppState::Battle).unwrap();
    }
}
