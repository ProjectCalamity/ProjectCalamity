use bevy::{prelude::*, utils::Uuid};

use crate::common::{networking::schema::Player, config::Config, logic::{GameLogicPlugin, gameboard_gen::generate_gameboard, PlayerTeam, TeamColour}};

use self::{graphical::GraphicalPlugin, ui::UIPlugin};

pub mod graphical;
pub mod ui;
pub mod networking;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_state::<ClientState>()
            .init_resource::<Spritesheet>()
            .add_plugin(GameLogicPlugin)
            .add_plugin(GraphicalPlugin)
            .add_plugin(UIPlugin)
            .add_startup_system(create_player)
            .add_startup_system(load_assets);
    }
}

pub struct SingleplayerPlugin;

impl Plugin for SingleplayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(test_spawn_players_temp)
            .add_system(generate_gameboard.in_set(OnUpdate(ClientState::Game)));
    }
}

fn test_spawn_players_temp(mut commands: Commands) {
    commands.spawn(Player { username: "Aurora".to_string(), id: Uuid::new_v4() }).insert(PlayerTeam(TeamColour::Red));
    commands.spawn(Player { username: "AurorAlt".to_string(), id: Uuid::new_v4() }).insert(PlayerTeam(TeamColour::Blue));
}

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientState {
    #[default]
    MainMenu,
    Game
}

fn create_player(mut commands: Commands, conf: Res<Config>) {
    commands.spawn(Player {
        username: conf.client_config.username.clone(),
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

    info!("Assets loaded");
}