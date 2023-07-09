pub mod neo_gameboard;
pub mod units;

use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};

use self::{
    neo_gameboard::{Gameboard, Tile},
    units::UnitID,
};
pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Archetype>()
            .register_type::<Attack>()
            .register_type::<Defense>()
            .register_type::<Health>()
            .register_type::<Movement>()
            .register_type::<PlayerTeam>()
            .register_type::<TileFeature>()
            .register_type::<TileFeatures>()
            .register_type::<TurnExecuteStage>()
            .register_type::<Unit>()
            .register_type::<UnitAction>();
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, FromReflect, PartialEq, Reflect, Serialize)]
pub enum Terrain {
    Desert,
    Forest,
    #[default]
    Grass,
    Jungle,
    Mountains,
    Savanna,
    ShallowWater,
    Water,
}

impl Terrain {
    fn to_atlas_index(&self, rand: &mut ThreadRng) -> u16 {
        match self {
            Terrain::Desert => return rand.gen_range(16..20),
            Terrain::Forest => return rand.gen_range(24..28),
            Terrain::Grass => return rand.gen_range(0..4),
            Terrain::Jungle => return rand.gen_range(12..16),
            Terrain::Mountains => return rand.gen_range(20..24),
            Terrain::Savanna => return rand.gen_range(28..32),
            Terrain::ShallowWater => return rand.gen_range(8..12),
            Terrain::Water => return rand.gen_range(4..8),
        };
    }
}

#[derive(Bundle, Reflect)]
struct UnitActionBundle {
    unit_action: UnitAction,
}

#[derive(Clone, Component, Debug, Deserialize, Reflect, Serialize)]
pub struct UnitAction {
    pub action_type: UnitActions,
    pub turn_stage: TurnExecuteStage,
    pub curr_pos: Vec2,
    pub action_pos: Vec2,
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
    pub pos: Vec2,
    pub health: Health,
    pub attack: Attack,
    pub defense: Defense,
    pub movement: Movement,
    pub turn_execute_stage: TurnExecuteStage,
    pub archetype: Archetype,
    pub owner: PlayerTeam,
}

impl Unit {
    pub fn calculate_traversible_tiles(&self, gameboard: &Gameboard, movement: f32) -> Vec<Vec2> {
        let mut tile_movement_costs =
            vec![vec![None::<f32>; gameboard.y() as usize]; gameboard.x() as usize];

        let (x, y) = (self.pos.x as usize, self.pos.y as usize);
        tile_movement_costs[x][y] = Some(0f32);

        // Recursion me harder
        gameboard.tile(x, y).unwrap().propogate_movement_costs(
            vec![gameboard.tile(x, y).unwrap()],
            &mut tile_movement_costs,
            gameboard,
        );

        let mut movable_tiles = Vec::<Vec2>::new();
        for x in 0..tile_movement_costs.len() {
            for y in 0..tile_movement_costs[0].len() {
                print!(" | {:?}", tile_movement_costs[x][y].unwrap());
                if tile_movement_costs[x][y].unwrap() <= movement {
                    movable_tiles.push(gameboard.tile(x, y).unwrap().pos());
                }
            }
            println!("");
        }
        println!("\n\n\n NEW TABLE \n\n\n");

        // Let's quickly print our thingy

        return movable_tiles;
    }
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

#[derive(
    Clone, Component, Debug, Default, Deserialize, FromReflect, PartialEq, Reflect, Serialize,
)]
pub struct TurnExecuteStage(pub TurnExecuteStages);

#[derive(
    Clone, Component, Debug, Default, Deserialize, FromReflect, PartialEq, Reflect, Serialize,
)]
pub enum TurnExecuteStages {
    PreTurn,
    #[default]
    MidTurn,
    AfterTurn,
}

#[derive(
    Clone, Component, Debug, Default, Deserialize, FromReflect, PartialEq, Reflect, Serialize,
)]
pub struct Archetype(pub Archetypes);

#[derive(
    Clone, Component, Debug, Default, Deserialize, FromReflect, PartialEq, Reflect, Serialize,
)]
pub enum Archetypes {
    Magic,
    Science,
    #[default]
    None,
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
    pub visible_to_players: Vec<PlayerTeam>,
}

#[derive(Clone, Component, Debug, Deserialize, FromReflect, PartialEq, Reflect, Serialize)]
pub enum TileFeatures {
    CurrencySite(Archetype),
    Nest(PlayerTeam),
}

#[derive(
    Clone,
    Component,
    Debug,
    Default,
    Deserialize,
    Eq,
    FromReflect,
    Hash,
    PartialEq,
    Reflect,
    Serialize,
)]
pub struct PlayerTeam(pub TeamColour);

#[derive(
    Clone, Debug, Default, Deserialize, Eq, FromReflect, Hash, PartialEq, Reflect, Serialize,
)]
pub enum TeamColour {
    #[default]
    Blue,
    Red,
    Purple,
    Yellow,
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
