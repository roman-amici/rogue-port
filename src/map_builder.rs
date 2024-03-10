use std::cmp::{max, min};

use rand::{Rng, RngCore};
use sdl2::rect::{Point, Rect};

use crate::prelude::*;

pub struct MapBuilder {
    pub num_rooms: usize,
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(
        width_tiles: usize,
        height_tiles: usize,
        tile_map: Vec<usize>,
        rng: &mut dyn RngCore,
    ) -> Self {
        let mut builder = MapBuilder {
            map: Map::new(width_tiles, height_tiles, tile_map),
            rooms: vec![],
            player_start: Point::new(0, 0),
            num_rooms: 10,
        };

        builder.build_random_rooms(rng);
        builder.build_corridors(rng);
        builder.player_start = builder.rooms[0].center();

        builder
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut dyn RngCore) {
        let mut attempts = 0;
        while attempts < 50 && self.rooms.len() < self.num_rooms {
            let room = Rect::new(
                rng.gen_range(1..self.map.width_tiles - 10) as i32,
                rng.gen_range(1..self.map.height_tiles - 10) as i32,
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
                        let index = self.map.map_index(x as usize, y as usize);
                        self.map.tiles[index] = TileType::Floor;
                    }
                }

                self.rooms.push(room);
            } else {
                attempts += 1;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(index) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[index] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(index) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[index] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut dyn RngCore) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x().cmp(&b.center().x()));

        // skip the first room
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev_center = rooms[i - 1].center();
            let new_center = room.center();

            if rng.gen_bool(0.5) {
                self.apply_horizontal_tunnel(prev_center.x(), new_center.x(), prev_center.y());
                self.apply_vertical_tunnel(prev_center.y(), new_center.y(), new_center.x());
            } else {
                self.apply_vertical_tunnel(prev_center.y(), new_center.y(), prev_center.x());
                self.apply_horizontal_tunnel(prev_center.x(), new_center.x(), new_center.y());
            }
        }
    }
}
