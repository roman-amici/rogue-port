use sdl2::rect::Point;

use crate::tile_render::TileRender;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub width_tiles: usize,
    pub height_tiles: usize,

    pub tiles: Vec<TileType>,
    pub tile_map: Vec<usize>,
}

impl Map {
    pub fn new(width_tiles: usize, height_tiles: usize, tile_map: Vec<usize>) -> Self {
        Self {
            width_tiles,
            height_tiles,
            tiles: vec![TileType::Wall; width_tiles * height_tiles],
            tile_map,
        }
    }

    pub fn map_index(&self, x: usize, y: usize) -> usize {
        self.width_tiles * y + x
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(self.map_index(point.x as usize, point.y as usize))
        } else {
            None
        }
    }

    pub fn map_index_to_coords(&self, index: usize) -> (usize, usize) {
        let x = index % self.width_tiles;
        let y = index / self.width_tiles;

        (x, y)
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0
            && point.x < self.width_tiles as i32
            && point.y >= 0
            && point.y < self.height_tiles as i32
    }

    pub fn player_can_enter(&self, point: Point) -> bool {
        self.in_bounds(point)
            && self.tiles[self.map_index(point.x as usize, point.y as usize)] == TileType::Floor
    }

    pub fn render(&self, render: &mut TileRender) {
        for y in 0..self.height_tiles {
            for x in 0..self.width_tiles {
                let index = self.map_index(x, y);
                match self.tiles[index] {
                    TileType::Floor => {
                        render.draw_tile_grid(x, y, self.tile_map[TileType::Floor as usize])
                    }
                    TileType::Wall => {
                        render.draw_tile_grid(x, y, self.tile_map[TileType::Wall as usize])
                    }
                }
            }
        }
    }
}
