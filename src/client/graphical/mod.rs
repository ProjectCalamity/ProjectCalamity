pub mod inputs;

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_fast_tilemap::{Map, MapBundle, MeshManagedByMap};

use crate::common::{
    config::Config,
    logic::{units::UnitID, *},
};

use self::inputs::{
    keyboard_input, mouse_click_events, mouse_pan_events, scroll_camera, scroll_events,
    select_unit, zoom_camera, GridPosClickEvent, PanEvent, TurnCompletedEvent, ZoomEvent,
};

use super::{ClientState, Spritesheet};

pub struct GraphicalPlugin;

impl Plugin for GraphicalPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameCameraScalingInfo>()
            .add_event::<GridPosClickEvent>()
            .add_event::<PanEvent>()
            .add_event::<TurnCompletedEvent>()
            .add_event::<ZoomEvent>()
            .add_startup_system(spawn_gameboard)
            .add_system(setup_scaling.in_schedule(OnEnter(ClientState::Game)))
            .add_system(render.in_set(OnUpdate(ClientState::Game)))
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
    pos: Vec2,
}

#[derive(FromReflect, PartialEq, Reflect)]
pub enum Icons {
    Circle,
    Cross,
    Selector,
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
    mut images: ResMut<Assets<Image>>,
) {
    let map = Map::builder(
        UVec2 {
            x: config.gameboard_config.width,
            y: config.gameboard_config.height,
        },
        asset_server.load("sprites/tilemap_atlas_new.png"),
        Vec2 { x: 16f32, y: 16f32 },
    )
    .build(&mut images);

    commands
        .spawn(MapBundle::new(map))
        .insert(MeshManagedByMap)
        .insert(Name::new("Gameboard"));
}

fn render(
    mut commands: Commands,
    spritesheet: Res<Spritesheet>,
    unrendered_units: Query<(Entity, &Unit, Without<RenderedUnit>)>,
    mut rendered_units: Query<(&Unit, &mut Transform, With<RenderedUnit>)>,
    map_q: Query<&Map>,
) {
    let map = map_q.single();
    for (entity, unit, _) in &unrendered_units {
        // Since this gets us the middle of tiles, and we want the bottom left,
        // we take the diagonally down left tile, and average the positions

        let pos = map.map_to_world(unit.pos);
        let bundle = SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(texture_index_from_unit_id(&unit.id)),
            texture_atlas: spritesheet.characters.clone(), // To optimising Aurora, it's a handle!
            transform: Transform {
                translation: Vec3 {
                    x: pos.x,
                    y: pos.y,
                    z: 10f32,
                },
                scale: Vec3::splat(0.7),
                ..default()
            },
            ..default()
        };
        commands.entity(entity).insert(bundle).insert(RenderedUnit);
    }

    for (unit, mut transform, _) in &mut rendered_units {
        // Align to grid
        let pos = map.map_to_world(unit.pos);
        if transform.translation.xy() != pos {
            transform.translation = Vec3::new(pos.x, pos.y, 10f32);
        }

        // TODO: Set sprite
    }
}

fn texture_index_from_unit_id(uid: &UnitID) -> usize {
    return match uid {
        UnitID::ScienceGenericTest => 0,
        UnitID::MagicGenericTest => 1,
    };
}

#[derive(Component, Reflect)]
struct GameScalable;

#[derive(Component, Reflect)]
pub struct GameCamera;

fn setup_scaling(
    mut commands: Commands,
    orth_q: Query<(Entity, &OrthographicProjection, With<GameCamera>)>,
) {
    let orth = orth_q.single().1;
    let x_scl = orth.area.max.x - orth.area.min.x;
    let y_scl = orth.area.max.y - orth.area.min.y;
    let unit_scl = f32::min(x_scl, y_scl);

    // TEMPORARILY spawn unit
    // TODO: Remove this

    commands
        .spawn(Unit {
            id: UnitID::ScienceGenericTest,
            pos: Vec2::new(10f32, 10f32),
            health: Health(10f32),
            attack: Attack::default(),
            defense: Defense::default(),
            movement: Movement(5),
            turn_execute_stage: TurnExecuteStage::default(),
            archetype: Archetype::default(),
            owner: PlayerTeam(TeamColour::Red),
        })
        .insert(Name::new("Unit"));

    commands
        .entity(orth_q.single().0)
        .insert(GameCameraScalingInfo {
            x_scl: x_scl,
            y_scl: y_scl,
            unit_scl: unit_scl,
            unit_delta: 1f32,
        });
}
