use super::*;

/*
    A super messy list of units
    Organised by archaetype -> class -> path -> tier
*/

#[derive(Clone, Debug, Default, Deserialize, FromReflect, PartialEq, Reflect, Serialize)]
pub enum UnitID {
    #[default]
    ScienceGenericTest,
    MagicGenericTest,
}

/*
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;                                                                            ;;
;;               ----==| T I E R   Z E R O   U N I T S |==----                ;;
;;                                                                            ;;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
*/

//NOTE HASNT BEEN EDITED
// pub const NONE_NONE_NONE_NONE_TZERO: Unit = Unit {
//     health: Health(3f32),
//     attack: Attack {
//         base: 1f32,
//         range: 1i32,
//         splash: false,
//         splash_multiplier: 1f32,
//         magic_multiplier: 1f32,
//         science_multiplier: 1f32,
//     },
//     defense: Defense {
//         base: 0f32,
//         magic_multiplier: 1f32,
//         science_multiplier: 1f32,
//     },
//     movement: Movement(1),
//     turn_execute_stage: TurnExecuteStage(TurnExecuteStages::MidTurn),
//     archetype: Archetype(Archetypes::None),
// };

// const TIER_ZERO_BASE_UNIT: Unit = Unit {
//     health: Health(3f32),
//     attack: Attack {
//         base: 1f32,
//         range: 1i32,
//         splash: false,
//         splash_multiplier: 1f32,
//         magic_multiplier: 1f32,
//         science_multiplier: 1f32,
//     },
//     defense: Defense {
//         base: 0f32,
//         magic_multiplier: 1f32,
//         science_multiplier: 1f32,
//     },
//     movement: Movement(1),
//     turn_execute_stage: TurnExecuteStage(TurnExecuteStages::MidTurn),
//     archetype: Archetype(Archetypes::None),
// };

// /*
// ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// ;;                                                                            ;;
// ;;                ----==| T I E R   O N E   U N I T S |==----                 ;;
// ;;                                                                            ;;
// ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// */

// const SCIENCE_SUPPORT_HEALER_TONE: Unit = Unit {
//     health: Health(3f32),
//     attack: Attack {
//         base: 1f32,
//         range: 1i32,
//         splash: false,
//         splash_multiplier: 1f32,
//         magic_multiplier: 1f32,
//         science_multiplier: 1f32,
//     },
//     defense: Defense {
//         base: 0f32,
//         magic_multiplier: 1f32,
//         science_multiplier: 1f32,
//     },
//     movement: Movement(1),
//     turn_execute_stage: TurnExecuteStage(TurnExecuteStages::MidTurn),
//     archetype: Archetype(Archetypes::None),
// };
