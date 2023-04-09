use bevy::prelude::*;
use noise::{Perlin, NoiseFn};
use super::*;

#[derive(Component, Debug, Reflect)]
pub struct GameboardGenerationParameters {
    seed: u32,
    mountain_scale: f64,
    water_scale: f64,
    mountain_range: [f64; 2],
    water_range: [f64; 2]
}

pub fn generate_gameboard(mut commands: Commands, mut existing: Query<&mut TileInfo>, info: Query<(&Gameboard, &GameboardGenerationParameters)>) {


    // TEMPORARILY spawn unit
    // TODO: Remove this

    commands.spawn(UnitBundle {
        unit: Unit { id: UnitID::ScienceGenericTest, pos: [2, 2], health: Health(3f32), attack: Attack { base: 2f32, range: 1, splash: false, splash_multiplier: 1f32, magic_multiplier: 1f32, science_multiplier: 1f32 }, defense: Defense { base: 3f32, magic_multiplier: 1f32, science_multiplier: 1f32 }, movement: Movement(1), turn_execute_stage: TurnExecuteStage(TurnExecuteStages::MidTurn), archetype: Archetype(Archetypes::Science) },
    });

    commands.spawn(UnitBundle {
        unit: Unit { id: UnitID::MagicGenericTest, pos: [4, 2], health: Health(3f32), attack: Attack { base: 2f32, range: 1, splash: false, splash_multiplier: 1f32, magic_multiplier: 1f32, science_multiplier: 1f32 }, defense: Defense { base: 3f32, magic_multiplier: 1f32, science_multiplier: 1f32 }, movement: Movement(1), turn_execute_stage: TurnExecuteStage(TurnExecuteStages::MidTurn), archetype: Archetype(Archetypes::Science) },
    });

    if existing.iter().len() == 0 {

        let gameboard = Gameboard {
            name: "Testing Game".to_string(),
            max_x: 32,
            max_y: 32,
        };

        let params = GameboardGenerationParameters {
            seed: 1,
            mountain_scale: 6f64,
            water_scale: 6f64,
            mountain_range: [0.95, 1f64],
            water_range: [-1f64, 0.8f64],
        };

        let max_x = gameboard.max_x;
        let max_y = gameboard.max_y;

        let mountain_range = params.mountain_range;
        let water_range = params.water_range;

        let mountain_scale = params.mountain_scale;
        let water_scale = params.water_scale;

        let water_noise = Perlin::new(params.seed >> (params.seed % 5));
        let mountain_noise = Perlin::new(params.seed << (params.seed % 7));

        let gameboard_id = commands.spawn(gameboard).id();
        commands.entity(gameboard_id).insert(params);

        for x in 0..max_x {
            for y in 0..max_y {
                
                let water_value = water_noise.get([x as f64 / water_scale, y as f64 / water_scale]);
                let mountain_value = mountain_noise.get([x as f64 / water_scale, y as f64 / mountain_scale]);

                let terrain = match water_value - mountain_value {
                    x if x > water_range[0] && x < water_range[1] => Geography::Water,
                    x if x > mountain_range[0] && x < mountain_range[1] => Geography::Mountains,
                    _ => Geography::None,
                };

                commands.spawn(TileInfo {
                    pos: [x as i32, y as i32],
                    geography: terrain,
                });
            }
        }
    } else {
        let params = info.single().1;

        let mountain_range = params.mountain_range;
        let water_range = params.water_range;

        let mountain_scale = params.mountain_scale;
        let water_scale = params.water_scale;

        let water_noise = Perlin::new(params.seed >> (params.seed % 5));
        let mountain_noise = Perlin::new(params.seed << (params.seed % 7));
            
        existing.iter_mut().for_each(|mut t| {
            let water_value = water_noise.get([t.pos[0] as f64 / water_scale, t.pos[1] as f64 / water_scale]);
            let mountain_value = mountain_noise.get([t.pos[0] as f64 / water_scale, t.pos[1] as f64 / mountain_scale]);

            t.geography = match water_value - mountain_value {
                x if x > water_range[0] && x < water_range[1] => Geography::Water,
                x if x > mountain_range[0] && x < mountain_range[1] => Geography::Mountains,
                _ => Geography::None,
            };
        });
    }   
}