use bevy::app::AppExit;
use bevy::prelude::*;

use crate::AppState;

pub mod main_menu;
pub mod settings;

/// Marker for despawning when exiting `AppState::MainMenu`
pub struct StateCleanup;

pub struct MenuMaterials {
    button_normal: Handle<ColorMaterial>,
    button_hover: Handle<ColorMaterial>,
    button_active: Handle<ColorMaterial>,

    menu_panel_background: Handle<ColorMaterial>,

    transparent: Handle<ColorMaterial>,
}

impl FromResources for MenuMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        MenuMaterials {
            button_normal: materials.add(Color::rgb(0.3, 0.3, 0.36).into()),
            button_hover: materials.add(Color::rgb(0.4, 0.4, 0.46).into()),
            button_active: materials.add(Color::rgb(0.24, 0.24, 0.32).into()),

            menu_panel_background: materials.add(Color::rgb(0.2, 0.2, 0.24).into()),

            transparent: materials.add(Color::NONE.into()),
        }
    }
}

pub enum ClickAction {
    ChangeState(AppState),
    Exit,
}

pub fn button_interact(
    mut state: ResMut<State<AppState>>,
    mut app_exit: ResMut<Events<AppExit>>,
    materials: Res<MenuMaterials>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &ClickAction),
        (Mutated<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material, action) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *material = materials.button_active.clone();

                match action {
                    ClickAction::ChangeState(next) => state.set_next(*next).unwrap(),
                    ClickAction::Exit => app_exit.send(AppExit),
                }
            }
            Interaction::Hovered => *material = materials.button_hover.clone(),
            Interaction::None => *material = materials.button_normal.clone(),
        }
    }
}
