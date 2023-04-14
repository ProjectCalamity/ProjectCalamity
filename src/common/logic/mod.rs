pub mod units;
pub mod gameboard_gen;

use bevy::prelude::*;

use self::{units::UnitID, gameboard_gen::{generate_gameboard, GameboardGenerationParameters}};
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
            .register_type::<TileFeature>()
            .register_type::<TileInfo>()
            .register_type::<TurnExecuteStage>()
            .register_type::<Unit>()
            .add_system(generate_gameboard)
            .add_system(calculate_traversable_tiles);
    }
}

/*

Some unit moves specs

All players send a UnitMoves struct, containing:
 - Type
    -> Move
    -> Attack
    -> Heal
    -> Build
 - Initial unit position [we can get the unit from this]
 - Desired position

Then, we (in 3 stages, doing this for each stage):
 - Check that the tile is within movement/range of the unit
 - Provisionally perform the actions of the move
 - Check for conflicts
 - Continue to next stage

Finally we return gamestate
*/

// I'll write this as a normal fn for now, and deal with Bevy's shitfuckery later
// fn execute_move(mut commands: Commands, tiles: Query<(Entity, &mut TileInfo, Option<&mut Unit>)>, actions: Query<(Entity, &UnitAction)>) {

//     // We literally don't even have the components to perform a single move...
//     if tiles.iter().len() < 2 || actions.iter().len() < 1 {
//         return;
//     }

//     // PRE-TURN - movements, some attacks
//     actions.iter().for_each(|(entity, action)| {
//         // Input sanitisation? Never met her.
//         // Move out of the fucking board. I dare you. I double dare you. I TRIPLE dare you.
//         // TODO: refine distence calcs

//         let binding_one = tiles
//             .iter()
//             .filter(|(_e, t, _u)| t.pos[0] == action.action_pos[0] && t.pos[1] == action.action_pos[1])
//             .collect::<Vec<(Entity, &TileInfo, Option<&Unit>)>>();
//         let act_t = binding_one
//             .get(0)
//             .unwrap();
//         let binding_two = tiles
//             .iter()
//             .filter(|(_e, t, _u)| t.pos[0] == action.curr_pos[0] && t.pos[1] == action.curr_pos[1])
//             .collect::<Vec<(Entity, &TileInfo, Option<&Unit>)>>();
//         let curr_t = binding_two
//             .get(0)
//             .unwrap();
//         let dist = (action.action_pos[0] - action.curr_pos[0]).abs() + (action.action_pos[1] - action.curr_pos[1]).abs();
//         // STEP 1: Check
//         if action.action_type == UnitActions::Move {
//             if let Some(unit) = curr_t.2 {
//                 if dist > unit.movement.0 {
//                     // This is what happens when you don't read my code comments.
//                     // *Stay inside the board*
//                     return;
//                 }
//                 else {
//                 // Move.
//                     commands.entity(curr_t.0).remove::<Unit>();
//                     commands.entity(act_t.0).insert(Unit {
//                         ..Default::default()
//                     });
//                 }
//             }
//             else {
//                 return
//             }
//         }
//     // TODO: Handle other types
//     // (Attack, Heal and Build)

//         commands.entity(entity).remove::<UnitAction>();
//     })

// }

// ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// ;;                                                                            ;;
// ;;          ----==| N O T   I N   U S E   C U R R E N T L Y |==----           ;;
// ;;                                                                            ;;
// ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

/*
fn exec_action(mut commands: Commands, action: &UnitAction, tiles: Query<(Entity, &mut Tile, &Movement)>) {
    // Input sanitisation? Never met her.
    // Move out of the fucking board. I dare you. I double dare you. I TRIPLE dare you.
    // TODO: refine distence calcs

    let binding = tiles
        .iter()
        .filter(|(_e, t, _m)| t.pos[0] == action.action_pos[0] && t.pos[1] == action.action_pos[1])
        .collect::<Vec<(Entity, &Tile, &Movement)>>();
    let act_t = binding
        .get(0)
        .unwrap();
    let binding = tiles
        .iter()
        .filter(|(_e, t, _m)| t.pos[0] == action.curr_pos[0] && t.pos[1] == action.curr_pos[1])
        .collect::<Vec<(Entity, &Tile, &Movement)>>();
    let curr_t = binding
        .get(0)
        .unwrap();
    let dist = (action.action_pos[0] - action.curr_pos[0]).abs() + (action.action_pos[1] - action.curr_pos[1]).abs();
    if action.action_type == UnitActions::Move {
        if dist > curr_t.2.0 {
            // This is what happens when you don't read my code comments.
            // *Stay inside the board*
            return;
        }
        else {
        // Move.
            eprintln!("Moving unit {:?} to {:?}", curr_t.1.pos, act_t.1.pos);
            commands.entity(curr_t.0).remove::<Unit>();
            commands.entity(act_t.0).insert(Unit {
                ..Default::default()
            });
        }
    }
    // TODO: Handle other types
    // (Attack, Heal and Build)
}
 */

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

#[derive(Component, Reflect)]
pub struct Gameboard {
    pub name: String,
    pub max_x: u32,
    pub max_y: u32,
}

#[derive(Component, Debug, Reflect, FromReflect)]
pub struct TileInfo {
    pub pos: [i32; 2],
    pub geography: Geography,
}

#[derive(Debug, FromReflect, PartialEq, Reflect)]
pub enum Geography {
    None,
    Water,
    Mountains
}

#[derive(Bundle, Reflect)]
struct UnitActionBundle{
    unit_action: UnitAction,
}

#[derive(Component, Debug, Reflect)]
struct UnitAction {
    action_type: UnitActions,
    turn_stage: TurnExecuteStage,
    curr_pos: [i32; 2],
    action_pos: [i32; 2]
}

#[derive(Debug, Reflect, PartialEq)]
enum UnitActions {
    Move,
    Attack,
    Heal,
    Build,
}

#[derive(Bundle, Default, Reflect, FromReflect)]
pub struct UnitBundle {
    pub unit: Unit,
}

#[derive(Component, Debug, Default, Reflect, FromReflect)]
pub struct Unit {
    pub id: UnitID,
    pub pos: [i32; 2],
    pub health: Health,
    pub attack: Attack,
    pub defense: Defense,
    pub movement: Movement,
    pub turn_execute_stage: TurnExecuteStage,
    pub archetype: Archetype,
}

#[derive(Component, Debug, Default, FromReflect, Reflect)]
pub struct Health(pub f32);

#[derive(Component, Debug, Default, FromReflect, Reflect)]
pub struct Attack {
    pub base: f32,
    pub range: i32,
    pub splash: bool, // (0.3 * base) per adjacent unit
    pub splash_multiplier: f32,
    pub magic_multiplier: f32,
    pub science_multiplier: f32,
}

#[derive(Component, Debug, Default, FromReflect, Reflect)]
pub struct Defense {
    pub base: f32,
    pub magic_multiplier: f32,
    pub science_multiplier: f32,
}

#[derive(Component, Debug, Default, FromReflect, Reflect)]
pub struct Movement(pub i32);

#[derive(Component, Debug, Default, FromReflect, PartialEq, Reflect)]
pub struct TurnExecuteStage(pub TurnExecuteStages);

#[derive(Component, Debug, Default, FromReflect, PartialEq, Reflect)]
pub enum TurnExecuteStages {
    PreTurn,
    #[default]
    MidTurn,
    AfterTurn,
}

#[derive(Component, Debug, Default, PartialEq, Reflect, FromReflect)]
pub struct Archetype(pub Archetypes);

#[derive(Component, Debug, Default, FromReflect, PartialEq, Reflect)]
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

#[derive(Component, Debug, FromReflect, PartialEq, Reflect)]
pub struct TileFeature {
    pub pos: [i32; 2],
    pub feature: TileFeatures
}

#[derive(Component, Debug, FromReflect, PartialEq, Reflect)]
pub enum TileFeatures {
    CurrencySite(Archetype),
    Nest(PlayerTeam)
}

#[derive(Debug, FromReflect, PartialEq, Reflect)]
pub struct PlayerTeam(pub TeamColour);

#[derive(Debug, FromReflect, PartialEq, Reflect)]
pub enum TeamColour {
    Blue,
    Red
}