use std::ops::Add;

use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_fast_tilemap::Map;

use crate::common::logic::neo_gameboard::Gameboard;

use super::GameCamera;

pub struct TurnCompletedEvent;

#[derive(Debug, Component, FromReflect, Reflect)]
pub struct SelectedUnit(pub Vec2);

#[derive(Debug)]
pub struct ZoomEvent {
    zoom: f32,
}

pub struct PanEvent {
    delta_x: f32,
    delta_y: f32,
}

#[derive(Debug)]
pub struct GridPosClickEvent {
    pos: Vec2,
}

pub fn scroll_events(
    mut zoom_evw: EventWriter<ZoomEvent>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        match ev.unit {
            // From mice, etc. For now, only bother handling these
            MouseScrollUnit::Line => {
                let e = ZoomEvent {
                    zoom: (1 as f32 + (0.1 * -ev.y)),
                };
                if e.zoom != 0f32 {
                    zoom_evw.send(e);
                }
            }
            MouseScrollUnit::Pixel => {
                let e = ZoomEvent {
                    zoom: (1 as f32 + (0.05 * -ev.y)),
                };
                if e.zoom != 0f32 {
                    zoom_evw.send(e);
                }
            }
        }
    }
}

pub fn keyboard_input(keys: Res<Input<KeyCode>>, mut turn_evw: EventWriter<TurnCompletedEvent>) {
    if keys.just_pressed(KeyCode::Return) {
        turn_evw.send(TurnCompletedEvent);
        info!("Ending turn")
    }
}

pub fn mouse_pan_events(
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut pan_evw: EventWriter<PanEvent>,
) {
    if buttons.pressed(MouseButton::Right) {
        motion_evr.iter().for_each(|ev| {
            pan_evw.send(PanEvent {
                delta_x: -ev.delta.x,
                delta_y: -ev.delta.y,
            })
        })
    }
}

pub fn mouse_click_events(
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut grid_pos_click_evw: EventWriter<GridPosClickEvent>,
) {
    if buttons.just_released(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = camera_q.single();

        let pos = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
            .unwrap();
        grid_pos_click_evw.send(GridPosClickEvent { pos })
    }
}

pub fn select_unit(
    mut commands: Commands,
    mut click_evr: EventReader<GridPosClickEvent>,
    map: Query<&Map>,
    gameboard: Query<&Gameboard>,
) {
    for click_event in click_evr.iter() {
        let map = map.single();
        // This is a bit messy, because the tilemap consider's the tile's "position" as it's top left corner
        let click_pos = click_event.pos.add(Vec2::new(8f32, -8f32));
        let map_pos = map.world_to_map(click_pos).round();
        let world_pos = map.map_to_world(map_pos).add(Vec2::new(-8f32, 8f32));
    }
}

pub fn zoom_camera(
    mut zoom_evr: EventReader<ZoomEvent>,
    mut cam: Query<(&mut Transform, With<Camera2d>, With<GameCamera>)>,
) {
    let (mut transform, _, _) = cam.single_mut();
    zoom_evr.iter().for_each(|ev| {
        let pot_x = transform.scale.x * ev.zoom;
        let pot_y = transform.scale.y * ev.zoom;

        transform.scale.x = pot_x;
        transform.scale.y = pot_y;

        if transform.scale.x > 1f32 || transform.scale.y > 1f32 {
            transform.scale.x = 1f32;
            transform.scale.y = 1f32;
        }

        if transform.scale.x < 0.2 || transform.scale.y < 0.2 {
            transform.scale.x = 0.2;
            transform.scale.y = 0.2;
        }
    })
}

pub fn scroll_camera(
    mut pan_evr: EventReader<PanEvent>,
    mut cam: Query<(&mut Transform, With<Camera2d>, With<GameCamera>)>,
) {
    const SENATIVITY: f32 = 0.7;

    let mut transform = cam.single_mut().0;
    pan_evr.iter().for_each(|ev| {
        let delta_x = transform.scale.x * ev.delta_x * SENATIVITY;
        let delta_y = transform.scale.y * ev.delta_y * SENATIVITY * -1f32;

        transform.translation.x += delta_x;
        transform.translation.y += delta_y;
    })
}
