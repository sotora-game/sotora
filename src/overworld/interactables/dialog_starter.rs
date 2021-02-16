use bevy::prelude::*;

// use crate::AppState;

// TODO: Add some fields about the dialog here
#[derive(Clone)]
pub struct DialogStarter {
    pub npc_name: String,
}

pub fn interactable_start_dialog(
    In(interactable): In<Option<DialogStarter>>,
    // mut state: ResMut<State<AppState>>,
) {
    if let Some(dialog) = interactable {
        println!("You want to talk to {}", dialog.npc_name);
        // TODO: Enable this once Dialog state exists
        // state.set_next(AppState::Dialog).unwrap();
    }
}
