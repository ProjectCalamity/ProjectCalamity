pub mod inputs;

use bevy::prelude::*;
use bevy_fast_tilemap::{Map, MapBundle, MeshManagedByMap};

use crate::common::{logic::*, config::Config};

use self::inputs::{ZoomEvent, scroll_events, zoom_camera, PanEvent, mouse_pan_events, scroll_camera, mouse_click_events, GridPosClickEvent, select_unit, TurnCompletedEvent, keyboard_input};

use super::{ClientState};

pub struct GraphicalPlugin;

impl Plugin for GraphicalPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<GameCameraScalingInfo>()
            .add_event::<GridPosClickEvent>()
            .add_event::<PanEvent>()
            .add_event::<TurnCompletedEvent>()
            .add_event::<ZoomEvent>()
            .add_startup_system(spawn_gameboard)
            .add_system(setup_scaling.in_schedule(OnEnter(ClientState::Game)))
            .add_systems(
                (
                    conform_transforms_tiles, 
                    conform_transforms_units, 
                    conform_transforms_features, 
                    conform_transforms_icons,
                    conform_transforms_unit_action_ghosts
                )
                .after(update_transforms)
                .in_set(OnUpdate(ClientState::Game))
            )
            .add_system(scroll_events.in_set(OnUpdate(ClientState::Game)))
            .add_system(select_unit.in_set(OnUpdate(ClientState::Game)))
            .add_system(zoom_camera.in_set(OnUpdate(ClientState::Game)))
            .add_system(mouse_click_events.in_set(OnUpdate(ClientState::Game)))
            .add_system(mouse_pan_events.in_set(OnUpdate(ClientState::Game)))
            .add_system(scroll_camera.in_set(OnUpdate(ClientState::Game)))
            .add_system(keyboard_input.in_set(OnUpdate(ClientState::Game)));
    }
}

#[derive(Component, Debug, Reflect)]
pub struct GameCameraScalingInfo {
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
struct RenderedUnitAction;

#[derive(Component)]
pub struct RenderedIcon;

fn spawn_gameboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
    mut images: ResMut<Assets<Image>>
) {
    info!("SIZE {:?} {:?}", config.gameboard_config.width, config.gameboard_config.height);
    let map = Map::builder(
        UVec2 { x: config.gameboard_config.width, y: config.gameboard_config.height },
        asset_server.load("sprites/tilemap_atlas_new.png"),
        Vec2 { x: 16f32, y: 16f32 }
    ).build(&mut images);

    commands.spawn(MapBundle::new(map))
        .insert(MeshManagedByMap)
        .insert(Name::new("Gameboard"));
}

fn update_transforms(mut cam: Query<(&OrthographicProjection, &mut GameCameraScalingInfo)>, board_info: Query<&Gameboard>) {
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
    cam: Query<(&OrthographicProjection, &GameCameraScalingInfo)>, 
    mut tiles: Query<(Entity, &TileInfo, &mut Transform, With<GameScalable>)>,
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
    cam: Query<(&OrthographicProjection, &GameCameraScalingInfo)>, 
    mut features: Query<(Entity, &TileFeature, &mut Transform, With<GameScalable>)>,
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
    cam: Query<(&OrthographicProjection, &GameCameraScalingInfo)>, 
    mut units: Query<(Entity, &Unit, &mut Transform, With<GameScalable>)>
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

fn conform_transforms_unit_action_ghosts(
    cam: Query<(&OrthographicProjection, &GameCameraScalingInfo)>, 
    mut units: Query<(Entity, &UnitAction, &mut Transform, With<GameScalable>)>,
) {

    let scl = cam.single().1;

    if scl.unit_delta > 2f32 {
        return;
    }

    units.iter_mut().for_each(|(_e, ua, mut t, ())| {

        t.translation.x = (scl.unit_scl / 32f32) * (ua.action_pos[0] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.translation.y = (scl.unit_scl / 32f32) * (ua.action_pos[1] - (32 / 2)) as f32; // [gameboard_width / 2]
        t.scale.x *= scl.unit_delta;
        t.scale.y *= scl.unit_delta;
    });
}

fn conform_transforms_icons(
    cam: Query<(&OrthographicProjection, &GameCameraScalingInfo)>, 
    mut units: Query<(Entity, &Icon, &mut Transform, With<GameScalable>)>
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
struct GameScalable;

#[derive(Component, Reflect)]
pub struct GameCamera;

fn setup_scaling(mut commands: Commands, orth_q: Query<(Entity, &OrthographicProjection, With<GameCamera>)>) {
    let orth = orth_q.single().1;
    let x_scl = orth.area.max.x - orth.area.min.x;
    let y_scl = orth.area.max.y - orth.area.min.y;
    let unit_scl = f32::min(x_scl, y_scl);
    
    // TEMPORARILY spawn unit
    // TODO: Remove this

    commands.entity(orth_q.single().0).insert(GameCameraScalingInfo {
        x_scl: x_scl,
        y_scl: y_scl,
        unit_scl: unit_scl,
        unit_delta: 1f32,
    });
}