use bevy::prelude::*;

use crate::AppState;
use crate::UiAssets;

pub mod main_menu;
pub mod settings;

/// Every logical action for which we can have a UI button
///
/// Use as marker components to identify the buttons.
pub mod button {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct EnterGame;

    #[derive(Component)]
    pub struct ExitApp;

    #[derive(Component)]
    pub struct OpenSettingsMenu;

    #[derive(Component)]
    pub struct ExitSettingsMenu;
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(main_menu::setup))
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                    .with_system(
                        button_interact::<button::ExitApp>.chain(main_menu::button_exit_app),
                    )
                    .with_system(
                        button_interact::<button::EnterGame>.chain(main_menu::button_enter_game),
                    )
                    .with_system(
                        button_interact::<button::OpenSettingsMenu>
                            .chain(main_menu::button_open_settings_menu),
                    ),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu)
                    .with_system(crate::despawn_all::<main_menu::StateCleanup>),
            )
            // Settings menu
            .add_system_set(
                SystemSet::on_enter(AppState::SettingsMenu).with_system(settings::setup),
            )
            .add_system_set(
                SystemSet::on_update(AppState::SettingsMenu).with_system(
                    button_interact::<button::ExitSettingsMenu>
                        .chain(settings::button_exit_settings_menu),
                ),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::SettingsMenu)
                    .with_system(crate::despawn_all::<settings::StateCleanup>),
            );
    }
}

pub fn button_interact<B: Component>(
    assets: Res<UiAssets>,
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    let mut clicked = false;

    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = assets.button_active;
                clicked = true;
            }
            Interaction::Hovered => *color = assets.button_hover,
            Interaction::None => *color = assets.button_normal,
        }
    }

    clicked
}
