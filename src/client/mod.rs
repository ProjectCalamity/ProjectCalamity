use bevy::{prelude::*, utils::Uuid};
use kayak_ui::prelude::kayak_font::KayakFont;

use crate::common::{networking::schema::Player, config::Config, logic::GameLogicPlugin};

use self::{menus::MenusPlugin, graphical::GraphicalPlugin, networking::ClientNetworkPlugin};

pub mod graphical;
pub mod menus;
pub mod networking;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_state::<ClientState>()
            .init_resource::<Spritesheet>()
            .init_resource::<Fonts>()
            .add_plugin(MenusPlugin)
            .add_plugin(GameLogicPlugin)
            .add_plugin(GraphicalPlugin)
            .add_plugin(ClientNetworkPlugin)
            .add_startup_system(create_player)
            .add_startup_system(load_assets);
    }
}

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientState {
    #[default]
    MainMenu,
    Game
}

fn create_player(mut commands: Commands, conf: Res<Config>) {
    commands.spawn(Player {
        username: conf.client_config.as_ref().unwrap().username.clone(),
        id: Uuid::new_v4(),
    });
}

#[derive(Resource, Default)]
pub struct Spritesheet {
    pub characters: Handle<TextureAtlas>,
    pub tiles: Handle<TextureAtlas>,
    pub tile_icons: Handle<TextureAtlas>,
    pub selector_icons: Handle<TextureAtlas>
}

#[derive(Resource, Default)]
pub struct Fonts {
    pub regular: Handle<KayakFont>,
    pub bold: Handle<KayakFont>,
}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {

    let character_img_handle = asset_server.load("sprites/character_atlas.png");
    let chatacter_texture_atlas = TextureAtlas::from_grid(character_img_handle, Vec2::new(19f32, 23f32), 2, 1, Some(Vec2::new(1f32, 0f32)), None);
    let character_texture_atlas_handle = texture_atlases.add(chatacter_texture_atlas);

    let tile_img_handle = asset_server.load("sprites/tilemap_atlas.png");
    let tile_texture_atlas = TextureAtlas::from_grid(tile_img_handle, Vec2::new(32f32, 32f32), 4, 4, Some(Vec2::new(1f32, 1f32)), None);
    let tile_texture_atlas_handle = texture_atlases.add(tile_texture_atlas);

    let tile_icons_img_handle = asset_server.load("sprites/tile_icons_atlas.png");
    let tile_icons_texture_atlas = TextureAtlas::from_grid(tile_icons_img_handle, Vec2::new(24f32, 24f32), 4, 1, Some(Vec2::new(1f32, 1f32)), None);
    let tile_icons_texture_atlas_handle = texture_atlases.add(tile_icons_texture_atlas);

    let selector_icons_img_handle = asset_server.load("sprites/selector_icons_atlas.png");
    let selector_icons_texture_atlas = TextureAtlas::from_grid(selector_icons_img_handle, Vec2::new(32f32, 32f32), 3, 1, Some(Vec2::new(1f32, 1f32)), None);
    let selector_icons_texture_atlas_handle = texture_atlases.add(selector_icons_texture_atlas);

    commands.insert_resource(Spritesheet{ characters: character_texture_atlas_handle, tiles: tile_texture_atlas_handle, tile_icons: tile_icons_texture_atlas_handle, selector_icons: selector_icons_texture_atlas_handle});

    let regular = asset_server.load("fonts/atkinson_hyperlegible_regular.kayak_font");
    let bold = asset_server.load("fonts/atkinson_hyperlegible_bold.kayak_font");

    commands.insert_resource(Fonts { regular: regular, bold: bold})
}