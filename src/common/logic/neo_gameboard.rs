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
}

#[derive(Component, Default, FromReflect, Reflect)]
pub struct Tile {
    contents: Terrain,
    feature: Option<TileFeature>,
    visible_for: Vec<TeamColour>,
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
        };

        let mut rand = rand::thread_rng();
        let seed = rand.gen_range(0..u128::MAX);

        let scale =
            u32::max(gameboard_config.width, gameboard_config.height) as f64 * (1f64 / 32f64);

        let heightmap_seed = (seed >> 96) as u32;
        let inlandness_seed = ((seed >> 64) & 0xFFFF_FFFF) as u32;
        let climate_seed = ((seed >> 32) & 0xFFFF_FFFF) as u32;
        let rainfall_seed = (seed & 0xFFFF_FFFF) as u32;

        info!("SEED INFO: Seed {:?} | Heightmap seed {:?} | Inlandness seed {:?} | Climate seed {:?} | Rainfall seed {:?}", seed, heightmap_seed, inlandness_seed, climate_seed, rainfall_seed);

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
