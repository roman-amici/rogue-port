use std::collections::HashSet;

use rand::{Rng, RngCore};
use sdl2::rect::Point;

use crate::{resources::*, utilities::dijkstra_map::DijkstraMap};

pub struct MapBuilder {
    pub map: Map,
    pub spawn_points: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub dijkstra_map: DijkstraMap,
}

impl MapBuilder {
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            map: Map::new(width, height),
            spawn_points: vec![],
            player_start: Point::new(0, 0),
            amulet_start: Point::new(0, 0),
            dijkstra_map: DijkstraMap::new(width, height),
        }
    }

    pub fn find_most_distant(&mut self) -> Point {
        self.dijkstra_map.fill_all(
            (self.player_start.x as usize, self.player_start.y as usize),
            &self.map,
        );

        self.dijkstra_map.max_distance_tile()
    }

    pub fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    pub fn fill_boundary(&mut self, tile: TileType) {
        for x in 0..self.map.width_tiles {
            let index = self.map.map_index(x, 0);
            self.map.tiles[index] = tile;
            let index = self.map.map_index(x, self.map.height_tiles - 1);
            self.map.tiles[index] = tile;
        }

        for y in 0..self.map.height_tiles {
            let index = self.map.map_index(0, y);
            self.map.tiles[index] = tile;

            let index = self.map.map_index(self.map.width_tiles - 1, y);
            self.map.tiles[index] = tile;
        }
    }

    pub fn random_monster_spawns(
        &self,
        num_monsters: usize,
        start: Point,
        rng: &mut dyn RngCore,
    ) -> Vec<Point> {
        let mut monsters = HashSet::new();

        let spawnable = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter_map(|(index, tile)| {
                let (x, y) = self.map.map_index_to_coords(index);
                let point = Point::new(x as i32, y as i32);
                if *tile == TileType::Wall || point == start {
                    None
                } else {
                    Some(point)
                }
            })
            .collect::<Vec<Point>>();

        for _ in 0..num_monsters {
            let index = rng.gen_range(0..spawnable.len());

            let point = spawnable[index];
            monsters.insert(point);
        }

        monsters.into_iter().collect()
    }
}
