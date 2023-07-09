use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_fast_tilemap::Map;

use crate::{
    client::graphical::{GameScalable, Icon, Icons},
    common::logic::{TraversableTiles, Unit, UnitAction, UnitActions},
};

use super::{GameCamera, RenderedIcon};

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
    mut grid_pos_click_evw: EventWriter<GridPosClickEvent>,
    map_q: Query<&Map>,
) {
    if buttons.just_released(MouseButton::Left) {
        let map = map_q.single();
        let window = windows.single();

        let pos = map.world_to_map(window.cursor_position().unwrap());
        grid_pos_click_evw.send(GridPosClickEvent { pos })
    }
}

pub fn select_unit(
    mut commands: Commands,
    mut click_evr: EventReader<GridPosClickEvent>,
    units: Query<(&mut Unit, &TraversableTiles)>,
    selected: Query<(Entity, &SelectedUnit)>,
    icons: Query<(Entity, &RenderedIcon)>,
    actions: Query<(Entity, &UnitAction)>,
) {
    for click_event in click_evr.iter() {
        info!("CLICK EVENT {:?}", click_event);
        // Get rid of all pre-existing icons
        icons
            .iter()
            .for_each(|(e, _)| commands.entity(e).despawn_recursive());

        // Wherever we click, we definitely want to despawn all icons
        icons
            .iter()
            .for_each(|(e, _)| commands.entity(e).despawn_recursive());

        // We already have a unit selected; now we're selecting it's position
        if selected.iter().len() == 1 {
            let selection_pos = selected.single().1 .0;
            // Ensure that the move is
            //    a) Valid
            //    b) Possible
            let unit_vec = units
                .iter()
                .filter(|(u, t)| u.pos == selection_pos && t.0.contains(&click_event.pos))
                .collect::<Vec<_>>();
            if unit_vec.len() == 1 {
                let (unit, _) = unit_vec[0];

                // Despawn all previous UnitActions pertaining to this unit
                actions
                    .iter()
                    .filter(|(_, a)| a.curr_pos == selection_pos)
                    .for_each(|(e, _)| commands.entity(e).despawn_recursive());

                // Spawn a new UnitAction
                // TODO: Deal with other action type contingencies
                commands.spawn(UnitAction {
                    action_type: UnitActions::Move,
                    turn_stage: unit.turn_execute_stage.clone(),
                    curr_pos: unit.pos,
                    action_pos: click_event.pos,
                });

                // Deselect unit, and despawn icons
                selected
                    .iter()
                    .for_each(|(e, _)| commands.entity(e).despawn_recursive());
            }
        }

        // No unit is selected, and thus we should attempt to select the unit at the click event
        if selected.iter().len() == 0 {
            let unit_vec = units
                .iter()
                .filter(|(u, _)| u.pos == click_event.pos)
                .collect::<Vec<_>>();

            // Select a unit if it's there
            if unit_vec.len() == 1 {
                // Select
                commands
                    .spawn(SelectedUnit(click_event.pos))
                    .insert(Name::new("Selected Unit"));

                // Spawn icons
                commands
                    .spawn(Icon {
                        icon: Icons::Selector,
                        pos: click_event.pos,
                    })
                    .insert(GameScalable)
                    .insert(Name::new("Selector"));

                unit_vec[0].1 .0.iter().for_each(|pos| {
                    commands
                        .spawn(Icon {
                            icon: Icons::Circle,
                            pos: pos.clone(),
                        })
                        .insert(GameScalable)
                        .insert(Name::new("Movable Tile"));
                });
            }
        }

        if selected.iter().len() > 1 {
            info!("More than one unit is selected, this shouldn't be possible. Deselecting all units.");
            selected
                .iter()
                .for_each(|(e, _)| commands.entity(e).despawn_recursive());
        }
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
