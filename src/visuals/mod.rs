mod sprites;
mod inputs;

use bevy::prelude::*;
use rand::Rng;

use crate::game::{*, units::UnitID};

use self::{sprites::{load_sprites, Spritesheet}, inputs::{ZoomEvent, scroll_events, zoom_camera, PanEvent, mouse_pan_events, scroll_camera, mouse_click_events, GridPosClickEvent, select_unit, GridBounds}};

pub struct VisualPlaygroundPlugin;

impl Plugin for VisualPlaygroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CameraScalingInfo>()
            .init_resource::<Spritesheet>()
            .add_event::<PanEvent>()
            .add_event::<ZoomEvent>()
            .add_event::<GridPosClickEvent>()
            .add_startup_system(load_sprites)
            .add_startup_system(spawn_camera)
            .add_system(render_terrain)
            .add_system(render_features)
            .add_system(render_units)
            .add_system(render_icons)
            .add_system(update_transforms)
            .add_systems((conform_transforms_tiles, conform_transforms_units, conform_transforms_features, conform_transforms_icons).after(update_transforms))
            .add_system(scroll_events)
            .add_system(select_unit)
            .add_system(zoom_camera)
            .add_system(mouse_click_events)
            .add_system(mouse_pan_events)
            .add_system(scroll_camera);
    }
}

#[derive(Component, Reflect)]
pub struct CameraScalingInfo {
    x_scl: f32,
    y_scl: f32,
    unit_scl: f32,
    unit_delta: f32,
}

#[derive(Component, FromReflect, Reflect)]
pub struct Icon {
    icon: Icons,
    pos: [i32; 2]
}

#[derive(FromReflect, PartialEq, Reflect)]
pub enum Icons {
    Circle,
    Cross,
    Selector
}

#[derive(Component)]
struct RenderedTerrain;

#[derive(Component)]
struct RenderedFeature;

#[derive(Component)]
struct RenderedUnit;

#[derive(Component)]
pub struct RenderedIcon;

/*
    Rendering is performed in five "passes":
    
    1. Terrain z = 0
    2. Features z = 10
    3. Units z = 20
    4. Icons z = 30
    5. UI z = 50

    Each has a seperate system.
*/

// Step 1: Terrain
fn render_terrain(
    mut commands: Commands,
    mut rendered_tiles: Query<(&TileInfo, &mut TextureAtlasSprite, With<RenderedTerrain>)>,
    nonrendered_tiles: Query<(Entity, &TileInfo, Without<Handle<TextureAtlas>>)>,
    spritesheet: Res<Spritesheet>
) {
    nonrendered_tiles.iter().for_each(|(e, t, ())| {

        let mut bundle = SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(texture_index_from_geography(&t.geography)),
            texture_atlas: spritesheet.tiles.clone(),
            ..default()
        };

        bundle.transform.scale.x *= 0.7;
        bundle.transform.scale.y *= 0.7;
        bundle.transform.translation.z = 0f32;

        commands.entity(e).insert(bundle).insert(Scalable).insert(RenderedTerrain);
    });

    rendered_tiles.iter_mut().for_each(|(t, mut a, ())| {
        if t.geography != geography_from_texture_index(a.index) {
            a.index = texture_index_from_geography(&t.geography);
        }
    });
}

fn geography_from_texture_index(index: usize) -> Geography {
    return match index {
        0..=3 => Geography::None,
        8..=11 => Geography::Water,
        4..=7 => Geography::Mountains,
        _ => Geography::None
    }
}

fn texture_index_from_geography(geo: &Geography) -> usize {
    let mut rng = rand::thread_rng();
    return match geo {
        Geography::None => rng.gen_range(0..3),
        Geography::Water => rng.gen_range(8..11),
        Geography::Mountains => rng.gen_range(4..7),
    };
}

// Step 2: Features
fn render_features(
    mut commands: Commands,
    mut rendered_features: Query<(&TileFeature, &mut TextureAtlasSprite, With<RenderedFeature>)>,
    nonrendered_features: Query<(Entity, &TileFeature, Without<Handle<TextureAtlas>>)>,
    spritesheet: Res<Spritesheet>
) {
    nonrendered_features.iter().for_each(|(e, tf, ())| {
        let mut bundle = SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(texture_index_from_tile_feature(&tf.feature)),
            texture_atlas: spritesheet.tile_icons.clone(),
            ..default()
        };

        bundle.transform.scale.x *= 0.7;
        bundle.transform.scale.y *= 0.7;
        bundle.transform.translation.z = 10f32;

        commands.entity(e).insert(bundle).insert(Scalable).insert(RenderedTerrain);
    });

    rendered_features.iter_mut().for_each(|(tf, mut a, ())| {
        if tf.feature != tile_feature_from_texture_index(a.index) {
            a.index = texture_index_from_tile_feature(&tf.feature);
        }
    });
}

fn tile_feature_from_texture_index(index: usize) -> TileFeatures {
    return match index {
        0 => TileFeatures::CurrencySite(Archetype(Archetypes::Magic)),
        1 => TileFeatures::CurrencySite(Archetype(Archetypes::Science)),
        3 => TileFeatures::Nest(PlayerTeam(TeamColour::Red)),
        _ => TileFeatures::Nest(PlayerTeam(TeamColour::Blue))
    }
}

fn texture_index_from_tile_feature(tf: &TileFeatures) -> usize {
    return match tf {
        TileFeatures::CurrencySite(Archetype(Archetypes::Magic)) => 0,
        TileFeatures::CurrencySite(Archetype(Archetypes::Science)) => 1,
        TileFeatures::Nest(PlayerTeam(TeamColour::Red)) => 3,
        _ => 4,
    };
}

// Step 3: Units
fn render_units(
    mut commands: Commands,
    mut rendered_units: Query<(&Unit, &mut TextureAtlasSprite, With<RenderedUnit>)>,
    nonrendered_units: Query<(Entity, &Unit, Without<Handle<TextureAtlas>>)>,
    spritesheet: Res<Spritesheet>
) {
    nonrendered_units.iter().for_each(|(e, u, ())| {
        let mut bundle = SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(texture_index_from_unit_id(&u.id)),
            texture_atlas: spritesheet.characters.clone(),
            ..default()
        };

        bundle.transform.scale.x *= 0.7;
        bundle.transform.scale.y *= 0.7;
        bundle.transform.translation.z = 20f32;

        commands.entity(e).insert(bundle).insert(Scalable).insert(RenderedTerrain);
    });

    rendered_units.iter_mut().for_each(|(u, mut a, ())| {
        if u.id != unit_id_from_texture_index(a.index) {
            a.index = texture_index_from_unit_id(&u.id);
        }
    });
}

fn unit_id_from_texture_index(index: usize) -> UnitID {
    return match index {
        0 => UnitID::ScienceGenericTest,
        1 => UnitID::MagicGenericTest,
        _ => UnitID::ScienceGenericTest,
    }
}

fn texture_index_from_unit_id(uid: &UnitID) -> usize {
    return match uid {
        UnitID::ScienceGenericTest => 0,
        UnitID::MagicGenericTest => 1,
    };
}

// Step 4: Icons
fn render_icons(
    mut commands: Commands,
    mut rendered_icons: Query<(&Icon, &mut TextureAtlasSprite, With<RenderedIcon>)>,
    nonrendered_icons: Query<(Entity, &Icon, Without<Handle<TextureAtlas>>)>,
    spritesheet: Res<Spritesheet>
) {
    nonrendered_icons.iter().for_each(|(e, i, ())| {
        let mut bundle = SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(texture_index_from_icon(&i.icon)),
            texture_atlas: spritesheet.selector_icons.clone(),
            ..default()
        };

        bundle.transform.scale.x *= 0.7;
        bundle.transform.scale.y *= 0.7;
        bundle.transform.translation.z = 30f32;

        commands.entity(e).insert(bundle).insert(Scalable).insert(RenderedIcon);
    });

    rendered_icons.iter_mut().for_each(|(i, mut a, ())| {
        if i.icon != icon_from_texture_index(a.index) {
            a.index = texture_index_from_icon(&i.icon);
        }
    });
}

fn icon_from_texture_index(index: usize) -> Icons {
    return match index {
        0 => Icons::Selector,
        1 => Icons::Circle,
        2 => Icons::Cross,
        _ => Icons::Selector
    }
}

fn texture_index_from_icon(i: &Icons) -> usize {
    return match i {
        Icons::Selector => 0,
        Icons::Circle => 1,
        Icons::Cross => 2,
    };
}

fn update_transforms(mut cam: Query<(&OrthographicProjection, &mut CameraScalingInfo)>, board_info: Query<&Gameboard>) {
    if !cam.is_empty() && !board_info.is_empty() {

        let orth = cam.single().0;

        let x_scl = orth.area.max.x - orth.area.min.x;
        let y_scl = orth.area.max.y - orth.area.min.y;
        
        let mut scl = cam.single_mut().1;
        
        let unit_delta = f32::min(x_scl, y_scl) / scl.unit_scl;
        
        scl.x_scl = x_scl;
        scl.y_scl = y_scl;
        scl.unit_scl = f32::min(x_scl, y_scl);
        scl.unit_delta = unit_delta;
        
    }
}



fn conform_transforms_tiles(
    cam: Query<(&OrthographicProjection, &CameraScalingInfo)>, 
    mut tiles: Query<(Entity, &TileInfo, &mut Transform, With<Scalable>)>,
) {

    let scl = cam.single().1;

    tiles.iter_mut().for_each(|(_e, ti, mut t, ())| {
        t.translation.x = (scl.unit_scl / 32f32) * (ti.pos[0] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.translation.y = (scl.unit_scl / 32f32) * (ti.pos[1] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.scale.x *= scl.unit_delta;
        t.scale.y *= scl.unit_delta;
    });
}

fn conform_transforms_features(
    cam: Query<(&OrthographicProjection, &CameraScalingInfo)>, 
    mut features: Query<(Entity, &TileFeature, &mut Transform, With<Scalable>)>,
) {
    let scl = cam.single().1;

    features.iter_mut().for_each(|(_e, tf, mut t, ())| {
        t.translation.x = (scl.unit_scl / 32f32) * (tf.pos[0] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.translation.y = (scl.unit_scl / 32f32) * (tf.pos[1] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.scale.x *= scl.unit_delta;
        t.scale.y *= scl.unit_delta;
    });
}

fn conform_transforms_units(
    cam: Query<(&OrthographicProjection, &CameraScalingInfo)>, 
    mut units: Query<(Entity, &Unit, &mut Transform, With<Scalable>)>
) {

    let scl = cam.single().1;

    if scl.unit_delta > 2f32 {
        return;
    }

    units.iter_mut().for_each(|(_e, u, mut t, ())| {
        t.translation.x = (scl.unit_scl / 32f32) * (u.pos[0] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.translation.y = (scl.unit_scl / 32f32) * (u.pos[1] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.scale.x *= scl.unit_delta;
        t.scale.y *= scl.unit_delta;
    });
}

fn conform_transforms_icons(
    cam: Query<(&OrthographicProjection, &CameraScalingInfo)>, 
    mut units: Query<(Entity, &Icon, &mut Transform, With<Scalable>)>
) {

    let scl = cam.single().1;

    if scl.unit_delta > 2f32 {
        return;
    }

    units.iter_mut().for_each(|(_e, i, mut t, ())| {
        t.translation.x = (scl.unit_scl / 32f32) * (i.pos[0] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.translation.y = (scl.unit_scl / 32f32) * (i.pos[1] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.scale.x *= scl.unit_delta;
        t.scale.y *= scl.unit_delta;
    });
}

#[derive(Component, Reflect)]
struct Scalable;

fn spawn_camera(mut commands: Commands) {
    let cam = Camera2dBundle::default();
    let x_scl = cam.projection.area.max.x - cam.projection.area.min.x;
    let y_scl = cam.projection.area.max.y - cam.projection.area.min.y;
    let unit_scl = f32::min(x_scl, y_scl);

    commands.spawn(cam).insert(CameraScalingInfo {
        x_scl: x_scl,
        y_scl: y_scl,
        unit_scl: unit_scl,
        unit_delta: 1f32,
    }).insert(GridBounds {
        position: Vec2::splat(0f32),
        size: Vec2::splat(unit_scl),
    });

    // TEMPORARILY spawn unit
    // TODO: Remove this

    commands.spawn(UnitBundle {
        unit: Unit { id: UnitID::ScienceGenericTest, pos: [2, 2], health: Health(3f32), attack: Attack { base: 2f32, range: 1, splash: false, splash_multiplier: 1f32, magic_multiplier: 1f32, science_multiplier: 1f32 }, defense: Defense { base: 3f32, magic_multiplier: 1f32, science_multiplier: 1f32 }, movement: Movement(1), turn_execute_stage: TurnExecuteStage(TurnExecuteStages::MidTurn), archetype: Archetype(Archetypes::Science) },
    });

    commands.spawn(UnitBundle {
        unit: Unit { id: UnitID::MagicGenericTest, pos: [4, 2], health: Health(3f32), attack: Attack { base: 2f32, range: 1, splash: false, splash_multiplier: 1f32, magic_multiplier: 1f32, science_multiplier: 1f32 }, defense: Defense { base: 3f32, magic_multiplier: 1f32, science_multiplier: 1f32 }, movement: Movement(1), turn_execute_stage: TurnExecuteStage(TurnExecuteStages::MidTurn), archetype: Archetype(Archetypes::Science) },
    });
}