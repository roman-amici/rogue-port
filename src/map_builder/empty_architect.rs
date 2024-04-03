use rand::Rng;
use sdl2::rect::Point;

use crate::{utilities::dijkstra_map::DijkstraMap, Map, MapArchitect, MapBuilder, TileType};

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(
        &mut self,
        width: usize,
        height: usize,
        rng: &mut dyn rand::prelude::RngCore,
    ) -> crate::MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(width, height),
            player_start: Point::new(0, 0),
            amulet_start: Point::new(0, 0),
            spawn_points: vec![],
            dijkstra_map: DijkstraMap::new(width, height),
        };

        mb.fill(TileType::Floor);
        mb.fill_boundary(TileType::Wall);
        mb.player_start = Point::new((width / 2) as i32, (height / 2) as i32);
        mb.amulet_start = mb.find_most_distant();

        for _ in 0..50 {
            mb.spawn_points.push(Point::new(
                rng.gen_range(1..width) as i32,
                rng.gen_range(1..height) as i32,
            ));
        }

        mb
    }
}
