use bevy_ecs::system::Resource;
use sdl2::{render::Canvas, video::Window};

use crate::resources::map::TileType;

#[derive(Resource)]
pub struct TileMapLayer {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>,
}

impl TileMapLayer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![TileType::Wall; width * height],
        }
    }

    pub fn set(&mut self, col: usize, row: usize, tile: TileType) {
        let index = row * self.width + col;

        self.tiles[index] = tile;
    }

    pub fn get_index(&self, col: usize, row: usize) -> usize {
        row * self.width + col
    }
}
