use rand::{Rng, RngCore};
use sdl2::rect::Point;

use crate::{Map, MapArchitect, MapBuilder, TileType};

pub struct RandomWalkArchitect {}

impl RandomWalkArchitect {
    fn random_walk(start: Point, rng: &mut dyn RngCore, map: &mut Map) {
        let mut pos = start;
        let mut iterations = 0;
        loop {
            let index = map.map_index(pos.x as usize, pos.y as usize);
            map.tiles[index] = TileType::Floor;
            match rng.gen_range(0..4) {
                0 => pos.x -= 1,
                1 => pos.x += 1,
                2 => pos.y -= 1,
                _ => pos.y += 1,
            }

            if !map.in_bounds(pos) {
                break;
            }

            iterations += 1;
            if iterations > 1000 {
                break;
            }
        }
    }
}

impl MapArchitect for RandomWalkArchitect {
    fn new(&mut self, width: usize, height: usize, rng: &mut dyn RngCore) -> MapBuilder {
        let mut builder = MapBuilder::empty(width, height);

        builder.fill(TileType::Wall);

        let center = builder.map.center();
        Self::random_walk(center, rng, &mut builder.map);

        builder.player_start = center;
        builder.dijkstra_map.fill_all(
            (
                builder.player_start.x as usize,
                builder.player_start.y as usize,
            ),
            &builder.map,
        );

        builder.amulet_start = builder.dijkstra_map.max_distance_tile();

        builder.monster_spawn = builder.random_monster_spawns(25, builder.player_start, rng);

        builder
    }
}
