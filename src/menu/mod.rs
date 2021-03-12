use bevy::{ecs::component::Component, prelude::*};

use crate::AppState;
use crate::Stage;
use crate::UiAssets;

pub mod main_menu;
pub mod settings;

/// Every logical action for which we can have a UI button
///
/// Use as marker components to identify the buttons.
pub mod button {
    pub struct EnterGame;
    pub struct ExitApp;
    pub struct OpenSettingsMenu;
    pub struct ExitSettingsMenu;
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(
            Stage::AppState,
            AppState::MainMenu,
            main_menu::setup.system(),
        )
        .on_state_update(
            Stage::AppState,
            AppState::MainMenu,
            button_interact::<button::ExitApp>
                .system()
                .chain(main_menu::button_exit_app.system()),
        )
        .on_state_update(
            Stage::AppState,
            AppState::MainMenu,
            button_interact::<button::EnterGame>
                .system()
                .chain(main_menu::button_enter_game.system()),
        )
        .on_state_update(
            Stage::AppState,
            AppState::MainMenu,
            button_interact::<button::OpenSettingsMenu>
                .system()
                .chain(main_menu::button_open_settings_menu.system()),
        )
        .on_state_exit(
            Stage::AppState,
            AppState::MainMenu,
            crate::despawn_all::<main_menu::StateCleanup>.system(),
        )
        // Settings menu
        .on_state_enter(
            Stage::AppState,
            AppState::SettingsMenu,
            settings::setup.system(),
        )
        .on_state_update(
            Stage::AppState,
            AppState::SettingsMenu,
            button_interact::<button::ExitSettingsMenu>
                .system()
                .chain(settings::button_exit_settings_menu.system()),
        )
        .on_state_exit(
            Stage::AppState,
            AppState::SettingsMenu,
            crate::despawn_all::<settings::StateCleanup>.system(),
        );
    }
}

pub fn button_interact<B: Component>(
    materials: Res<UiAssets>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Mutated<Interaction>, With<Button>, With<B>),
    >,
) -> bool {
    let mut clicked = false;

    for (interaction, mut material) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *material = materials.button_active.clone();
                clicked = true;
            }
            Interaction::Hovered => *material = materials.button_hover.clone(),
            Interaction::None => *material = materials.button_normal.clone(),
        }
    }

    clicked
}
