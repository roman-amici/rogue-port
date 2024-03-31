use std::collections::HashSet;

use rand::{Rng, RngCore};
use sdl2::rect::Point;

use crate::{utilities::point_utils::cartesian_distance, Map, MapArchitect, MapBuilder, TileType};

pub struct CellularAutomataArchitect {}

impl CellularAutomataArchitect {
    fn fill_noise(builder: &mut MapBuilder, rng: &mut dyn RngCore) {
        for room in builder.map.tiles.iter_mut() {
            match rng.gen_range(0..100) {
                0..=55 => *room = TileType::Wall,
                _ => *room = TileType::Floor,
            }
        }
    }

    fn count_neighbors(map: &Map, center: Point, tile: TileType) -> usize {
        let mut count = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if ix == 0 && iy == 0 {
                    continue;
                }

                if let Some(idx) = map.try_idx(Point::new(center.x + ix, center.y + iy)) {
                    if map.tiles[idx] == tile {
                        count += 1
                    }
                }
            }
        }

        count
    }

    fn iterate(new_tiles: &mut [TileType], map: &Map) {
        for y in 1..(map.height_tiles - 1) {
            for x in 1..(map.width_tiles - 1) {
                let neighbors =
                    Self::count_neighbors(map, Point::new(x as i32, y as i32), TileType::Wall);
                let index = map.map_index(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[index] = TileType::Wall;
                } else {
                    new_tiles[index] = TileType::Floor;
                }
            }
        }
    }

    fn find_start(map: &Map) -> Point {
        let center = Point::new(map.width_tiles as i32 / 2, map.height_tiles as i32 / 2);

        if map.tiles[map.map_index(center.x as usize, center.y as usize)] == TileType::Floor {
            return center;
        }

        let mut min_distance = f32::INFINITY;
        let mut min_point = center;
        for y in 1..(map.height_tiles - 1) {
            for x in 1..(map.width_tiles - 1) {
                let index = map.map_index(x, y);
                if map.tiles[index] == TileType::Wall {
                    continue;
                }

                let point = Point::new(x as i32, y as i32);
                let distance = cartesian_distance(center, point);

                if distance < min_distance {
                    min_distance = distance;
                    min_point = point;
                }
            }
        }

        min_point
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, width: usize, height: usize, rng: &mut dyn RngCore) -> MapBuilder {
        let mut builder = MapBuilder::empty(width, height);

        Self::fill_noise(&mut builder, rng);
        builder.fill_boundary(TileType::Wall);

        let mut new_tiles =
            vec![TileType::Floor; builder.map.width_tiles * builder.map.height_tiles];
        for _ in 0..10 {
            Self::iterate(&mut new_tiles, &builder.map);
            std::mem::swap(&mut new_tiles, &mut builder.map.tiles);
        }

        builder.player_start = Self::find_start(&builder.map);
        builder.monster_spawn = builder.random_monster_spawns(45, builder.player_start, rng);

        builder.dijkstra_map.fill_all(
            (
                builder.player_start.x as usize,
                builder.player_start.y as usize,
            ),
            &builder.map,
        );

        builder.amulet_start = builder.dijkstra_map.max_distance_tile();

        builder
    }
}
