use bevy_ecs::system::Resource;
use sdl2::rect::Point;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Resource)]
pub struct Map {
    pub width_tiles: usize,
    pub height_tiles: usize,

    pub tiles: Vec<TileType>,
    pub revealed_tiles : Vec<bool>,
}

impl Map {
    pub fn new(width_tiles: usize, height_tiles: usize) -> Self {
        Self {
            width_tiles,
            height_tiles,
            tiles: vec![TileType::Wall; width_tiles * height_tiles],
            revealed_tiles : vec![false; width_tiles * height_tiles],
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

    pub fn can_enter(&self, point: Point) -> bool {
        self.in_bounds(point)
            && self.tiles[self.map_index(point.x as usize, point.y as usize)] == TileType::Floor
    }

    pub fn is_opaque(&self, idx : usize) -> bool {
        self.tiles[idx] != TileType::Floor
    }
}
