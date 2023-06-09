use bevy::prelude::*;
use bevy_fast_tilemap::FastTileMapPlugin;

use crate::common::logic::{neo_gameboard::spawn_gameboard, GameLogicPlugin};

use self::{graphical::GraphicalPlugin, ui::UIPlugin};

pub mod graphical;
pub mod ui;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_state::<ClientState>()
            .init_resource::<Spritesheet>()
            .add_plugin(FastTileMapPlugin::default())
            .add_plugin(GameLogicPlugin)
            .add_plugin(GraphicalPlugin)
            .add_plugin(UIPlugin)
            .add_startup_system(load_assets);
    }
}

pub struct SingleplayerPlugin;

impl Plugin for SingleplayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_gameboard.in_set(OnUpdate(ClientState::Game)));
    }
}

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientState {
    #[default]
    MainMenu,
    Game,
}

#[derive(Resource, Default)]
pub struct Spritesheet {
    pub characters: Handle<TextureAtlas>,
    pub tiles: Handle<TextureAtlas>,
    pub tile_icons: Handle<TextureAtlas>,
    pub selector_icons: Handle<TextureAtlas>,
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let character_img_handle = asset_server.load("sprites/character_atlas.png");
    let chatacter_texture_atlas = TextureAtlas::from_grid(
        character_img_handle,
        Vec2::new(19f32, 23f32),
        2,
        1,
        Some(Vec2::new(1f32, 0f32)),
        None,
    );
    let character_texture_atlas_handle = texture_atlases.add(chatacter_texture_atlas);

    let tile_img_handle = asset_server.load("sprites/tilemap_atlas.png");
    let tile_texture_atlas = TextureAtlas::from_grid(
        tile_img_handle,
        Vec2::new(32f32, 32f32),
        4,
        4,
        Some(Vec2::new(1f32, 1f32)),
        None,
    );
    let tile_texture_atlas_handle = texture_atlases.add(tile_texture_atlas);

    let tile_icons_img_handle = asset_server.load("sprites/tile_icons_atlas.png");
    let tile_icons_texture_atlas = TextureAtlas::from_grid(
        tile_icons_img_handle,
        Vec2::new(24f32, 24f32),
        4,
        1,
        Some(Vec2::new(1f32, 1f32)),
        None,
    );
    let tile_icons_texture_atlas_handle = texture_atlases.add(tile_icons_texture_atlas);

    let selector_icons_img_handle = asset_server.load("sprites/selector_icons_atlas.png");
    let selector_icons_texture_atlas = TextureAtlas::from_grid(
        selector_icons_img_handle,
        Vec2::new(32f32, 32f32),
        3,
        1,
        Some(Vec2::new(1f32, 1f32)),
        None,
    );
    let selector_icons_texture_atlas_handle = texture_atlases.add(selector_icons_texture_atlas);

    commands.insert_resource(Spritesheet {
        characters: character_texture_atlas_handle,
        tiles: tile_texture_atlas_handle,
        tile_icons: tile_icons_texture_atlas_handle,
        selector_icons: selector_icons_texture_atlas_handle,
    });

    info!("Assets loaded");
}
