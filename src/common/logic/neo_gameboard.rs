use bevy::prelude::*;
use bevy_fast_tilemap::{Map, MapReadyEvent};
use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Perlin,
};
use rand::Rng;

use crate::common::config::Config;

use super::{TeamColour, Terrain, TileFeature};

#[derive(Component, Default, Reflect)]
pub struct Gameboard {
    tiles: Vec<Vec<Tile>>,
    x: u32,
    y: u32,
}

impl Gameboard {
    pub fn tile(&self, x: usize, y: usize) -> Option<&Tile> {
        // So we don't have to deal with the pain of Vec::get(), which always
        // returns Option<&T>, we do this logic ourselves, because his is a 2D
        // array, which would otherwise return an Option<&&Tile> and require
        // .unwrap()'s and pain

        if self.tiles.len() > x && self.tiles[0].len() > y {
            return Some(&self.tiles[x][y]);
        }

        return None;
    }

    pub fn adjacent_tiles(&self, x: usize, y: usize) -> Vec<&Tile> {
        let mut adjacent_tiles = Vec::<&Tile>::with_capacity(9);
        // [-1, 1] - x is +1 from its actual value
        for x_mod in 0..3 {
            for y_mod in 0..3 {
                if x + x_mod > 0 {
                    if let Some(col) = self.tiles.get(x + x_mod - 1) {
                        if y + y_mod > 0 {
                            if let Some(tile) = col.get(y + y_mod - 1) {
                                adjacent_tiles.push(tile)
                            }
                        }
                    }
                }
            }
        }
        return adjacent_tiles;
    }

    pub fn x(&self) -> u32 {
        return self.x;
    }

    pub fn y(&self) -> u32 {
        return self.y;
    }
}

#[derive(Component, Debug, Default, FromReflect, Reflect)]
pub struct Tile {
    contents: Terrain,
    feature: Option<TileFeature>,
    visible_for: Vec<TeamColour>,
    pos: Vec2,
}

impl Tile {
    pub fn pos_usize(&self) -> (usize, usize) {
        return (self.pos.x as usize, self.pos.y as usize);
    }

    pub fn pos(&self) -> Vec2 {
        return self.pos;
    }

    pub fn movement_cost(&self) -> f32 {
        let speed_modifier = match self.contents {
            Terrain::Desert => 1.2,
            Terrain::Forest => 0.9,
            Terrain::Grass => 1.5,
            Terrain::Jungle => 0.7,
            Terrain::Mountains => 0.5,
            Terrain::Savanna => 1.4,
            Terrain::ShallowWater => 0.9,
            Terrain::Water => 0.7,
        };

        return 1f32 / speed_modifier;
    }

    pub fn propogate_movement_costs(
        &self,
        tile_ring: Vec<&Tile>,
        tile_movement_costs: &mut Vec<Vec<Option<f32>>>,
        gameboard: &Gameboard,
    ) {
        let mut propogate_tiles = Vec::<&Tile>::with_capacity(8);

        for (og_x, og_y) in tile_ring.iter().map(|t| t.pos_usize()) {
            // We only want the unset tiles, so we always propogate outwards
            let surrounding_tiles = gameboard.adjacent_tiles(og_x, og_y);
            for tile in surrounding_tiles {
                let (x, y) = tile.pos_usize();
                if tile_movement_costs[x][y] == None {
                    tile_movement_costs[x][y] =
                        Some(tile.movement_cost() + tile_movement_costs[og_x][og_y].unwrap());
                    propogate_tiles.push(tile);
                }
            }

            // Order from low to high, so we lock in the lowest value. Because
            // floats can't be ordered for some reason, multiply by 10000 and then
            // use a u32
            propogate_tiles
                .iter()
                .map(|t| {
                    let (x, y) = t.pos_usize();
                    return (tile_movement_costs[x][y].unwrap() * 10000f32) as u32;
                })
                .collect::<Vec<u32>>()
                .sort();
        }

        // Recursion is a bitch
        if propogate_tiles.len() > 0 {
            self.propogate_movement_costs(propogate_tiles, tile_movement_costs, gameboard);
        }
    }
}

pub fn spawn_gameboard(
    mut commands: Commands,
    config: Res<Config>,
    mut images: ResMut<Assets<Image>>,
    gameboard_q: Query<&Gameboard>,
    mut evs: EventReader<MapReadyEvent>,
    mut maps: Query<&mut Map>,
) {
    if gameboard_q.iter().len() > 0 {
        return;
    }

    for ev in evs.iter() {
        let map = maps.get_mut(ev.map).unwrap();

        let gameboard_config = &config.gameboard_config;

        let mut gameboard = Gameboard {
            tiles: Vec::with_capacity(gameboard_config.width as usize),
            x: gameboard_config.width,
            y: gameboard_config.height,
        };

        let mut rand = rand::thread_rng();
        let seed = rand.gen_range(0..u128::MAX);

        let scale =
            u32::max(gameboard_config.width, gameboard_config.height) as f64 * (1f64 / 32f64);

        let heightmap_seed = (seed >> 96) as u32;
        let inlandness_seed = ((seed >> 64) & 0xFFFF_FFFF) as u32;
        let climate_seed = ((seed >> 32) & 0xFFFF_FFFF) as u32;
        let rainfall_seed = (seed & 0xFFFF_FFFF) as u32;

        let heightmap = new_perlin_noise(
            scale,
            heightmap_seed,
            gameboard_config.width,
            gameboard_config.height,
        );
        let inlandness = new_perlin_noise(
            scale,
            inlandness_seed,
            gameboard_config.width,
            gameboard_config.height,
        );
        let climate = new_perlin_noise(
            scale,
            climate_seed,
            gameboard_config.width,
            gameboard_config.height,
        );

        for x in 0..gameboard_config.width {
            gameboard
                .tiles
                .push(Vec::with_capacity(gameboard_config.height as usize));
            for y in 0..gameboard_config.height {
                let tile = tile_at_position(x, y, &heightmap, &inlandness, &climate);
                if let Ok(mut m) = map.get_mut(&mut *images) {
                    m.set(x, y, tile.to_atlas_index(&mut rand));
                }
                gameboard.tiles.get_mut(x as usize).unwrap().push(Tile {
                    contents: tile,
                    feature: None,
                    visible_for: Vec::new(),
                    pos: Vec2::new(x as f32, y as f32),
                });
            }
        }

        commands.spawn(gameboard).insert(Name::new("Gameboard"));
    }
}

fn tile_at_position(
    x: u32,
    y: u32,
    heightmap: &NoiseMap,
    climate: &NoiseMap,
    rainfall: &NoiseMap,
) -> Terrain {
    let x = x as usize;
    let y = y as usize;

    // First, deal with water/land. If height < 0, we're underwater.
    if heightmap.get_value(x, y) < 0f64 {
        // Underwater
        if heightmap.get_value(x, y) < -0.5f64 {
            // We're low underwater, so we use deep water
            return Terrain::Water;
        } else {
            // We're relatively shallow
            return Terrain::ShallowWater;
        }
    }
    // By the same token, if we're really high, we're getting mountains regardless.
    else if heightmap.get_value(x, y) > 0.92f64 {
        // Mountain time
        return Terrain::Mountains;
    }
    // Finally, a special beaches exception
    else if heightmap.get_value(x, y) > 0f64 && heightmap.get_value(x, y) < 0.1f64 {
        return Terrain::Desert;
    }
    // Now we know that we're on land, we look to the other variables
    else {
        // This is basically a 4 corner graph
        /*  Wet --------- Dry
           H
           o Jungle   Desert
           t
           |
           |    Grassland
           C
           o
           l Forest  Savanna
           d
        */
        if rainfall.get_value(x, y) < -0.1f64 && climate.get_value(x, y) < -0.1f64 {
            return Terrain::Savanna;
        } else if rainfall.get_value(x, y) > 0.1f64 && climate.get_value(x, y) < -0.1f64 {
            return Terrain::Forest;
        } else if rainfall.get_value(x, y) < -0.1f64 && climate.get_value(x, y) > 0.1f64 {
            return Terrain::Desert;
        } else if rainfall.get_value(x, y) > 0.1f64 && climate.get_value(x, y) > 0.1f64 {
            return Terrain::Jungle;
        }
        // Just a default. Nothing *should* get through to here...
        else {
            return Terrain::Grass;
        }
    }
}

fn new_perlin_noise(scale: f64, seed: u32, width: u32, height: u32) -> NoiseMap {
    let fbm = Fbm::<Perlin>::new(seed);
    let noisemap = PlaneMapBuilder::<_, 2>::new(&fbm)
        .set_size(width as usize, height as usize)
        .set_x_bounds(-scale, scale)
        .set_y_bounds(-scale, scale)
        .build();
    return noisemap;
}
