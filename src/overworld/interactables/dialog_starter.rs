use bevy::prelude::*;

use crate::dialog::DialogResource;
use crate::AppState;

#[derive(Clone)]
pub struct DialogStarter {
    pub npc_name: String,
    pub sprite: Handle<ColorMaterial>,
}

pub fn interactable_start_dialog(
    In(interactable): In<Option<DialogStarter>>,
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
) {
    if let Some(dialog) = interactable {
        commands.insert_resource(DialogResource {
            npc_name: dialog.npc_name,
            sprite: dialog.sprite,
        });

        state.set_next(AppState::Dialog).unwrap();
    }
}
