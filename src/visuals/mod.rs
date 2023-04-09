mod sprites;

use bevy::prelude::*;
use rand::Rng;

use crate::game::{*, units::UnitID};

use self::sprites::{load_sprites, Spritesheet};

pub struct VisualPlaygroundPlugin;

impl Plugin for VisualPlaygroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CameraScalingInfo>()
            .init_resource::<Spritesheet>()
            .add_startup_system(spawn_camera)
            .add_system(render_terrain)
            .add_system(render_units)
            .add_system(update_transforms)
            .add_systems((conform_transforms_tiles, conform_transforms_units).after(update_transforms))
            .add_system(load_sprites);
    }
}

#[derive(Component, Reflect)]
struct CameraScalingInfo {
    x_scl: f32,
    y_scl: f32,
    unit_scl: f32,
    unit_delta: f32,
}

#[derive(Component)]
struct RenderedTerrain;

#[derive(Component)]
struct RenderedFeature;

#[derive(Component)]
struct RenderedUnit;

#[derive(Component)]
struct RenderedIcon;

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

// Step 3: Units
fn render_units(
    mut commands: Commands,
    mut rendered_units: Query<(&Unit, &mut TextureAtlasSprite, With<RenderedUnit>)>,
    nonrendered_units: Query<(Entity, &Unit, Without<RenderedUnit>)>,
    spritesheet: Res<Spritesheet>
) {

    nonrendered_units.iter().for_each(|(e, u, ())| {

        println!("RENDERING UNIT :{:?}", u);
        let mut bundle = SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(unit_sprite_index_from_id(&u.id)),
            texture_atlas: spritesheet.characters.clone(),
            ..default()
        };

        bundle.transform.scale.x *= 0.7;
        bundle.transform.scale.y *= 0.7;
        bundle.transform.translation.z = 20f32;

        commands.entity(e).insert(bundle).insert(Scalable).insert(RenderedUnit);
    });

    rendered_units.iter_mut().for_each(|(u, mut a, ())| {
        if a.index != unit_sprite_index_from_id(&u.id) {
            a.index = unit_sprite_index_from_id(&u.id);
        }
    });
}

fn unit_sprite_index_from_id(unit_id: &UnitID) -> usize {
    return match unit_id {
        UnitID::ScienceGenericTest => 0,
        UnitID::MagicGenericTest => 1,
    };
}

fn id_from_unit_sprite_index(index: usize) -> UnitID {
    return match index {
        0 => UnitID::ScienceGenericTest,
        1 => UnitID::ScienceGenericTest,
        _ => UnitID::ScienceGenericTest,
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

    tiles.iter_mut().for_each(|(e, ti, mut t, ())| {
        t.translation.x = (scl.unit_scl / 32f32) * (ti.pos[0] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.translation.y = (scl.unit_scl / 32f32) * (ti.pos[1] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.scale.x *= scl.unit_delta;
        t.scale.y *= scl.unit_delta;
    });
}

fn conform_transforms_units(
    cam: Query<(&OrthographicProjection, &CameraScalingInfo)>, 
    mut units: Query<(Entity, &Unit, &mut Transform, With<Scalable>)>
) {

    let scl = cam.single().1;

    units.iter_mut().for_each(|(e, u, mut t, ())| {
        t.translation.x = (scl.unit_scl / 32f32) * (u.pos[0] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.translation.y = (scl.unit_scl / 32f32) * (u.pos[1] - (32 / 2)) as f32; // [gameboard_width / 2]
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
    });
}