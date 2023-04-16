use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Spritesheet {
    pub characters: Handle<TextureAtlas>,
    pub tiles: Handle<TextureAtlas>,
    pub tile_icons: Handle<TextureAtlas>,
    pub selector_icons: Handle<TextureAtlas>
}

#[derive(Resource, Default)]
pub struct Fonts {
    pub regular: Handle<Font>,
    pub italic: Handle<Font>,
    pub bold: Handle<Font>,
    pub bold_italic: Handle<Font>,
}

pub fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {

    let character_img_handle = asset_server.load("sprites/character_atlas.png");
    let chatacter_texture_atlas = TextureAtlas::from_grid(character_img_handle, Vec2::new(19f32, 23f32), 2, 1, Some(Vec2::new(1f32, 0f32)), None);
    let character_texture_atlas_handle = texture_atlases.add(chatacter_texture_atlas);

    let tile_img_handle = asset_server.load("sprites/tilemap_atlas.png");
    let tile_texture_atlas = TextureAtlas::from_grid(tile_img_handle, Vec2::new(32f32, 32f32), 4, 3, Some(Vec2::new(1f32, 1f32)), None);
    let tile_texture_atlas_handle = texture_atlases.add(tile_texture_atlas);

    let tile_icons_img_handle = asset_server.load("sprites/tile_icons_atlas.png");
    let tile_icons_texture_atlas = TextureAtlas::from_grid(tile_icons_img_handle, Vec2::new(24f32, 24f32), 4, 1, Some(Vec2::new(1f32, 1f32)), None);
    let tile_icons_texture_atlas_handle = texture_atlases.add(tile_icons_texture_atlas);

    let selector_icons_img_handle = asset_server.load("sprites/selector_icons_atlas.png");
    let selector_icons_texture_atlas = TextureAtlas::from_grid(selector_icons_img_handle, Vec2::new(32f32, 32f32), 3, 1, Some(Vec2::new(1f32, 1f32)), None);
    let selector_icons_texture_atlas_handle = texture_atlases.add(selector_icons_texture_atlas);

    commands.insert_resource(Spritesheet{ characters: character_texture_atlas_handle, tiles: tile_texture_atlas_handle, tile_icons: tile_icons_texture_atlas_handle, selector_icons: selector_icons_texture_atlas_handle});
}

pub fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let regular = asset_server.load("fonts/AtkinsonHyperlegible-Regular.ttf");
    let italic = asset_server.load("fonts/AtkinsonHyperlegible-Italic.ttf");
    let bold = asset_server.load("fonts/AtkinsonHyperlegible-Bold.ttf");
    let bold_italic = asset_server.load("fonts/AtkinsonHyperlegible-BoldItalic.ttf");

    commands.insert_resource(Fonts { regular: regular, italic: italic, bold: bold, bold_italic: bold_italic })
}