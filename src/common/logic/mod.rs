pub mod units;
pub mod gameboard_gen;

use bevy::{prelude::*};
use bevy_quinnet::server::Server;
use serde::{Serialize, Deserialize};

use crate::server::{networking::ClientIDMap, ServerGameManager};

use self::{units::UnitID, gameboard_gen::GameboardGenerationParameters};

use super::networking::schema::{PlayerTileInfo, ServerMessages};
pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Archetype>()
            .register_type::<Attack>()
            .register_type::<Defense>()
            .register_type::<Gameboard>()
            .register_type::<GameboardGenerationParameters>()
            .register_type::<Health>()
            .register_type::<Movement>()
            .register_type::<PlayerTeam>()
            .register_type::<TileFeature>()
            .register_type::<TileFeatures>()
            .register_type::<TileInfo>()
            .register_type::<TurnExecuteStage>()
            .register_type::<Unit>()
            .add_system(calculate_traversable_tiles);
    }
}

fn calculate_traversable_tiles(
    mut commands: Commands,
    unit_q: Query<(Entity, &Unit)>,
    tiles_q: Query<&TileInfo>
) {
    unit_q.iter().for_each(|(e, u)| {

        let mut reachable_tiles = Vec::<[i32; 2]>::new();

        tiles_q.iter().for_each(|t| {
            // TODO: Improve searching algorithm
            if ((u.pos[0] - t.pos[0]).abs() + (u.pos[1] - t.pos[1]).abs() <= u.movement.0) 
                && u.pos != t.pos
            {
                reachable_tiles.push(t.pos.clone());
            }
        });
        commands.entity(e).insert(TraversableTiles(reachable_tiles));
    })
}

#[derive(Component, FromReflect, Reflect)]
pub struct TraversableTiles(pub Vec<[i32; 2]>);

#[derive(Component, FromReflect, Reflect)]
pub struct ViewableTiles(pub Vec<[i32; 2]>);


#[derive(Clone, Component, Debug, Deserialize, Reflect, Serialize)]
pub struct Gameboard {
    pub name: String,
    pub max_x: u32,
    pub max_y: u32,
}

#[derive(Component, Debug, Reflect, FromReflect)]
pub struct TileInfo {
    pub pos: [i32; 2],
    pub geography: Geography,
    pub visible_to_players: Vec<PlayerTeam>,
}

impl TileInfo {
    pub fn player_tile_info(&self, player: PlayerTeam, feature: Option<&&TileFeature>) -> PlayerTileInfo {
        let mut geography = Geography::Fog;
        if self.visible_to_players.contains(&player) {
            geography = self.geography;
        }

        let mut visible_features = None;

        if let Some(feature_ref) = feature {
            if self.visible_to_players.contains(&player) {
                let feature = feature_ref.clone().clone();
                if feature.feature != TileFeatures::CurrencySite(Archetype(Archetypes::Science)) 
                    && feature.feature != TileFeatures::CurrencySite(Archetype(Archetypes::Magic)) {
                    visible_features = Some(feature.clone());
                } else if feature.visible_to_players.contains(&player) {
                    visible_features = Some(feature.clone());
                }
            }
        }

        return PlayerTileInfo { 
            geography: geography, 
            pos: self.pos, 
            visible_features: visible_features
        };
    }
}

#[derive(Clone, Copy, Debug, Deserialize, FromReflect, PartialEq, Reflect, Serialize)]
pub enum Geography {
    None,
    Water,
    Mountains,
    Fog
}

#[derive(Bundle, Reflect)]
struct UnitActionBundle{
    unit_action: UnitAction,
}

#[derive(Clone, Component, Debug, Deserialize, Reflect, Serialize)]
pub struct UnitAction {
    pub action_type: UnitActions,
    pub turn_stage: TurnExecuteStage,
    pub curr_pos: [i32; 2],
    pub action_pos: [i32; 2]
}

impl UnitAction {
    pub fn apply(
        &self, 
        server: &Server,
        units_q: &mut Vec<Mut<Unit>>,
        tiles: &mut Vec<Mut<TileInfo>>,
        tile_features: &mut Vec<Mut<TileFeature>>,
        game_manager: &ServerGameManager,
        clients: &ClientIDMap,
    ) {
        if self.action_type == UnitActions::Move {
            // Move
            // Server
            let mut units = units_q
                .iter_mut()
                .filter(|u| u.pos == self.curr_pos)
                .collect::<Vec<_>>();

            let unit = units.get_mut(0).unwrap();

            unit.pos = self.action_pos;

            // Reveal tiles
            // Calculate viewable tiles
            let tiles_to_reveal = tiles
                .iter_mut()
                .filter(|tile| 
                    (unit.pos[0] - tile.pos[0]).abs()
                    + (unit.pos[1] - tile.pos[1]).abs()
                    <= unit.movement.0
                    && !tile.visible_to_players.contains(&unit.owner)
                )
                .map(|t| {
                    t.visible_to_players.push(unit.owner.clone());
                    return t;
                })
                .collect::<Vec<_>>();

            let client_id = game_manager.client_id(&unit.owner, &clients.map);
            
            // Register tiles as visible to client on server

            tiles_to_reveal.iter().for_each(|tile| {

                let mut feature = None;

                let relevant_feature_filtered = tile_features
                    .iter()
                    .filter_map(|tf| {
                        if tf.pos == tile.pos {
                            if let TileFeatures::Nest(_) = tf.feature {
                                return Some(tf);
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    })
                    .collect::<Vec<_>>();
                if relevant_feature_filtered.len() == 1 {
                    let feature_uncloned = relevant_feature_filtered[0];
                    // God is dead, and I have killed them
                    feature = Some(TileFeature {
                        pos: feature_uncloned.pos,
                        feature: feature_uncloned.feature.clone(),
                        visible_to_players: feature_uncloned.visible_to_players.clone(),
                    });
                }

                server.endpoint().send_message(
                    client_id.clone(),
                    ServerMessages::PlayerTileInfoPacket { 
                        tile: PlayerTileInfo {
                            pos: tile.pos, 
                            geography: tile.geography, 
                            visible_features: feature
                        } 
                    }
                )
                .unwrap();
            });

            // Move client-side (only those that can see the tile on which this unit is)
            // We need to reveal before moving, so that we can see the tile that the unit moves to
            // We also need to make sure we despawn the unit from the client's perspective if it moves out
            // of territory that they know about

            let tile_at_unit_pos = &tiles
                .iter()
                .filter(|t| t.pos == self.action_pos)
                .collect::<Vec<_>>()
                [0];
            
            let visible_clients = tile_at_unit_pos
                .visible_to_players
                .iter()
                .map(|t| game_manager.client_id(t, &clients.map))
                .collect::<Vec<_>>();
                
            visible_clients
                .iter()
                .for_each(|id| { 
                    server
                        .endpoint()
                        .send_message(
                            id.clone(), 
                            ServerMessages::UnitModifyPacket { 
                                prev_pos: self.curr_pos, 
                                // For some reason, `clone()` wouldn't work, so we have to do this godforsaken bullshit
                                unit: Unit { 
                                    id: unit.id.clone(), 
                                    pos: unit.pos.clone(), 
                                    health: unit.health.clone(), 
                                    attack: unit.attack.clone(), 
                                    defense: unit.defense.clone(), 
                                    movement: unit.movement.clone(), 
                                    turn_execute_stage: unit.turn_execute_stage.clone(), 
                                    archetype: unit.archetype.clone(), 
                                    owner: unit.owner.clone() 
                                }
                            }
                        )
                        .unwrap(); 
                    }
                );
        }

        // TODO: Special

        else if self.action_type == UnitActions::Attack {
            info!("ATTACK ACTION TODO")
        }
    }
}

#[derive(Clone, Debug, Deserialize, Reflect, PartialEq, Serialize)]
pub enum UnitActions {
    Move,
    Attack,
    Heal,
    Build,
}

#[derive(Bundle, Default, Reflect, FromReflect)]
pub struct UnitBundle {
    pub unit: Unit,
}

#[derive(Clone, Component, Debug, Default, Deserialize, Reflect, FromReflect, Serialize)]
pub struct Unit {
    pub id: UnitID,
    pub pos: [i32; 2],
    pub health: Health,
    pub attack: Attack,
    pub defense: Defense,
    pub movement: Movement,
    pub turn_execute_stage: TurnExecuteStage,
    pub archetype: Archetype,
    pub owner: PlayerTeam,
}

#[derive(Clone, Component, Debug, Default, Deserialize, FromReflect, Reflect, Serialize)]
pub struct Health(pub f32);

#[derive(Clone, Component, Debug, Default, Deserialize, FromReflect, Reflect, Serialize)]
pub struct Attack {
    pub base: f32,
    pub range: i32,
    pub splash: bool, // (0.3 * base) per adjacent unit
    pub splash_multiplier: f32,
    pub magic_multiplier: f32,
    pub science_multiplier: f32,
}

#[derive(Clone, Component, Debug, Default, Deserialize, FromReflect, Reflect, Serialize)]
pub struct Defense {
    pub base: f32,
    pub magic_multiplier: f32,
    pub science_multiplier: f32,
}

#[derive(Clone, Component, Debug, Default, Deserialize, FromReflect, Reflect, Serialize)]
pub struct Movement(pub i32);

#[derive(Clone, Component, Debug, Default, Deserialize, FromReflect, PartialEq, Reflect, Serialize)]
pub struct TurnExecuteStage(pub TurnExecuteStages);

#[derive(Clone, Component, Debug, Default, Deserialize, FromReflect, PartialEq, Reflect, Serialize)]
pub enum TurnExecuteStages {
    PreTurn,
    #[default]
    MidTurn,
    AfterTurn,
}

#[derive(Clone, Component, Debug, Default, Deserialize, FromReflect, PartialEq, Reflect, Serialize)]
pub struct Archetype(pub Archetypes);

#[derive(Clone, Component, Debug, Default, Deserialize, FromReflect, PartialEq, Reflect, Serialize)]
pub enum Archetypes {
    Magic,
    Science,
    #[default]
    None
}

#[derive(Component, Reflect, FromReflect)]
struct HealAction {
    base: f32,
    splash: bool, // (0.3 * base) per adjacent unit
    range: i32,
}

#[derive(Clone, Component, Debug, Deserialize, FromReflect, PartialEq, Reflect, Serialize)]
pub struct TileFeature {
    pub pos: [i32; 2],
    pub feature: TileFeatures,
    pub visible_to_players: Vec<PlayerTeam>
}

#[derive(Clone, Component, Debug, Deserialize, FromReflect, PartialEq, Reflect, Serialize)]
pub enum TileFeatures {
    CurrencySite(Archetype),
    Nest(PlayerTeam)
}

#[derive(Clone, Component, Debug, Default, Deserialize, Eq, FromReflect, Hash, PartialEq, Reflect, Serialize)]
pub struct PlayerTeam(pub TeamColour);

#[derive(Clone, Debug, Default, Deserialize, Eq, FromReflect, Hash, PartialEq, Reflect, Serialize)]
pub enum TeamColour {
    #[default]
    Blue,
    Red,
    Purple,
    Yellow
}

impl TeamColour {
    pub fn from_int(index: &usize) -> TeamColour {
        return match index {
            0 => TeamColour::Blue,
            1 => TeamColour::Red,
            2 => TeamColour::Purple,
            3 => TeamColour::Yellow,
            _ => TeamColour::Blue,
        };
    }
}