use bevy::prelude::*;

use self::main_menu::{spawn_main_menu, despawn_main_menu, scale_main_menu_background, button_interaction};


use super::{ClientState, graphical::load_assets::Fonts};

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_main_menu.in_schedule(OnEnter(ClientState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(ClientState::MainMenu)))
            .add_system(scale_main_menu_background.in_set(OnUpdate(ClientState::MainMenu)))
            .add_system(button_interaction.in_set(OnUpdate(ClientState::MainMenu)));
    }
}

pub struct ProjectCalamityStyle;

pub struct SpawnableButton {
    button: ButtonBundle,
    text: TextBundle,
}

impl ProjectCalamityStyle {
    pub const BACKGROUND_COLOUR: Color = Color::Rgba { red: 0.3f32, green: 0.3f32, blue: 0.3f32 as f32, alpha: 0.9f32 };
    pub const BUTTON_COLOUR: Color = Color::Rgba { red: 0f32, green: 0f32, blue: 0f32, alpha: 0.5f32 };
    pub const BUTTON_HOVER_COLOUR: Color = Color::Rgba { red: 0.2f32, green: 0.2f32, blue: 0.2f32, alpha: 0.3f32 };

    pub const BUTTON_STYLE: Style = Style { 
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        size: Size::new(Val::Percent(30f32), Val::Percent(10f32)), ..Style::DEFAULT 
    };

    fn generate_generic_button(fonts: &Res<Fonts>, text: &str) -> SpawnableButton {

        return SpawnableButton {
            button: ButtonBundle {
                style: ProjectCalamityStyle::BUTTON_STYLE,
                background_color: ProjectCalamityStyle::BUTTON_COLOUR.into(),
                ..default()
            },
            text: TextBundle {
                text: Text { 
                    sections: vec![
                        TextSection::new(text, 
                        TextStyle { font: fonts.bold.clone(), font_size: 42f32, color: Color::WHITE })
                    ], 
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            }
        };
    }
}

mod main_menu;