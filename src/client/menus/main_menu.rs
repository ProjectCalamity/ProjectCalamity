use bevy::prelude::*;

use crate::client::{graphical::load_assets::Fonts, ClientState};

use super::ProjectCalamityStyle;

#[derive(Component, FromReflect, Reflect)]
pub struct MainMenu;

#[derive(Component, FromReflect, Reflect)]
pub struct ProjectCalamityWordMark;

#[derive(Component, FromReflect, Reflect)]
pub struct MainMenuBackground;

#[derive(Component, FromReflect, Reflect)]
pub struct SingeplayerButton;

#[derive(Component, FromReflect, Reflect)]
pub struct MultiplayerButton;

#[derive(Component, FromReflect, Reflect)]
pub struct SettingsButton;

#[derive(Component, FromReflect, Reflect)]
pub struct QuitButton;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: Res<Fonts>
) {
    build_main_menu(&mut commands, &asset_server, fonts);
}

pub fn scale_main_menu_background(
    mut background: Query<(&mut Style, With<MainMenuBackground>)>,
    window: Query<&Window>
) {
    background.iter_mut().for_each(|(st, ())| {
        let mut size = st.size;
        let window = window.single();
        let max_dim = Val::Px(f32::max(window.width(), window.height()));
        if size.width != max_dim || size.height != max_dim {
            size.width = max_dim;
            size.height = max_dim;
        }
    });
}

pub fn button_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &Children), (Changed<Interaction>, With<Button>)>,
    mut client_state: ResMut<NextState<ClientState>>,
    text_query: Query<&Text>,
) {
    for (int, mut col, chi) in &mut interaction_query {
        let text = text_query.get(chi[0]).unwrap();
        match int {
            Interaction::Clicked => {
                match text.sections[0].value.as_str() {
                    "SINGLEPLAYER" => {
                        client_state.0 = Some(ClientState::Game);
                    },
                    _ => {}
                }
            },
            Interaction::Hovered => { col.0 = ProjectCalamityStyle::BUTTON_HOVER_COLOUR.into() },
            Interaction::None => { col.0 = ProjectCalamityStyle::BUTTON_COLOUR.into() },
        }
    }
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu: Query<(Entity, &MainMenu)>
) {
    main_menu.iter().for_each(|(e, _)| commands.entity(e).despawn_recursive());
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    fonts: Res<Fonts>
) -> Entity {
    return commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(100f32), Val::Percent(100f32)),
            gap: Size::new(Val::Percent(2f32), Val::Percent(2f32)),
            ..default()
        },
        background_color: ProjectCalamityStyle::BACKGROUND_COLOUR.into(),
        ..default()
    }).insert(MainMenu)
    .insert(Name::new("Main Menu UI"))
    .with_children(|parent| {
        parent.spawn(ImageBundle {
            image: UiImage::new(asset_server.load("images/project_calamity_title_image.png")),
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Px(1024f32), Val::Px(1024f32)),
                ..default()
            },
            background_color: ProjectCalamityStyle::BACKGROUND_COLOUR.into(),
            ..default()
        }).insert(Name::new("Background Image"))
        .insert(MainMenuBackground);
    }).with_children(|parent| {
        parent.spawn(
            ImageBundle {
                image: UiImage::new(asset_server.load("images/project_calamity_wordmark.png")),
                style: Style { 
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(55f32), Val::Percent(30f32)), 
                    ..default()
                },
                ..default()
        }).insert(Name::new("Title Wordmark"));

        // Singleplayer
        let button = ProjectCalamityStyle::generate_generic_button(&fonts, "SINGLEPLAYER");
        parent.spawn(button.button).with_children(|p| { p.spawn(button.text); })
            .insert(Name::new("Singleplayer Button"))
            .insert(SingeplayerButton);

        // Multiplayer
        let button = ProjectCalamityStyle::generate_generic_button(&fonts, "MULTIPLAYER");
        parent.spawn(button.button).with_children(|p| { p.spawn(button.text); })
            .insert(Name::new("Multiplayer Button"))
            .insert(MultiplayerButton);

        // Settings
        let button = ProjectCalamityStyle::generate_generic_button(&fonts, "SETTINGS");
        parent.spawn(button.button).with_children(|p| { p.spawn(button.text); })
            .insert(Name::new("Settings Button"))
            .insert(SettingsButton);

        // Quit
        let button = ProjectCalamityStyle::generate_generic_button(&fonts, "QUIT");
        parent.spawn(button.button).with_children(|p| { p.spawn(button.text); })
            .insert(Name::new("Quit Button"))
            .insert(QuitButton);
    }).id();
}