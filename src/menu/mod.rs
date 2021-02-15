use bevy::app::AppExit;
use bevy::prelude::*;

use crate::AppState;

pub mod main_menu;
pub mod settings;

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

pub enum ClickAction {
    ChangeState(AppState),
    Exit,
}

pub fn button_interact(
    mut state: ResMut<State<AppState>>,
    mut app_exit: ResMut<Events<AppExit>>,
    materials: Res<MenuAssets>,
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
