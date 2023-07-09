pub mod inputs;

use std::ops::Add;

use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_fast_tilemap::{Map, MapBundle, MeshManagedByMap};

use crate::common::{
    config::Config,
    logic::{neo_gameboard::Gameboard, units::UnitID, *},
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
            .add_system(test_spawn_unit.in_schedule(OnEnter(ClientState::Game)))
            .add_system(render.in_set(OnUpdate(ClientState::Game)))
            .add_system(show_movable_tiles.in_set(OnUpdate(ClientState::Game)))
            .add_system(scroll_events.in_set(OnUpdate(ClientState::Game)))
            .add_system(select_unit.in_set(OnUpdate(ClientState::Game)))
            .add_system(zoom_camera.in_set(OnUpdate(ClientState::Game)))
            .add_system(mouse_click_events.in_set(OnUpdate(ClientState::Game)))
            .add_system(mouse_pan_events.in_set(OnUpdate(ClientState::Game)))
            .add_system(scroll_camera.in_set(OnUpdate(ClientState::Game)))
            .add_system(keyboard_input.in_set(OnUpdate(ClientState::Game)));
    }
}

#[derive(Component)]
pub struct IconTagTemp;

fn show_movable_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    units: Query<&Unit>,
    prev_icons: Query<Entity, With<IconTagTemp>>,
    gameboard: Query<&Gameboard>,
    map: Query<&Map>,
) {
    for gameboard in gameboard.iter() {
        let map = map.single();

        prev_icons.iter().for_each(|e| commands.entity(e).despawn());

        for unit in units.iter() {
            for tile in unit.calculate_traversible_tiles(gameboard, unit.movement.0 as f32) {
                let world_pos = map.map_to_world(tile).add(Vec2::new(-8f32, 8f32));
                commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Quad::new(Vec2::splat(8f32)).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
                        transform: Transform::from_translation(Vec3::new(
                            world_pos.x,
                            world_pos.y,
                            100f32,
                        )),
                        ..default()
                    })
                    .insert(IconTagTemp);
            }
        }
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
        // The unit needs to be transformed so it's center aligns with the center of the tile

        let pos = map.map_to_world(unit.pos).add(Vec2::splat(8f32));
        let bundle = SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(texture_index_from_unit_id(&unit.id)),
            texture_atlas: spritesheet.characters.clone(), // To optimising Aurora, it's a handle!
            transform: Transform {
                translation: Vec3 {
                    x: pos.x,
                    y: pos.y,
                    z: 10f32,
                },
                scale: Vec3::splat(0.5),
                ..default()
            },
            ..default()
        };
        commands.entity(entity).insert(bundle).insert(RenderedUnit);
    }

    for (unit, mut transform, _) in &mut rendered_units {
        // Align to grid
        let pos = map.map_to_world(unit.pos).add(Vec2::new(8f32, -8f32));
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

fn test_spawn_unit(mut commands: Commands) {
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
}
