use bevy::prelude::*;

use crate::AppState;
use crate::APPSTATES;

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
        app.init_resource::<MenuAssets>()
            .on_state_enter(APPSTATES, AppState::MainMenu, main_menu::setup.system())
            .on_state_update(
                APPSTATES,
                AppState::MainMenu,
                button_interact::<button::ExitApp>
                    .system()
                    .chain(main_menu::button_exit_app.system()),
            )
            .on_state_update(
                APPSTATES,
                AppState::MainMenu,
                button_interact::<button::EnterGame>
                    .system()
                    .chain(main_menu::button_enter_game.system()),
            )
            .on_state_update(
                APPSTATES,
                AppState::MainMenu,
                button_interact::<button::OpenSettingsMenu>
                    .system()
                    .chain(main_menu::button_open_settings_menu.system()),
            )
            .on_state_exit(
                APPSTATES,
                AppState::MainMenu,
                crate::despawn_all::<main_menu::StateCleanup>.system(),
            )
            // Settings menu
            .on_state_enter(APPSTATES, AppState::SettingsMenu, settings::setup.system())
            .on_state_update(
                APPSTATES,
                AppState::SettingsMenu,
                button_interact::<button::ExitSettingsMenu>
                    .system()
                    .chain(settings::button_exit_settings_menu.system()),
            )
            .on_state_exit(
                APPSTATES,
                AppState::SettingsMenu,
                crate::despawn_all::<settings::StateCleanup>.system(),
            );
    }
}

pub struct MenuAssets {
    button_normal: Handle<ColorMaterial>,
    button_hover: Handle<ColorMaterial>,
    button_active: Handle<ColorMaterial>,

    menu_panel_background: Handle<ColorMaterial>,

    transparent: Handle<ColorMaterial>,

    font_light: Handle<Font>,
    font_light_italic: Handle<Font>,
    font_regular: Handle<Font>,
    font_regular_italic: Handle<Font>,
    font_bold: Handle<Font>,
    font_bold_italic: Handle<Font>,
}

impl FromResources for MenuAssets {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let assets = resources.get_mut::<AssetServer>().unwrap();

        MenuAssets {
            button_normal: materials.add(Color::rgb(0.3, 0.3, 0.36).into()),
            button_hover: materials.add(Color::rgb(0.4, 0.4, 0.46).into()),
            button_active: materials.add(Color::rgb(0.24, 0.24, 0.32).into()),

            menu_panel_background: materials.add(Color::rgb(0.2, 0.2, 0.24).into()),

            transparent: materials.add(Color::NONE.into()),

            font_light: assets.load("fonts/sansation/Sansation-Light.ttf"),
            font_light_italic: assets.load("fonts/sansation/Sansation-LightItalic.ttf"),
            font_regular: assets.load("fonts/sansation/Sansation-Regular.ttf"),
            font_regular_italic: assets.load("fonts/sansation/Sansation-Italic.ttf"),
            font_bold: assets.load("fonts/sansation/Sansation-Bold.ttf"),
            font_bold_italic: assets.load("fonts/sansation/Sansation-BoldItalic.ttf"),
        }
    }
}

pub fn button_interact<B: Component>(
    materials: Res<MenuAssets>,
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
