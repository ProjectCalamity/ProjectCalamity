use bevy::prelude::*;
use noise::{Perlin, NoiseFn};
use rand::Rng;
use crate::{common::networking::schema::Player,};

use super::*;

#[derive(Component, Debug, Reflect)]
pub struct GameboardGenerationParameters {
    seed: u32,
    mountain_scale: f64,
    water_scale: f64,
    mountain_range: [f64; 2],
    water_range: [f64; 2]
}

pub fn generate_gameboard(
    mut commands: Commands, 
    existing: Query<&mut TileInfo>, 
    players: Query<(&Player, &PlayerTeam)>,
) {
    if existing.iter().len() == 0 && players.iter().len() > 0 {
        info!("Generating gameboard");

        let gameboard = Gameboard {
            name: "Testing Game".to_string(),
            max_x: 1024,
            max_y: 1024,
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

        let nest_tiles = calculate_nest_positions(max_x, max_y, players.iter().len() as u32);

        for x in 0..max_x {
            for y in 0..max_y {
                
                let water_value = water_noise.get([x as f64 / water_scale, y as f64 / water_scale]);
                let mountain_value = mountain_noise.get([x as f64 / water_scale, y as f64 / mountain_scale]);

                let terrain = match water_value - mountain_value {
                    x if x > water_range[0] && x < water_range[1] => TileGeography::Water,
                    x if x > mountain_range[0] && x < mountain_range[1] => TileGeography::Mountains,
                    _ => TileGeography::Grass,
                };

                let mut visibility = Vec::<PlayerTeam>::new();
                for i in 0..nest_tiles.len() {
                    let (n_x, n_y) = nest_tiles.get(i).unwrap();
                    if i32::abs(*n_x as i32 - x as i32) <= 1 && i32::abs(*n_y as i32 - y as i32) <= 1 {
                        visibility.push(players.iter().collect::<Vec<(&Player, &PlayerTeam)>>()[i].1.clone());
                    }
                }

                commands.spawn(TileInfo {
                    pos: [x as i32, y as i32],
                    geography: terrain,
                    visible_to_players: visibility, // All tiles are invisible; we'll make a 3x3 area around the nest visible initially
                }).insert(Name::new(format!("Tile at [{:?}, {:?}]", x, y)));

                let n = rand::thread_rng().gen_range(0..40);

                // Don't place a feature if we want a nest on there!
                if nest_tiles.iter().filter(|(n_x, n_y)| n_x == &x && n_y == &y).collect::<Vec<&(u32, u32)>>().len() > 0 {
                    continue;
                }

                if n == 0 || n == 1 {
                    // 0 is scrapyard, 1 is ancient library.
                    let tf: TileFeature;
                    if n == 0 {
                        tf = TileFeature { pos: [x as i32, y as i32], feature: TileFeatures::CurrencySite(Archetype(Archetypes::Science)), visible_to_players: Vec::new() };
                    } else {
                        tf = TileFeature { pos: [x as i32, y as i32], feature: TileFeatures::CurrencySite(Archetype(Archetypes::Magic)), visible_to_players: Vec::new() };
                    }
                    commands.spawn(tf)
                        .insert(Name::new(format!("Tile Feature at [{:?}, {:?}]", x, y)));
                }

            }
        }

        for (i, (_player, team)) in players.iter().enumerate() {
            let pos_bad = nest_tiles.get(i).unwrap();
            let pos = [pos_bad.0.clone() as i32, pos_bad.1.clone() as i32];
            commands.spawn(
                TileFeature { 
                    pos: pos,
                    feature: TileFeatures::Nest(team.clone()),
                    visible_to_players: vec![team.clone()]
                }
            ).insert(Name::new(format!("Nest at [{:?}, {:?}]", pos[0], pos[1])));
            
            // Two accompanying units

            for pos_mod in [-1, 1] {
                commands.spawn(UnitBundle {
                    unit: Unit { 
                        id: UnitID::ScienceGenericTest, 
                        pos: [pos[0], pos[1] + pos_mod], 
                        health: Health(3f32), 
                        attack: Attack { 
                            base: 1f32, 
                            range: 1, 
                            splash: false, 
                            splash_multiplier: 1f32, 
                            magic_multiplier: 1f32, 
                            science_multiplier: 1f32 
                        }, 
                        defense: Defense { 
                            base: 1f32, 
                            magic_multiplier: 1f32, 
                            science_multiplier: 1f32 
                        }, 
                        movement: Movement(1),
                        turn_execute_stage: TurnExecuteStage(TurnExecuteStages::MidTurn), 
                        archetype: Archetype(Archetypes::None), 
                        owner: team.clone() 
                    },
            }).insert(Name::new(format!("Unit at [{:?}]", [pos[0], pos[1] + pos_mod])));
            }
        }
    }
}

fn calculate_nest_positions(max_x: u32, max_y: u32, n: u32) -> Vec<(u32, u32)> {
    match n {
        2 => return vec![(1, max_y - 2), (max_x - 2, 1)],
        3 => return vec![(1, max_y - 2), (max_x - 2, max_y - 2), ((max_x / 2) as u32, 1)],
        4 => return vec![(1, 1), (1, max_y - 2), (max_x - 2, 1), (max_x - 2, max_y - 2)],
        _ => return vec![(1, 1), (1, max_y - 2), (max_x - 2, 1), (max_x - 2, max_y - 2)],
    }
}