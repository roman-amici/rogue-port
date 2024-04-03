use std::cmp::{max, min};

use rand::{Rng, RngCore};
use sdl2::rect::{Point, Rect};

use crate::{utilities::dijkstra_map::DijkstraMap, Map, MapArchitect, MapBuilder, TileType};

pub struct RoomsArchitect {
    pub rooms: Vec<Rect>,
}

impl RoomsArchitect {
    pub fn new() -> Self {
        Self { rooms: vec![] }
    }

    fn build_random_rooms(
        &mut self,
        builder: &mut MapBuilder,
        num_rooms: usize,
        rng: &mut dyn RngCore,
    ) {
        let mut attempts = 0;
        while attempts < 50 && self.rooms.len() < num_rooms {
            let room = Rect::new(
                rng.gen_range(1..builder.map.width_tiles - 10) as i32,
                rng.gen_range(1..builder.map.height_tiles - 10) as i32,
                rng.gen_range(2..10) as u32,
                rng.gen_range(2..10) as u32,
            );

            let overlap = self.rooms.iter().any(|r| r.has_intersection(room));
            if !overlap {
                attempts = 0;
                let x_start = room.x() as u32;
                let x_end = room.x() as u32 + room.width();

                let y_start = room.y() as u32;
                let y_end = room.y() as u32 + room.height();
                for x in x_start..=x_end {
                    for y in y_start..=y_end {
                        let index = builder.map.map_index(x as usize, y as usize);
                        builder.map.tiles[index] = TileType::Floor;
                    }
                }

                self.rooms.push(room);
            } else {
                attempts += 1;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, builder: &mut MapBuilder, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(index) = builder.map.try_idx(Point::new(x, y)) {
                builder.map.tiles[index] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, builder: &mut MapBuilder, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(index) = builder.map.try_idx(Point::new(x, y)) {
                builder.map.tiles[index] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, builder: &mut MapBuilder, rng: &mut dyn RngCore) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x().cmp(&b.center().x()));

        // skip the first room
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev_center = rooms[i - 1].center();
            let new_center = room.center();

            if rng.gen_bool(0.5) {
                self.apply_horizontal_tunnel(
                    builder,
                    prev_center.x(),
                    new_center.x(),
                    prev_center.y(),
                );
                self.apply_vertical_tunnel(
                    builder,
                    prev_center.y(),
                    new_center.y(),
                    new_center.x(),
                );
            } else {
                self.apply_vertical_tunnel(
                    builder,
                    prev_center.y(),
                    new_center.y(),
                    prev_center.x(),
                );
                self.apply_horizontal_tunnel(
                    builder,
                    prev_center.x(),
                    new_center.x(),
                    new_center.y(),
                );
            }
        }
    }
}

impl MapArchitect for RoomsArchitect {
    fn new(
        &mut self,
        width: usize,
        height: usize,
        rng: &mut dyn rand::prelude::RngCore,
    ) -> crate::MapBuilder {
        self.rooms.clear();

        let mut builder = MapBuilder {
            map: Map::new(width, height),
            player_start: Point::new(0, 0),
            spawn_points: vec![],
            amulet_start: Point::new(0, 0),
            dijkstra_map: DijkstraMap::new(width, height),
        };

        let num_rooms = rng.gen_range(10..20);

        self.build_random_rooms(&mut builder, num_rooms, rng);
        self.build_corridors(&mut builder, rng);
        builder.player_start = self.rooms[0].center();

        builder.dijkstra_map.fill_all(
            (
                builder.player_start.x as usize,
                builder.player_start.y as usize,
            ),
            &builder.map,
        );

        builder.amulet_start = builder.dijkstra_map.max_distance_tile();

        builder.spawn_points = self
            .rooms
            .iter()
            .skip(1)
            .map(|room| room.center())
            .collect();

        builder
    }
}
